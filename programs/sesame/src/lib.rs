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
        create_organizer::handler(ctx, title, website)
    }

    /// Create a new event, requires an organizer account
    #[access_control(
        create_event::access_control(&ctx)
    )]
    pub fn create_event(
        ctx: Context<CreateEvent>,
        title: String,
        website: String,
        tickets_limit: u16,
        timestamp: u64,
        location_type: u8,
        location: String,
        image_url: String,
    ) -> Result<()> {
        msg!("Instruction: CreateEvent");
        create_event::handler(
            ctx,
            title,
            website,
            tickets_limit,
            timestamp,
            location_type,
            location,
            image_url,
        )
    }

    /// Create a new ticket which belongs to someone
    #[access_control(
        ticket_issue::access_control(&ctx)
    )]
    pub fn ticket_issue(ctx: Context<TicketIssue>) -> Result<()> {
        msg!("Instruction: TicketIssue");
        ticket_issue::handler(ctx)
    }

    /// Delete a ticket that was refunded
    #[access_control(
        ticket_delete::access_control(&ctx)
    )]
    pub fn ticket_delete(ctx: Context<TicketDelete>, ticket_offset: u16) -> Result<()> {
        msg!("Instruction: TicketDelete");
        ticket_delete::handler(ctx, ticket_offset)
    }

    /// Update the ticket, set as checked in
    #[access_control(
        ticket_check_in::access_control(&ctx)
    )]
    pub fn ticket_check_in(ctx: Context<TicketCheckIn>, ticket_offset: u16) -> Result<()> {
        msg!("Instruction: TicketCheckIn");
        ticket_check_in::handler(ctx, ticket_offset)
    }

    /// Create an NFT POAP from a ticket
    #[access_control(
        ticket_mint::access_control(&ctx)
    )]
    pub fn ticket_mint(ctx: Context<TicketMint>, ticket_offset: u16) -> Result<()> {
        msg!("Instruction: TicketMint");
        ticket_mint::handler(ctx, ticket_offset)
    }

    /// Update an organizers data
    pub fn update_organizer(
        ctx: Context<UpdateOrganizer>,
        title: String,
        website: String,
    ) -> Result<()> {
        msg!("Instruction: UpdateOrganizer");
        update_organizer::handler(ctx, title, website)
    }

    /// Update an events data
    #[access_control(
        update_event::access_control(&ctx, tickets_limit)
    )]
    pub fn update_event(
        ctx: Context<UpdateEvent>,
        event_num: u32,
        title: String,
        website: String,
        tickets_limit: u16,
        timestamp: u64,
        location_type: u8,
        location: String,
        image_url: String,
    ) -> Result<()> {
        msg!("Instruction: UpdateEvent");
        update_event::handler(
            ctx,
            title,
            website,
            tickets_limit,
            timestamp,
            location_type,
            location,
            image_url,
        )
    }

    /// Create a new event pass, requires an organizer account
    #[access_control(
        create_event_pass::access_control(&ctx)
    )]
    pub fn create_event_pass(
        ctx: Context<CreateEventPass>,
        title: String,
        website: String,
        tickets_limit: u16,
        image_url: String,
    ) -> Result<()> {
        msg!("Instruction: CreateEventPass");
        create_event_pass::handler(ctx, title, website, tickets_limit, image_url)
    }

    /// Add an event to an event pass
    pub fn event_pass_add_event(ctx: Context<EventPassAddEvent>) -> Result<()> {
        msg!("Instruction: EventPassAddEvent");
        event_pass_add_event::handler(ctx)
    }

    /// Create a new event pass holder
    #[access_control(
        event_pass_holder_create::access_control(&ctx)
    )]
    pub fn event_pass_holder_create(ctx: Context<EventPassHolderCreate>) -> Result<()> {
        msg!("Instruction: EventPassHolderCreate");
        event_pass_holder_create::handler(ctx)
    }

    /// For an event pass holder, create a new ticket for a specific event
    #[access_control(
        ticket_issue_for_event_pass::access_control(&ctx)
    )]
    pub fn ticket_issue_for_event_pass(ctx: Context<TicketIssueForEventPass>, event_offset: u16, holder_offset: u16) -> Result<()> {
        msg!("Instruction: TicketIssueForEventPass");
        ticket_issue_for_event_pass::handler(ctx)
    }

    // Delete an event, including derived accounts
    //TODO: I need a function to delete an entire event with all its PDA tickets?

    // Delete an event pass, including derived accounts
    //TODO
}

// TODO - verify rent exemption for organizer, should be guaranteed when updating with longer strings

// TODO --
//  when minting NFTs, donate some coins to me (as implemented in IX create_event)
//  Add a PDA config & a function to update donate address and donate amount - or just update app ??

//TODO: Since event passes can be issued for any event, I probably want several ticket issue authorities per event?
// That way organizers could work together

//TODO: it is possible to create many EventPassValidEvent for the same event::key
// However the EventPassHolderTicket is seeded from event_pass_holder::key + event::key

//TODO: write proper tests
