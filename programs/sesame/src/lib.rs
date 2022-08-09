use anchor_lang::prelude::*;

mod errors;
mod instructions;
mod state;

use crate::instructions::*;

declare_id!("2GTUkXFnABGVHFMqT1tVofBLPrBTAxzjb4Z2rpeMGsJG");

pub mod donate_address {
    use anchor_lang::declare_id;
    declare_id!("JCsJe2cWR3wp9a4kvY9JK4qTR1FiBwxXSzsHyTZuZfFA"); //TODO - what address
}

#[program]
pub mod sesame {
    use super::*;

    /// Create a new event organizer
    pub fn create_organizer(
        ctx: Context<CreateOrganizer>,
        title: String,
        website: String,
    ) -> Result<()> {
        msg!("Instruction: CreateOrganizer");
        instructions::create_organizer::handler(ctx, title, website)
    }

    /// Create a new event, requires an organizer account
    #[access_control(
        instructions::create_event::access_control(&ctx)
    )]
    pub fn create_event(
        ctx: Context<CreateEvent>,
        title: String,
        website: String,
        tickets_limit: u16,
    ) -> Result<()> {
        msg!("Instruction: CreateEvent");
        instructions::create_event::handler(ctx, title, website, tickets_limit)
    }

    /// Create a new ticket which belongs to someone
    #[access_control(
        instructions::ticket_issue::access_control(&ctx)
    )]
    pub fn ticket_issue(ctx: Context<TicketIssue>, seat_id: String) -> Result<()> {
        msg!("Instruction: TicketIssue");
        instructions::ticket_issue::handler(ctx, seat_id)
    }

    /// Delete a ticket that was refunded
    #[access_control(
        instructions::ticket_delete::access_control(&ctx)
    )]
    pub fn ticket_delete(ctx: Context<TicketDelete>, seat_id: String) -> Result<()> {
        msg!("Instruction: TicketDelete");
        instructions::ticket_delete::handler(ctx, seat_id)
    }

    /// Update the ticket, set as checked in
    #[access_control(
        instructions::ticket_check_in::access_control(&ctx)
    )]
    pub fn ticket_check_in(ctx: Context<TicketCheckIn>, seat_id: String) -> Result<()> {
        msg!("Instruction: TicketCheckIn");
        instructions::ticket_check_in::handler(ctx, seat_id)
    }

    /// Create an NFT POAP from a ticket
    #[access_control(
        instructions::ticket_mint::access_control(&ctx)
    )]
    pub fn ticket_mint(ctx: Context<TicketMint>, seat_id: String) -> Result<()> {
        msg!("Instruction: TicketMint");
        instructions::ticket_mint::handler(ctx, seat_id)
    }

    /// Update an organizers data
    pub fn update_organizer(
        ctx: Context<UpdateOrganizer>,
        title: String,
        website: String,
    ) -> Result<()> {
        msg!("Instruction: UpdateOrganizer");
        instructions::update_organizer::handler(ctx, title, website)
    }

    /// Update an events data
    #[access_control(
        instructions::update_event::access_control(&ctx, tickets_limit)
    )]
    pub fn update_event(
        ctx: Context<UpdateEvent>,
        event_num: u32,
        title: String,
        website: String,
        tickets_limit: u16,
    ) -> Result<()> {
        msg!("Instruction: UpdateEvent");
        instructions::update_event::handler(ctx, event_num, title, website, tickets_limit)
    }

    // Delete an event, including derived accounts
    //TODO: I need a function to delete an event?
    //      As for the ticket accounts, they need to be deleted using JS
}

// TODO - verify rent exemption for organizer, event stay true when updating with longer strings

// TODO --
//  when minting NFTs, donate some coins to me (as implemented in IX create_event)
//  Add a PDA config & a function to update donate address and donate amount - or just update app ??

// #[derive(Debug, AnchorSerialize, AnchorDeserialize, PartialEq)]
// #[repr(u8)]
// pub enum EventStatus {
//     Created = 0,    // Tickets can be sold
//     Started = 1,    // Delete no longer possible? Actually... ppl might still want to refund? ALSO: can just unset the keys to stop an action
//     Ended = 2,      // Minting can be done
// }
// impl From<u8> for EventStatus {
//     fn from(value: u8) -> EventStatus {
//         match value {
//             0 => EventStatus::Created,
//             1 => EventStatus::Started,
//             2 => EventStatus::Ended,
//             _ => panic!("Unknown value: {}", value),
//         }
//     }
// } TODO  - do we need anything like this?
