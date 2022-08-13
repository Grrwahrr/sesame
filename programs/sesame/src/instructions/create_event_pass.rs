use anchor_lang::prelude::*;

use crate::{errors, state::EventPass, state::Organizer};

#[event]
pub struct EventPassCreated {
    event_pass: Pubkey,
}

#[derive(Accounts)]
#[instruction(
    title: String,
    website: String,
    tickets_limit: u16,
    image_url: String,
)]
pub struct CreateEventPass<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(
        mut,
        seeds = [b"Organizer", payer.key().as_ref()],
        bump,
    )]
    pub organizer: Box<Account<'info, Organizer>>,

    /// CHECK: This can be any account. It will authorize pass holder creation.
    pub pass_authority_issuer: UncheckedAccount<'info>,

    /// CHECK: This can be any account. It will authorize pass holder deletion.
    pub pass_authority_delete: UncheckedAccount<'info>,

    #[account(
        init,
        seeds = [
            b"EventPass", payer.key().as_ref(), &organizer.counter_passes.to_le_bytes()
        ],
        bump,
        payer = payer,
        space = EventPass::LEN
    )]
    pub event_pass: Box<Account<'info, EventPass>>,

    pub rent: Sysvar<'info, Rent>,

    pub system_program: Program<'info, System>,
}

pub fn handler(
    ctx: Context<CreateEventPass>,
    title: String,
    website: String,
    tickets_limit: u16,
    image_url: String,
) -> Result<()> {
    // Update organizer
    let organizer = &mut ctx.accounts.organizer;
    organizer.counter_passes = organizer
        .counter_passes
        .checked_add(1)
        .ok_or(errors::ErrorCode::OverflowError)?;

    // Store data
    let event_pass = &mut ctx.accounts.event_pass;
    event_pass.version = 1;
    event_pass.admin = ctx.accounts.payer.key();
    event_pass.pass_authority_issuer = ctx.accounts.pass_authority_issuer.key();
    event_pass.pass_authority_delete = ctx.accounts.pass_authority_delete.key();
    event_pass.tickets_limit = tickets_limit;
    event_pass.title = title;
    event_pass.website = website;
    event_pass.artwork = image_url;

    emit!(EventPassCreated {
        event_pass: event_pass.key()
    });

    Ok(())
}

pub fn access_control(ctx: &Context<CreateEventPass>) -> Result<()> {
    // Make sure the set authorities do not match the event pass creator
    if ctx.accounts.pass_authority_issuer.key() == ctx.accounts.payer.key() {
        return Err(errors::ErrorCode::InvalidPassAuthorityIssuer.into());
    }

    if ctx.accounts.pass_authority_delete.key() == ctx.accounts.payer.key() {
        return Err(errors::ErrorCode::InvalidPassAuthorityDelete.into());
    }

    Ok(())
}
