pub mod bytewise;
#[cfg(feature = "memory-cache")]
pub mod memcache;
pub mod python_generated;
pub mod std;

pub const MAX_VALID_NUMBER: u16 = 1920;

cfg_if::cfg_if! {
    if #[cfg(feature = "memory-cache")] {
        pub use self::memcache::*;
    } else {
        // Note: Currently bytewise is faster than memory-cache
        pub use self::bytewise::*;
    }
}

#[test]
fn validate() {
    for i in 0..=MAX_VALID_NUMBER {
        let str = format!("{} ", i);
        assert_eq!(
            python_generated::parse_coordinate(str.as_bytes()),
            Some((i, [].as_slice()))
        );
        assert_eq!(
            std::parse_coordinate(str.as_bytes()),
            Some((i, [].as_slice()))
        );
        assert_eq!(
            bytewise::parse_coordinate(str.as_bytes()),
            Some((i, [].as_slice()))
        );
    }

    let i = MAX_VALID_NUMBER + 1;
    let str = format!("{} ", i);
    assert_eq!(python_generated::parse_coordinate(str.as_bytes()), None);
    assert_eq!(std::parse_coordinate(str.as_bytes()), None);
    assert_eq!(bytewise::parse_coordinate(str.as_bytes()), None);
}
