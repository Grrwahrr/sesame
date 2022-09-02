use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg("Overflow Error")]
    OverflowError, // 6000

    #[msg("Not authorized to perform this action")]
    NotAuthorized, // 6001

    #[msg("No more tickets may be issued")]
    NoMoreTicketsLeft, // 6002

    #[msg("Ticket has already checked in")]
    TicketAlreadyCheckedIn, // 6003

    #[msg("There are no more tickets left")]
    NoTicketsLeft, // 6004

    #[msg("Can not mint NFT for a ticket that was not checked in")]
    TicketWasNotCheckedIn, // 6005

    #[msg("The event creator must not be the ticket issue authority")]
    InvalidTicketAuthorityIssuer, // 6006

    #[msg("The event creator must not be the ticket check in authority")]
    InvalidTicketAuthorityCheckIn, // 6007

    #[msg("The event creator must not be the ticket delete authority")]
    InvalidTicketAuthorityDelete, // 6008

    #[msg("The event pass creator must not be the pass issue authority")]
    InvalidPassAuthorityIssuer, // 6009

    #[msg("The event pass creator must not be the pass delete authority")]
    InvalidPassAuthorityDelete, // 6010

    #[msg("This event pass can not issue any more tickets")]
    NoMoreTicketsLeftInEventPass, // 6011

    #[msg("There are no more passes that can be issued")]
    LimitOfEventPassHoldersReached, // 6012

    #[msg("There are already more event pass holders than allowed by the given limit")]
    MoreHoldersExist, // 6013

    #[msg("There are already more tickets for this event than allowed by the given limit")]
    MoreTicketsIssued, // 6014
}
