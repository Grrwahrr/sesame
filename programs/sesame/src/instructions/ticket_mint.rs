use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token, TokenAccount};

use crate::{errors, state::Event, state::Ticket, state::TicketState};

#[event]
pub struct TicketNFTMinted {
    event: Pubkey,
    ticket: Pubkey,
    seat_id: u16,
    nft_account: Pubkey,
}

#[derive(Accounts)]
#[instruction(
    seat_id: u16
)]
pub struct TicketMint<'info> {
    /// The account that will receive the NFT
    #[account(mut)]
    pub nft_owner: Signer<'info>,

    #[account(address = ticket.owner @ errors::ErrorCode::NotAuthorized)]
    pub ticket_owner: Signer<'info>,

    pub event: Box<Account<'info, Event>>,

    #[account(
        mut,
        seeds = [b"Ticket", event.key().as_ref(), &seat_id.to_le_bytes()],
        bump,
    )]
    pub ticket: Box<Account<'info, Ticket>>,

    #[account(init,
        seeds = [ b"NFTMint", event.key().as_ref(), ticket_owner.key().as_ref()],
        bump,
        payer = nft_owner,
        mint::decimals = 0,
        mint::authority = nft_owner
    )]
    pub nft_mint: Account<'info, Mint>,

    #[account(init,
        seeds = [nft_mint.key().as_ref(), nft_owner.key().as_ref()],
        bump,
        payer = nft_owner,
        token::mint = nft_mint,
        token::authority = nft_owner,
    )]
    pub nft_account: Box<Account<'info, TokenAccount>>,

    pub token_program: Program<'info, Token>,

    pub rent: Sysvar<'info, Rent>,

    pub system_program: Program<'info, System>,
}

/// Mint a ticket as an NFT with Metaplex data.
/// The ticket account is not deleted, so that the rent can be reclaimed later.
pub fn handler(ctx: Context<TicketMint>, seat_id: u16) -> Result<()> {
    // Update ticket data
    let ticket = &mut ctx.accounts.ticket;
    ticket.state = TicketState::Minted;

    // Create an NFT spl_token mint decimals = 0
    // Mint one token to nft_owner
    // Create Metaplex NFT meta data for mint account
    // Unset mint authority

    // https://github.com/anoushk1234/metaplex-anchor-nft/blob/master/programs/metaplex-anchor-nft/src/lib.rs
    // https://spl.solana.com/token#non-fungible-tokens
    // https://docs.metaplex.com/programs/token-metadata/

    emit!(TicketNFTMinted {
        event: ctx.accounts.event.key(),
        ticket: ctx.accounts.ticket.key(),
        seat_id,
        nft_account: ctx.accounts.nft_account.key()
    });

    todo!();

    Ok(())
}

pub fn access_control(ctx: &Context<TicketMint>) -> Result<()> {
    // Verify that the ticket had been checked-in at the event
    if ctx.accounts.ticket.state != TicketState::CheckedIn {
        return Err(errors::ErrorCode::TicketWasNotCheckedIn.into());
    }

    Ok(())
}
