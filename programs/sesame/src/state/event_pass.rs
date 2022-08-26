use anchor_lang::prelude::*;

#[account]
#[derive(Default)]
pub struct EventPass {
    /// Version
    pub version: u8,

    /// This key identifies the admin
    pub admin: Pubkey,

    /// This key identifies the pass vendor authority
    pub pass_authority_issuer: Pubkey,

    /// This key identifies the pass delete authority
    pub pass_authority_delete: Pubkey,

    /// The maximum number of tickets that can be issued per pass holder
    pub tickets_limit: u16,

    /// The number of events that have been attached
    pub counter_events: u16,

    /// The number of event pass holders
    pub counter_holders: u16,

    /// The maximum number of passes that can be issued
    pub limit_holders: u16,

    /// Event pass title
    pub title: String,

    /// Event pass website
    pub website: String,

    /// Event pass artwork
    pub artwork: String,
}

impl EventPass {
    pub const LEN: usize = std::mem::size_of::<EventPass>() + 600;
}
