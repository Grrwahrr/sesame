use anchor_lang::prelude::*;

use crate::{errors, state::Event, state::Ticket, state::TicketState};

#[event]
pub struct TicketDeleted {
    event: Pubkey,
    ticket: Pubkey,
    seat_id: String,
}

#[derive(Accounts)]
#[instruction(
    seat_id: String
)]
pub struct TicketDelete<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(address = ticket.owner @ errors::ErrorCode::NotAuthorized)]
    pub ticket_owner: Signer<'info>,

    pub event: Box<Account<'info, Event>>,

    #[account(
        mut,
        seeds = [b"Ticket", event.key().as_ref(), seat_id.as_bytes()],
        bump,
        close = authority
    )]
    pub ticket: Box<Account<'info, Ticket>>,

    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<TicketDelete>, seat_id: String) -> Result<()> {
    // The ticket account is closed using the close macro. The rent is refunded to the authority.

    emit!(TicketDeleted {
        event: ctx.accounts.event.key(),
        ticket: ctx.accounts.ticket.key(),
        seat_id
    });

    Ok(())
}

pub fn access_control(ctx: &Context<TicketDelete>) -> Result<()> {
    // Authority has to be one of those
    if ctx.accounts.authority.key() != ctx.accounts.event.ticket_authority_delete
        && ctx.accounts.authority.key() != ctx.accounts.event.admin
    {
        return Err(errors::ErrorCode::NotAuthorized.into());
    }

    // Verify the ticket is still in its initial state
    if ctx.accounts.ticket.state != TicketState::Initial {
        return Err(errors::ErrorCode::TicketAlreadyCheckedIn.into());
    }

    Ok(())
}
