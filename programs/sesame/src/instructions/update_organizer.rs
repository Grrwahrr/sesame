use anchor_lang::prelude::*;

use crate::state::Organizer;

#[derive(Accounts)]
#[instruction(
    title: String,
    website: String,
)]
pub struct UpdateOrganizer<'info> {
    pub authority: Signer<'info>,

    #[account(
        mut,
        seeds = [b"Organizer", authority.key().as_ref()],
        bump
    )]
    pub organizer: Box<Account<'info, Organizer>>,
}

pub fn handler(ctx: Context<UpdateOrganizer>, title: String, website: String) -> Result<()> {
    // Update data
    let organizer = &mut ctx.accounts.organizer;
    organizer.title = title;
    organizer.website = website;

    Ok(())
}
