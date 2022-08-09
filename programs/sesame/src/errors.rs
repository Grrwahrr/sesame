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

    #[msg("Ticket limit is less than the number of tickets already issued")]
    TicketLimitTooSmall, // 6004

    #[msg("Can not mint NFT for a ticket that was not checked in")]
    TicketWasNotCheckedIn, // 6005

    #[msg("The event creator must not be the ticket issue authority")]
    InvalidTicketAuthorityIssuer, // 6006

    #[msg("The event creator must not be the ticket check in authority")]
    InvalidTicketAuthorityCheckIn, // 6007

    #[msg("The event creator must not be the ticket delete authority")]
    InvalidTicketAuthorityDelete, // 6008
}
