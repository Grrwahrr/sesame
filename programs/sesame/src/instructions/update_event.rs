use anchor_lang::prelude::*;

use crate::{errors, state::Event};

#[derive(Accounts)]
#[instruction(
    event_num: u32,
    title: String,
    website: String,
    tickets_limit: u16,
    timestamp: u64,
    location_type: u8,
    location: String,
    image_url: String,
)]
pub struct UpdateEvent<'info> {
    pub authority: Signer<'info>,

    #[account(
        mut,
        seeds = [b"Event", authority.key().as_ref(), &event_num.to_le_bytes()],
        bump
    )]
    pub event: Box<Account<'info, Event>>,

    /// CHECK: This can be any account. It will authorize ticket creation.
    pub ticket_authority_issuer: UncheckedAccount<'info>,

    /// CHECK: This can be any account. It will authorize ticket deletion.
    pub ticket_authority_delete: UncheckedAccount<'info>,

    /// CHECK: This can be any account. It will authorize check in operations.
    pub ticket_authority_check_in: UncheckedAccount<'info>,
}

pub fn handler(
    ctx: Context<UpdateEvent>,
    title: String,
    website: String,
    tickets_limit: u16,
    timestamp: u64,
    location_type: u8,
    location: String,
    image_url: String,
) -> Result<()> {
    // Update data
    let event = &mut ctx.accounts.event;
    event.ticket_authority_issuer = ctx.accounts.ticket_authority_issuer.key();
    event.ticket_authority_delete = ctx.accounts.ticket_authority_delete.key();
    event.ticket_authority_check_in = ctx.accounts.ticket_authority_check_in.key();
    event.tickets_limit = tickets_limit;
    event.title = title;
    event.website = website;
    event.timestamp = timestamp;
    event.location_type = location_type.into();
    event.location = location;
    event.artwork = image_url;

    Ok(())
}

pub fn access_control(ctx: &Context<UpdateEvent>, tickets_limit: u16) -> Result<()> {
    // Make sure the set authorities do not match the event creator
    if ctx.accounts.ticket_authority_issuer.key() == ctx.accounts.authority.key() {
        return Err(errors::ErrorCode::InvalidTicketAuthorityIssuer.into());
    }

    if ctx.accounts.ticket_authority_delete.key() == ctx.accounts.authority.key() {
        return Err(errors::ErrorCode::InvalidTicketAuthorityDelete.into());
    }

    if ctx.accounts.ticket_authority_check_in.key() == ctx.accounts.authority.key() {
        return Err(errors::ErrorCode::InvalidTicketAuthorityCheckIn.into());
    }

    // Verify the ticket limit cannot be reduced to less than the number of tickets already issued
    if ctx.accounts.event.tickets_limit < tickets_limit {
        return Err(errors::ErrorCode::MoreTicketsIssued.into());
    }

    Ok(())
}
