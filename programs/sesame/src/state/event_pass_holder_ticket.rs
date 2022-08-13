use anchor_lang::prelude::*;

#[account]
#[derive(Default)]
pub struct EventPassHolderTicket {}

impl EventPassHolderTicket {
    pub const LEN: usize = std::mem::size_of::<EventPassHolderTicket>() + 8;
}
