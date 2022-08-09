use anchor_lang::prelude::*;

use crate::{errors, state::Event, state::Ticket, state::TicketState};

#[event]
pub struct TicketCheckedIn {
    event: Pubkey,
    ticket: Pubkey,
    seat_id: String,
}

#[derive(Accounts)]
#[instruction(
    seat_id: String
)]
pub struct TicketCheckIn<'info> {
    #[account(address = event.ticket_authority_check_in @ errors::ErrorCode::NotAuthorized)]
    pub authority: Signer<'info>,

    #[account(address = ticket.owner @ errors::ErrorCode::NotAuthorized)]
    pub ticket_owner: Signer<'info>,

    pub event: Box<Account<'info, Event>>,

    #[account(
        mut,
        seeds = [b"Ticket", event.key().as_ref(), seat_id.as_bytes()],
        bump,
    )]
    pub ticket: Box<Account<'info, Ticket>>,
}

pub fn handler(ctx: Context<TicketCheckIn>, seat_id: String) -> Result<()> {
    // Update ticket data
    let ticket = &mut ctx.accounts.ticket;
    ticket.state = TicketState::CheckedIn;

    emit!(TicketCheckedIn {
        event: ctx.accounts.event.key(),
        ticket: ticket.key(),
        seat_id
    });

    Ok(())
}

pub fn access_control(ctx: &Context<TicketCheckIn>) -> Result<()> {
    // Verify that the ticket is still in its initial state
    if ctx.accounts.ticket.state != TicketState::Initial {
        return Err(errors::ErrorCode::TicketAlreadyCheckedIn.into());
    }

    Ok(())
}
