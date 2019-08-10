mod parse_hex;
mod parse_usize;

pub use self::parse_hex::parse_v1 as parse_hex;
pub use self::parse_usize::{initialize_v2_cache as initialize_usize, parse_v2 as parse_usize};
