use anchor_lang::prelude::*;

#[account]
#[derive(Default)]
pub struct EventPassValidEvent {
    /// This key identifies the event
    pub event: Pubkey,
}

impl EventPassValidEvent {
    pub const LEN: usize = std::mem::size_of::<EventPassValidEvent>() + 8;
}
