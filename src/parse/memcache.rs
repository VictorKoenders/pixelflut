pub use self::num::*;

pub fn initialize() {
    assert_eq!(
        std::mem::size_of::<usize>(),
        std::mem::size_of::<u64>(),
        "Pixelflut is optimized for 64-bit platforms, but is run in a different platform (pointer size is {}, expected {})",
        std::mem::size_of::<usize>(),
        std::mem::size_of::<u64>(),
    );

    // Safety: Initialize should only be called once at the start of the application
    // therefor getting a mutable borrow should be fine.
    unsafe { NUM_CACHE.init() };
}

pub fn parse_coordinate(buffer: &[u8]) -> Option<(u16, &[u8])> {
    // Safety: NUM_CACHE won't change after `initialize` is called
    unsafe { NUM_CACHE.parse_coordinate(buffer) }
}
pub fn parse_color(_: &[u8]) -> Option<(u8, u8, u8)> {
    unimplemented!()
}

mod num {
    use crate::parse::MAX_VALID_NUMBER;
    use std::time::Instant;

    pub static mut NUM_CACHE: NumCache = NumCache::new();

    pub struct NumCache {
        entries: Vec<Option<u16>>,
    }
    impl NumCache {
        pub const fn new() -> Self {
            Self {
                entries: Vec::new(),
            }
        }
        pub fn init(&mut self) {
            println!("Initializing num cache, this might take a while");
            let start = Instant::now();
            self.allocate();

            for i in 0..=MAX_VALID_NUMBER {
                let str = i.to_string();
                let index = get_index_from_str(str.as_bytes());
                self.add(index, i as u16);
            }
            self.finalize();
            let size = self.memory_size();
            println!(
                "Integer parsing memory cache allocated {}mb in {:?}",
                size / 1024 / 1024,
                start.elapsed()
            );
        }

        fn allocate(&mut self) {
            let mut max_valid_number_str = MAX_VALID_NUMBER.to_string();
            for i in 1..max_valid_number_str.len() {
                max_valid_number_str.replace_range(i..=i, "9");
            }

            let expected_len = get_index_from_str(max_valid_number_str.as_bytes());
            self.entries.reserve(expected_len);
        }
        fn add(&mut self, index: usize, num: u16) {
            if self.entries.capacity() < index {
                println!("We did not allocate enough, ");
                println!("Tried adding number {} at index {}", num, index);
                println!("But we only have {} spots available", self.entries.len());
                panic!();
            }
            while self.entries.len() <= index {
                self.entries.push(None);
            }
            self.entries[index] = Some(num);
        }
        fn get(&self, index: usize) -> Option<u16> {
            self.entries.get(index).and_then(|e| *e)
        }
        fn memory_size(&self) -> usize {
            self.entries.len() * std::mem::size_of::<Option<u16>>()
        }
        pub fn finalize(&mut self) {
            self.entries.shrink_to_fit();
        }

        pub fn parse_coordinate<'a>(&self, buff: &'a [u8]) -> Option<(u16, &'a [u8])> {
            for i in [4, 3, 2, 1] {
                if buff.get(i) != Some(&b' ') {
                    continue;
                }
                // Safe because we know that buff[i] is a space
                let range = unsafe { buff.get_unchecked(..i) };
                let remaining = unsafe { buff.get_unchecked(i + 1..) };

                let index = get_index_from_str(range);
                if let Some(result) = self.get(index) {
                    return Some((result, remaining));
                }
            }
            None
        }
    }

    /// Turn a string into an integer. Only works with strings that are up to 4 bytes long.
    fn get_index_from_str(s: &[u8]) -> usize {
        // s.len().min(4) is slower than not checking it
        let len = s.len();

        // check that the string isn't too long, but only in debug mode
        if cfg!(debug_assertions) && len > 4 {
            panic!(
                "Could not get index from str with a len larger than 8 (requested {})",
                len
            );
        }

        let mut val: u32 = 0;
        // Copy over the first `len` bytes to `val`
        unsafe {
            std::ptr::copy_nonoverlapping(s.as_ptr(), &mut val as *mut u32 as *mut u8, len);
        }

        val as usize
    }

    #[test]
    fn test_get_index_from_str() {
        // "abc" is 0x616263
        assert_eq!(get_index_from_str(b"abc"), 0x0063_6261);
        assert_eq!(get_index_from_str(&b"abcdef"[0..3]), 0x0063_6261);
        assert_eq!(get_index_from_str(b"640"), 0x0030_3436);

        assert_eq!(get_index_from_str(b"a"), 0x61);
        assert_eq!(get_index_from_str(b"0"), 0x30);
        assert_eq!(get_index_from_str(b"20"), 0x3032);
        assert_eq!(get_index_from_str(b"920"), 0x30_3239);
        assert_eq!(get_index_from_str(b"1920"), 0x3032_3931);
    }

    #[test]
    #[should_panic]
    fn test_get_index_from_str_overflow() {
        // If we pass a str with length > 4, it should panic
        get_index_from_str(b"12345");
    }

    #[test]
    fn test_values() {
        let mut memcache = NumCache::new();
        memcache.init();
        assert_eq!(
            memcache.parse_coordinate(b"1920 "),
            Some((1920u16, Default::default()))
        );
    }
}
