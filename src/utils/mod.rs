mod parse_hex;
mod parse_usize;

pub use self::parse_hex::{initialize_v2 as initialize_hex, parse_v2 as parse_hex};
pub use self::parse_usize::{
    initialize_v2_cache as initialize_usize, parse_v2 as parse_usize,
    parse_with_len_v2 as parse_usize_with_len,
};

pub(self) fn get_index_from_str(s: &[u8]) -> usize {
    let len = s.len();
    if cfg!(debug_assertions) && len > 8 {
        panic!(
            "Could not get index from str with a len larger than 8 (requested {})",
            len
        );
    }
    let mut val: u64 = 0;
    unsafe {
        std::ptr::copy_nonoverlapping(s.as_ptr(), &mut val as *mut u64 as *mut u8, len);
    }

    val as usize
}

#[test]
fn test_get_index_from_str() {
    do_test_get_index_from_str();
}

pub(self) fn do_test_get_index_from_str() {
    // "abc" is 0x616263
    assert_eq!(get_index_from_str(b"abc"), 0x0063_6261);
    assert_eq!(get_index_from_str(&b"abcdef"[0..3]), 0x0063_6261);
    assert_eq!(get_index_from_str(b"640"), 0x0030_3436);
}
