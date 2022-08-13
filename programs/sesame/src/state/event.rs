use anchor_lang::prelude::*;

#[derive(Debug, Clone, AnchorSerialize, AnchorDeserialize, PartialEq, Eq)]
#[repr(u8)]
pub enum LocationType {
    Txt = 0,
    Gps = 1,
    Url = 2,
}
impl From<u8> for LocationType {
    fn from(value: u8) -> LocationType {
        match value {
            0 => LocationType::Txt,
            1 => LocationType::Gps,
            2 => LocationType::Url,
            _ => panic!("Unknown value: {}", value),
        }
    }
}

impl Default for LocationType {
    fn default() -> Self {
        LocationType::Txt
    }
}

#[account]
#[derive(Default)]
pub struct Event {
    /// Version
    pub version: u8,

    /// This key identifies the admin
    pub admin: Pubkey,

    /// This key identifies the ticket vendor authority
    pub ticket_authority_issuer: Pubkey,

    /// This key identifies the ticket delete authority
    pub ticket_authority_delete: Pubkey,

    /// This key identifies the ticket check in authority
    pub ticket_authority_check_in: Pubkey,

    /// The maximum number of tickets that can be issued
    pub tickets_limit: u16,

    /// The number of tickets that have been issued
    pub tickets_issued: u16,

    /// The number of tickets that have been deleted
    pub tickets_deleted: u16,

    /// Event time; as unix time in seconds; UTC timezone
    pub timestamp: u64,

    /// Event title
    pub title: String,

    /// Event website
    pub website: String,

    /// Type of location
    pub location_type: LocationType,

    /// Event location
    pub location: String,

    /// Event artwork
    pub artwork: String,
}

impl Event {
    pub const LEN: usize = std::mem::size_of::<Event>() + 600;
}
