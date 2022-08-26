use anchor_lang::prelude::*;

use crate::{errors, state::EventPass, state::EventPassHolder};

#[event]
pub struct EventPassHolderCreated {
    event_pass: Pubkey,
    event_pass_holder: Pubkey,
}

#[derive(Accounts)]
pub struct EventPassHolderCreate<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(mut)]
    pub event_pass: Box<Account<'info, EventPass>>,

    #[account(
        init,
        seeds = [
            b"EventPassHolder", event_pass.key().as_ref(), &event_pass.counter_holders.to_le_bytes()
        ],
        bump,
        payer = payer,
        space = EventPassHolder::LEN
    )]
    pub event_pass_holder: Box<Account<'info, EventPassHolder>>,

    pub event_pass_owner: SystemAccount<'info>,

    pub rent: Sysvar<'info, Rent>,

    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<EventPassHolderCreate>) -> Result<()> {
    // Update the event pass's counter for number of holders
    let event_pass = &mut ctx.accounts.event_pass;
    event_pass.counter_holders = event_pass
        .counter_holders
        .checked_add(1)
        .ok_or(errors::ErrorCode::OverflowError)?;

    // Store the event pass owner's key
    let event_pass_holder = &mut ctx.accounts.event_pass_holder;
    event_pass_holder.owner = ctx.accounts.event_pass_owner.key();

    emit!(EventPassHolderCreated {
        event_pass: event_pass.key(),
        event_pass_holder: event_pass_holder.key()
    });

    Ok(())
}

pub fn access_control(ctx: &Context<EventPassHolderCreate>) -> Result<()> {
    // Payer has to be one of those
    if ctx.accounts.payer.key() != ctx.accounts.event_pass.pass_authority_issuer
        && ctx.accounts.payer.key() != ctx.accounts.event_pass.admin
    {
        return Err(errors::ErrorCode::NotAuthorized.into());
    }

    // Make sure the number of pass holders doesn't exceed limit
    if ctx.accounts.event_pass.counter_holders >= ctx.accounts.event_pass.limit_holders {
        return Err(errors::ErrorCode::LimitOfEventPassHoldersReached.into());
    }

    Ok(())
}
