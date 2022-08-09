use anchor_lang::prelude::*;

use crate::{donate_address, errors, state::Event, state::Organizer};

#[event]
pub struct EventCreated {
    event: Pubkey,
}

#[derive(Accounts)]
#[instruction(
    title: String,
    website: String,
    tickets_limit: u16,
    timestamp: u64,
    location_type: u8,
    location: String,
    image_url: String,
)]
pub struct CreateEvent<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    /// CHECK: If the account matches a specific key, some SOL is send as a donation
    pub donate_to: UncheckedAccount<'info>,

    #[account(
        mut,
        seeds = [b"Organizer", payer.key().as_ref()],
        bump,
    )]
    pub organizer: Box<Account<'info, Organizer>>,

    /// CHECK: This can be any account. It will authorize ticket creation.
    pub ticket_authority_issuer: UncheckedAccount<'info>,

    /// CHECK: This can be any account. It will authorize ticket deletion.
    pub ticket_authority_delete: UncheckedAccount<'info>,

    /// CHECK: This can be any account. It will authorize check in operations.
    pub ticket_authority_check_in: UncheckedAccount<'info>,

    #[account(
        init,
        seeds = [
            b"Event", payer.key().as_ref(), &organizer.counter.to_le_bytes()
        ],
        bump,
        payer = payer,
        space = Event::LEN
    )]
    pub event: Box<Account<'info, Event>>,

    pub rent: Sysvar<'info, Rent>,

    pub system_program: Program<'info, System>,
}

pub fn handler(
    ctx: Context<CreateEvent>,
    title: String,
    website: String,
    tickets_limit: u16,
    timestamp: u64,
    location_type: u8,
    location: String,
    image_url: String,
) -> Result<()> {
    // Update organizer
    let organizer = &mut ctx.accounts.organizer;
    organizer.counter = organizer
        .counter
        .checked_add(1)
        .ok_or(errors::ErrorCode::OverflowError)?;

    // Store data
    let event = &mut ctx.accounts.event;
    event.version = 1;
    event.admin = ctx.accounts.payer.key();
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

    // Optional donation
    if ctx.accounts.donate_to.key() == donate_address::id() {
        let ix = anchor_lang::solana_program::system_instruction::transfer(
            &ctx.accounts.payer.key(),
            &ctx.accounts.donate_to.key(),
            10_000_000,
        );
        anchor_lang::solana_program::program::invoke(
            &ix,
            &[
                ctx.accounts.payer.to_account_info(),
                ctx.accounts.donate_to.to_account_info(),
            ],
        )?;
    }

    emit!(EventCreated { event: event.key() });

    Ok(())
}

pub fn access_control(ctx: &Context<CreateEvent>) -> Result<()> {
    // Make sure the set authorities do not match the event creator
    if ctx.accounts.ticket_authority_issuer.key() == ctx.accounts.payer.key() {
        return Err(errors::ErrorCode::InvalidTicketAuthorityIssuer.into());
    }

    if ctx.accounts.ticket_authority_delete.key() == ctx.accounts.payer.key() {
        return Err(errors::ErrorCode::InvalidTicketAuthorityDelete.into());
    }

    if ctx.accounts.ticket_authority_check_in.key() == ctx.accounts.payer.key() {
        return Err(errors::ErrorCode::InvalidTicketAuthorityCheckIn.into());
    }

    Ok(())
}
