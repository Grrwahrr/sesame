use anchor_lang::prelude::*;

use crate::{errors, state::Event, state::Ticket};

#[event]
pub struct TicketIssued {
    event: Pubkey,
    ticket: Pubkey,
}

#[derive(Accounts)]
pub struct TicketIssue<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(mut)]
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

pub fn handler(ctx: Context<TicketIssue>) -> Result<()> {
    // Update the events counter for issued tickets
    let event = &mut ctx.accounts.event;
    event.tickets_issued = event
        .tickets_issued
        .checked_add(1)
        .ok_or(errors::ErrorCode::OverflowError)?;

    // Store the ticket owner's key
    let ticket = &mut ctx.accounts.ticket;
    ticket.owner = ctx.accounts.ticket_owner.key();

    emit!(TicketIssued {
        event: event.key(),
        ticket: ticket.key()
    });

    Ok(())
}

pub fn access_control(ctx: &Context<TicketIssue>) -> Result<()> {
    // Payer has to be one of those
    if ctx.accounts.payer.key() != ctx.accounts.event.ticket_authority_issuer
        && ctx.accounts.payer.key() != ctx.accounts.event.admin
    {
        return Err(errors::ErrorCode::NotAuthorized.into());
    }

    // Make sure the ticket limit will not be exceeded (must be one less than MAX)
    if ctx.accounts.event.tickets_issued >= ctx.accounts.event.tickets_limit {
        return Err(errors::ErrorCode::NoMoreTicketsLeft.into());
    }

    Ok(())
}
