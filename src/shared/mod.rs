mod controller_data;

pub use controller_data::*;

pub const ADDRESS_LENGTH: usize = 5;
pub const ADDRESS: &[u8; ADDRESS_LENGTH] = b"mycar";

pub const CHANNEL: u8 = 120;