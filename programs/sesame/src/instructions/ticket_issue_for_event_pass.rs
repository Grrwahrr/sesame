use anchor_lang::prelude::*;

use crate::{
    errors, state::Event, state::EventPass, state::EventPassHolder, state::EventPassHolderTicket,
    state::EventPassValidEvent, state::Ticket,
};

#[event]
pub struct TicketIssuedForEventPass {
    event_pass: Pubkey,
    event_pass_holder: Pubkey,
    event: Pubkey,
    ticket: Pubkey,
}

#[derive(Accounts)]
#[instruction(
    event_offset: u16,
    holder_offset: u16
)]
pub struct TicketIssueForEventPass<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    pub event_pass: Box<Account<'info, EventPass>>,

    #[account(
        seeds = [
            b"EventPassValidEvent", event_pass.key().as_ref(), &event_offset.to_le_bytes()
        ],
        bump
    )]
    pub event_pass_valid_event: Box<Account<'info, EventPassValidEvent>>,

    #[account(
        mut,
        seeds = [
            b"EventPassHolder", event_pass.key().as_ref(), &holder_offset.to_le_bytes()
        ],
        bump,
    )]
    pub event_pass_holder: Box<Account<'info, EventPassHolder>>,

    #[account(address = event_pass_holder.owner)]
    pub event_pass_owner: Signer<'info>,

    #[account(
        init,
        seeds = [
            b"EventPassHolderTicket", event_pass_holder.key().as_ref(), event_pass_valid_event.event.as_ref()
        ],
        bump,
        payer = payer,
        space = EventPassHolderTicket::LEN
    )]
    pub event_pass_holder_ticket: Box<Account<'info, EventPassHolderTicket>>,

    #[account(mut, address = event_pass_valid_event.event)]
    pub event: Box<Account<'info, Event>>,

    #[account(
        init,
        seeds = [
            b"Ticket", event.key().as_ref(), &event.tickets_issued.to_le_bytes()
        ],
        bump,
        payer = payer,
        space = Ticket::LEN
    )]
    pub ticket: Box<Account<'info, Ticket>>,

    pub ticket_owner: SystemAccount<'info>,

    pub rent: Sysvar<'info, Rent>,

    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<TicketIssueForEventPass>) -> Result<()> {
    // Update the events counter for issued tickets
    let event = &mut ctx.accounts.event;
    event.tickets_issued = event
        .tickets_issued
        .checked_add(1)
        .ok_or(errors::ErrorCode::OverflowError)?;

    // Store the ticket owner's key
    let ticket = &mut ctx.accounts.ticket;
    ticket.owner = ctx.accounts.ticket_owner.key();

    // Increase the number of tickets created by the event pass holder account
    let event_pass_holder = &mut ctx.accounts.event_pass_holder;
    event_pass_holder.tickets_created = event_pass_holder
        .tickets_created
        .checked_add(1)
        .ok_or(errors::ErrorCode::OverflowError)?;

    emit!(TicketIssuedForEventPass {
        event_pass: ctx.accounts.event_pass.key(),
        event_pass_holder: event_pass_holder.key(),
        event: event.key(),
        ticket: ticket.key()
    });

    Ok(())
}

pub fn access_control(ctx: &Context<TicketIssueForEventPass>) -> Result<()> {
    // Payer has to be one of those
    if ctx.accounts.payer.key() != ctx.accounts.event.ticket_authority_issuer
        && ctx.accounts.payer.key() != ctx.accounts.event.admin
    {
        return Err(errors::ErrorCode::NotAuthorized.into());
    }

    // Make sure the events ticket limit will not be exceeded (must be one less than MAX)
    if ctx.accounts.event.tickets_issued >= ctx.accounts.event.tickets_limit {
        return Err(errors::ErrorCode::NoMoreTicketsLeft.into());
    }

    // Make sure the pass holder can still create tickets
    if ctx.accounts.event_pass_holder.tickets_created >= ctx.accounts.event_pass.limit_tickets {
        return Err(errors::ErrorCode::NoMoreTicketsLeftInEventPass.into());
    }

    Ok(())
}
