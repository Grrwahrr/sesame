use anchor_lang::prelude::*;

use crate::{errors, state::Event, state::Ticket};

#[event]
pub struct TicketDeleted {
    event: Pubkey,
    ticket: Pubkey,
    seat_id: u16,
}

#[derive(Accounts)]
#[instruction(
    seat_id: u16
)]
pub struct TicketDelete<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    pub ticket_owner: Signer<'info>,

    #[account(mut)]
    pub event: Box<Account<'info, Event>>,

    #[account(
        mut,
        seeds = [b"Ticket", event.key().as_ref(), &seat_id.to_le_bytes()],
        bump,
        close = authority
    )]
    pub ticket: Box<Account<'info, Ticket>>,

    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<TicketDelete>, seat_id: u16) -> Result<()> {
    // The ticket account is closed using the close macro. The rent is refunded to the authority.

    // Update some event data
    let event = &mut ctx.accounts.event;

    // Increase the count of deleted tickets
    event.tickets_deleted = event
        .tickets_deleted
        .checked_add(1)
        .ok_or(errors::ErrorCode::OverflowError)?;

    // Increase the ticket limit - this way the ticket can be re-issued
    event.tickets_limit = event
        .tickets_limit
        .checked_add(1)
        .ok_or(errors::ErrorCode::OverflowError)?;

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

    // Admin can also sign as ticket owner
    if ctx.accounts.ticket_owner.key() != ctx.accounts.ticket.owner
        && ctx.accounts.ticket_owner.key() != ctx.accounts.event.admin
    {
        return Err(errors::ErrorCode::NotAuthorized.into());
    }

    Ok(())
}
