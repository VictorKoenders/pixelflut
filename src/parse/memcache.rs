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
                let index = get_index_from_str(format!("{} ", i).as_bytes()).unwrap().0;
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
            let mut max_idx = 0;
            for i in 0..=MAX_VALID_NUMBER {
                let (idx, _) = get_index_from_str(format!("{} ", i).as_bytes()).unwrap();
                max_idx = max_idx.max(idx);
            }

            self.entries.reserve(max_idx);
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
            let (index, len) = get_index_from_str_v1(buff)?;
            if let Some(result) = self.get(index) {
                return Some((result, unsafe { buff.get_unchecked(len..) }));
            }

            None
        }
    }

    /// Turn a string into an integer. Only works with strings that are up to 4 bytes long.
    /// Will always point at the fastest implementation.
    #[inline]
    pub fn get_index_from_str(s: &[u8]) -> Option<(usize, usize)> {
        get_index_from_str_v2(s)
    }

    /// Turn a string into an integer. Only works with strings that are up to 4 bytes long.
    #[inline]
    pub fn get_index_from_str_v1(s: &[u8]) -> Option<(usize, usize)> {
        for i in 1..s.len().min(5) {
            if let Some(b' ') = s.get(i) {
                let len = i;
                let slice = unsafe { s.get_unchecked(..i) };

                let mut target = [0u8; 4];
                target[..len].copy_from_slice(slice);

                let idx = u32::from_le_bytes(target);
                return Some((idx as usize, len + 1));
            }
        }
        None
    }

    /// Turn a string into an integer. Only works with strings that are up to 4 bytes long.
    #[inline]
    pub fn get_index_from_str_v2(s: &[u8]) -> Option<(usize, usize)> {
        let len = s.iter().take(5).position(|b| *b == b' ')?;

        let mut target = [0u8; 4];
        target[..len].copy_from_slice(&s[0..len]);

        let idx = u32::from_le_bytes(target);
        Some((idx as usize, len + 1))
    }

    #[test]
    fn test_get_index_from_str() {
        // "abc" is 0x616263
        assert_eq!(get_index_from_str_v1(b"abc "), Some((0x0063_6261, 4)));
        assert_eq!(get_index_from_str_v1(b"640 "), Some((0x0030_3436, 4)));
        assert_eq!(get_index_from_str_v1(b"a "), Some((0x61, 2)));
        assert_eq!(get_index_from_str_v1(b"0 "), Some((0x30, 2)));
        assert_eq!(get_index_from_str_v1(b"20 "), Some((0x3032, 3)));
        assert_eq!(get_index_from_str_v1(b"920 "), Some((0x30_3239, 4)));
        assert_eq!(get_index_from_str_v1(b"1920 "), Some((0x3032_3931, 5)));

        assert_eq!(get_index_from_str_v2(b"abc "), Some((0x0063_6261, 4)));
        assert_eq!(get_index_from_str_v2(b"640 "), Some((0x0030_3436, 4)));
        assert_eq!(get_index_from_str_v2(b"a "), Some((0x61, 2)));
        assert_eq!(get_index_from_str_v2(b"0 "), Some((0x30, 2)));
        assert_eq!(get_index_from_str_v2(b"20 "), Some((0x3032, 3)));
        assert_eq!(get_index_from_str_v2(b"920 "), Some((0x30_3239, 4)));
        assert_eq!(get_index_from_str_v2(b"1920 "), Some((0x3032_3931, 5)));
    }

    #[test]
    fn test_values() {
        let mut memcache = NumCache::new();
        memcache.init();
        assert_eq!(
            memcache.parse_coordinate(b"1920 "),
            Some((1920u16, Default::default()))
        );
        assert_eq!(get_index_from_str_v1(b"503 2 "), Some((0x333035, 4)));
    }
}
