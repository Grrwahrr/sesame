use anchor_lang::prelude::*;

use crate::{errors, state::EventPass};

#[derive(Accounts)]
#[instruction(
    event_pass_num: u32,
    title: String,
    website: String,
    image_url: String,
    limit_tickets: u16,
    limit_holders: u16,
)]
pub struct UpdateEventPass<'info> {
    pub authority: Signer<'info>,

    #[account(
        mut,
        seeds = [b"EventPass", authority.key().as_ref(), &event_pass_num.to_le_bytes()],
        bump
    )]
    pub event_pass: Box<Account<'info, EventPass>>,

    /// CHECK: This can be any account. It will authorize event pass holder creation.
    pub pass_authority_issuer: UncheckedAccount<'info>,

    /// CHECK: This can be any account. It will authorize event pass holder deletion.
    pub pass_authority_delete: UncheckedAccount<'info>,
}

pub fn handler(
    ctx: Context<UpdateEventPass>,
    title: String,
    website: String,
    image_url: String,
    limit_tickets: u16,
    limit_holders: u16,
) -> Result<()> {
    // Update data
    let event_pass = &mut ctx.accounts.event_pass;
    event_pass.pass_authority_issuer = ctx.accounts.pass_authority_issuer.key();
    event_pass.pass_authority_delete = ctx.accounts.pass_authority_delete.key();
    event_pass.limit_tickets = limit_tickets;
    event_pass.limit_holders = limit_holders;
    event_pass.title = title;
    event_pass.website = website;
    event_pass.artwork = image_url;

    Ok(())
}

pub fn access_control(ctx: &Context<UpdateEventPass>, limit_holders: u16) -> Result<()> {
    // Make sure the set authorities do not match the event creator
    if ctx.accounts.pass_authority_issuer.key() == ctx.accounts.authority.key() {
        return Err(errors::ErrorCode::InvalidTicketAuthorityIssuer.into());
    }

    if ctx.accounts.pass_authority_delete.key() == ctx.accounts.authority.key() {
        return Err(errors::ErrorCode::InvalidTicketAuthorityDelete.into());
    }

    // Verify the holder limit cannot be reduced to less than the number of current holders
    if ctx.accounts.event_pass.limit_holders < limit_holders {
        return Err(errors::ErrorCode::MoreHoldersExist.into());
    }

    Ok(())
}
