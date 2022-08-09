use anchor_lang::prelude::*;

use crate::state::Organizer;

#[event]
pub struct OrganizerCreated {
    organizer: Pubkey,
}

#[derive(Accounts)]
#[instruction(
    title: String,
    website: String,
)]
pub struct CreateOrganizer<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(
        init,
        seeds = [b"Organizer", payer.key().as_ref()],
        bump,
        payer = payer,
        space = Organizer::LEN
    )]
    pub organizer: Box<Account<'info, Organizer>>,

    pub rent: Sysvar<'info, Rent>,

    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<CreateOrganizer>, title: String, website: String) -> Result<()> {
    // Store data
    let organizer = &mut ctx.accounts.organizer;
    organizer.title = title;
    organizer.website = website;

    emit!(OrganizerCreated {
        organizer: organizer.key(),
    });

    Ok(())
}
