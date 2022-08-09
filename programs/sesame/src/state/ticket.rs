use anchor_lang::prelude::*;

#[derive(Debug, Clone, AnchorSerialize, AnchorDeserialize, PartialEq, Eq)]
#[repr(u8)]
pub enum TicketState {
    Initial = 0,
    CheckedIn = 1,
    Minted = 2,
}

impl From<u8> for TicketState {
    fn from(value: u8) -> TicketState {
        match value {
            0 => TicketState::Initial,
            1 => TicketState::CheckedIn,
            2 => TicketState::Minted,
            _ => panic!("Unknown value: {}", value),
        }
    }
}

impl Default for TicketState {
    fn default() -> Self {
        TicketState::Initial
    }
}

/**
 * Each ticket is its own account with state information as well as a pubkey identifying the ticket owner.
 */
#[account]
#[derive(Default)]
pub struct Ticket {
    /// Ticket state
    pub state: TicketState,

    /// Ticket owner
    pub owner: Pubkey,
}

impl Ticket {
    pub const LEN: usize = std::mem::size_of::<Ticket>() + 8;
}
