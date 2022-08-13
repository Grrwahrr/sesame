use anchor_lang::prelude::*;

#[account]
#[derive(Default)]
pub struct Organizer {
    /// Number of events created
    pub counter_events: u32,

    /// Number of passes created
    pub counter_passes: u32,

    /// Organizer Title
    pub title: String,

    /// Organizer URL
    pub website: String,
}

impl Organizer {
    pub const LEN: usize = std::mem::size_of::<Organizer>() + 260;
}
