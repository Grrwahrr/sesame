use anchor_lang::prelude::*;

use crate::{errors, state::Event, state::Ticket};

#[event]
pub struct TicketIssued {
    event: Pubkey,
    ticket: Pubkey,
    seat_id: String,
}

#[derive(Accounts)]
#[instruction(
    seat_id: String
)]
pub struct TicketIssue<'info> {
    #[account(mut, address = event.ticket_authority_issuer @ errors::ErrorCode::NotAuthorized)]
    pub payer: Signer<'info>,

    #[account(mut)]
    pub event: Box<Account<'info, Event>>,

    #[account(
        init,
        seeds = [
            b"Ticket", event.key().as_ref(), seat_id.as_bytes()
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

pub fn handler(ctx: Context<TicketIssue>, seat_id: String) -> Result<()> {
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
        ticket: ticket.key(),
        seat_id
    });

    Ok(())
}

pub fn access_control(ctx: &Context<TicketIssue>) -> Result<()> {
    // Make sure the ticket limit will not be exceeded (must be one less than MAX)
    if ctx.accounts.event.tickets_issued >= ctx.accounts.event.tickets_limit {
        return Err(errors::ErrorCode::NoMoreTicketsLeft.into());
    }

    Ok(())
}
//TODO the event creator authority should also be able to issue tickets :) to make admin panel easier
