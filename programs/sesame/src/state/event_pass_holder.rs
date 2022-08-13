use anchor_lang::prelude::*;

#[account]
#[derive(Default)]
pub struct EventPassHolder {
    /// This key identifies the owner
    pub owner: Pubkey,

    /// The number of tickets already created by this pass
    pub tickets_created: u16,
}

impl EventPassHolder {
    pub const LEN: usize = std::mem::size_of::<EventPassHolder>() + 8;
}
