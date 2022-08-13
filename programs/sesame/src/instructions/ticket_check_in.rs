use anchor_lang::prelude::*;

use crate::{errors, state::Event, state::Ticket, state::TicketState};

#[event]
pub struct TicketCheckedIn {
    event: Pubkey,
    ticket: Pubkey,
    seat_id: u16,
}

#[derive(Accounts)]
#[instruction(
    seat_id: u16
)]
pub struct TicketCheckIn<'info> {
    pub authority: Signer<'info>,

    #[account(address = ticket.owner @ errors::ErrorCode::NotAuthorized)]
    pub ticket_owner: Signer<'info>,

    pub event: Box<Account<'info, Event>>,

    #[account(
        mut,
        seeds = [b"Ticket", event.key().as_ref(), &seat_id.to_le_bytes()],
        bump,
    )]
    pub ticket: Box<Account<'info, Ticket>>,
}

pub fn handler(ctx: Context<TicketCheckIn>, seat_id: u16) -> Result<()> {
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
    // Authority has to be one of those
    if ctx.accounts.authority.key() != ctx.accounts.event.ticket_authority_check_in
        && ctx.accounts.authority.key() != ctx.accounts.event.admin
    {
        return Err(errors::ErrorCode::NotAuthorized.into());
    }

    // Verify that the ticket is still in its initial state
    if ctx.accounts.ticket.state != TicketState::Initial {
        return Err(errors::ErrorCode::TicketAlreadyCheckedIn.into());
    }

    Ok(())
}
