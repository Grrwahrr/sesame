use anchor_lang::prelude::*;

use crate::state::{EventPass, EventPassValidEvent};
use crate::{errors, state::Event};

#[event]
pub struct EventPassEventAdded {
    event_pass: Pubkey,
    event: Pubkey,
}

#[derive(Accounts)]
pub struct EventPassAddEvent<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(mut, constraint = event_pass.admin == payer.key() @ errors::ErrorCode::NotAuthorized)]
    pub event_pass: Box<Account<'info, EventPass>>,

    pub event: Box<Account<'info, Event>>,

    #[account(
        init,
        seeds = [
            b"EventPassValidEvent", event_pass.key().as_ref(), &event_pass.counter_events.to_le_bytes()
        ],
        bump,
        payer = payer,
        space = EventPassValidEvent::LEN
    )]
    pub event_pass_valid_event: Box<Account<'info, EventPassValidEvent>>,

    pub rent: Sysvar<'info, Rent>,

    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<EventPassAddEvent>) -> Result<()> {
    // Update the events counter for the event pass
    let event_pass = &mut ctx.accounts.event_pass;
    event_pass.counter_events = event_pass
        .counter_events
        .checked_add(1)
        .ok_or(errors::ErrorCode::OverflowError)?;

    // Store the events key
    let event_pass_valid_event = &mut ctx.accounts.event_pass_valid_event;
    event_pass_valid_event.event = ctx.accounts.event.key();

    emit!(EventPassEventAdded {
        event_pass: event_pass.key(),
        event: ctx.accounts.event.key(),
    });

    Ok(())
}
