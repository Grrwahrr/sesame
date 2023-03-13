use anchor_lang::prelude::*;
use anchor_lang::solana_program::program::invoke;
use anchor_spl::token::{self, Mint, MintTo, Token, TokenAccount};
use mpl_token_metadata::instruction::{create_master_edition_v3, create_metadata_accounts_v2};

use crate::{donate_address, errors, state::Event, state::Ticket, state::TicketState};

#[event]
pub struct TicketNFTMinted {
    event: Pubkey,
    ticket: Pubkey,
    ticket_offset: u16,
    nft_account: Pubkey,
}

#[derive(Accounts)]
#[instruction(
    ticket_offset: u16
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
        seeds = [b"Ticket", event.key().as_ref(), &ticket_offset.to_le_bytes()],
        bump,
    )]
    pub ticket: Box<Account<'info, Ticket>>,

    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account(mut)]
    pub mint: UncheckedAccount<'info>,

    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account(mut)]
    pub metadata: UncheckedAccount<'info>,

    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account(mut)]
    pub master_edition: UncheckedAccount<'info>,

    /// CHECK: This is not dangerous because we don't read or write from this account
    pub token_metadata_program: UncheckedAccount<'info>,

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
pub fn handler(ctx: Context<TicketMint>, ticket_offset: u16) -> Result<()> {
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

    let cpi_accounts = MintTo {
        mint: ctx.accounts.mint.to_account_info(),
        to: ctx.accounts.nft_owner.to_account_info(),
        authority: ctx.accounts.nft_owner.to_account_info(),
    };

    let cpi_program = ctx.accounts.token_program.to_account_info();
    let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
    token::mint_to(cpi_ctx, 1)?;
    msg!("done mint_to");

    let account_info = vec![
        ctx.accounts.metadata.to_account_info(),
        ctx.accounts.mint.to_account_info(),
        ctx.accounts.nft_owner.to_account_info(),
        ctx.accounts.nft_owner.to_account_info(),
        ctx.accounts.token_metadata_program.to_account_info(),
        ctx.accounts.token_program.to_account_info(),
        ctx.accounts.system_program.to_account_info(),
        ctx.accounts.rent.to_account_info(),
    ];
    let creator = vec![
        mpl_token_metadata::state::Creator {
            address: donate_address::id(),
            verified: false,
            share: 100,
        },
        mpl_token_metadata::state::Creator {
            address: ctx.accounts.nft_owner.key(),
            verified: false,
            share: 0,
        },
    ];
    msg!("Creator Assigned");
    let symbol = "Event POPA".to_string();
    let title = "POAP Event Title".to_string();
    let uri = "URL".to_string();
    invoke(
        &create_metadata_accounts_v2(
            ctx.accounts.token_metadata_program.key(),
            ctx.accounts.metadata.key(),
            ctx.accounts.mint.key(),
            ctx.accounts.nft_owner.key(),
            ctx.accounts.nft_owner.key(),
            ctx.accounts.nft_owner.key(),
            title,
            symbol,
            uri,
            Some(creator),
            1,
            true,
            false,
            None,
            None,
        ),
        account_info.as_slice(),
    )?;
    msg!("done create_metadata_accounts_v2");

    let master_edition_infos = vec![
        ctx.accounts.master_edition.to_account_info(),
        ctx.accounts.mint.to_account_info(),
        ctx.accounts.nft_owner.to_account_info(),
        ctx.accounts.nft_owner.to_account_info(),
        ctx.accounts.metadata.to_account_info(),
        ctx.accounts.token_metadata_program.to_account_info(),
        ctx.accounts.token_program.to_account_info(),
        ctx.accounts.system_program.to_account_info(),
        ctx.accounts.rent.to_account_info(),
    ];
    invoke(
        &create_master_edition_v3(
            ctx.accounts.token_metadata_program.key(),
            ctx.accounts.master_edition.key(),
            ctx.accounts.mint.key(),
            ctx.accounts.nft_owner.key(),
            ctx.accounts.nft_owner.key(),
            ctx.accounts.metadata.key(),
            ctx.accounts.nft_owner.key(),
            Some(0),
        ),
        master_edition_infos.as_slice(),
    )?;
    msg!("done create_master_edition_v3");

    emit!(TicketNFTMinted {
        event: ctx.accounts.event.key(),
        ticket: ctx.accounts.ticket.key(),
        ticket_offset,
        nft_account: ctx.accounts.nft_account.key()
    });

    Ok(())
}

pub fn access_control(ctx: &Context<TicketMint>) -> Result<()> {
    // Verify that the ticket had been checked-in at the event
    if ctx.accounts.ticket.state != TicketState::CheckedIn {
        return Err(errors::ErrorCode::TicketWasNotCheckedIn.into());
    }

    Ok(())
}
