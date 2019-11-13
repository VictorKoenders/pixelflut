const MAX_VALID_NUMBER: usize = 1920;

#[cfg(not(feature = "memcache"))]
pub use self::impl_v1::*;
#[cfg(feature = "memcache")]
pub use self::impl_v2::*;

#[cfg(any(test, not(feature = "memcache")))]
mod impl_v1 {
    pub fn parse(buff: &[u8]) -> Option<usize> {
        parse_with_len(buff).map(|(num, _)| num)
    }

    pub fn parse_with_len(buff: &[u8]) -> Option<(usize, usize)> {
        let mut result = 0;
        for (index, b) in buff.iter().enumerate() {
            let b = *b;
            if b >= b'0' && b <= b'9' {
                result = result * 10 + (b - b'0') as usize;
            } else if index > 0 {
                return Some((result, index));
            } else {
                return None;
            }

            // We assume we'll never got 1 bil
            if result > 1_000_000_000 {
                return None;
            }
        }
        Some((result, buff.len()))
    }
    pub fn initialize_cache() {}
}

#[cfg(any(feature = "memcache", test))]
mod impl_v2 {
    use super::{NumCache, MAX_VALID_NUMBER};
    use crate::utils::{do_test_get_index_from_str, get_index_from_str};
    use std::sync::Mutex;

    static mut CACHE: NumCache = NumCache::new();
    lazy_static::lazy_static! {
        static ref CACHE_LOCK: Mutex<()> = Mutex::new(());
    }

    pub fn initialize_cache() {
        let _lock = CACHE_LOCK.lock();
        if unsafe { CACHE.is_initialized() } {
            return;
        }
        assert_eq!(
            std::mem::size_of::<u64>(),
            std::mem::size_of::<usize>(),
            "Pixelflut is optimized for 64-bit platforms, but is run in a different configuration"
        );
        unsafe { CACHE.allocate() };
        // Sanity check
        do_test_get_index_from_str();
        // Seed for all numbers < MAX_VALID_NUMBER
        for i in 0..MAX_VALID_NUMBER {
            let str = i.to_string();
            let index = get_index_from_str(str.as_bytes());
            unsafe { CACHE.add(index, i as u16, str.len() as u8) }
        }
        let size = unsafe { CACHE.memory_size() };
        println!("Integer parsing memory cache: {}mb", size / 1024 / 1024);
    }

    pub fn parse(buff: &[u8]) -> Option<usize> {
        parse_with_len(buff).map(|(num, _)| num)
    }

    pub fn parse_with_len(buff: &[u8]) -> Option<(usize, usize)> {
        #[cfg(test)]
        initialize_cache();
        for &i in &[4, 3, 2, 1] {
            let range = match buff.get(..i) {
                Some(r) => r,
                None => continue,
            };
            let index = get_index_from_str(range);
            if let Some(result) = unsafe { CACHE.get(index) } {
                return Some((result.num as usize, result.str_len as usize));
            }
        }
        None
    }
}

#[cfg(test)]
mod impl_v3 {
    use super::{NumCacheEntry, MAX_VALID_NUMBER};
    use crate::utils::{do_test_get_index_from_str, get_index_from_str};
    use std::collections::HashMap;
    use std::sync::Mutex;

    static mut CACHE: Option<HashMap<usize, NumCacheEntry>> = None;
    lazy_static::lazy_static! {
        static ref CACHE_LOCK: Mutex<()> = Mutex::new(());
    }

    pub fn initialize_cache() {
        let _lock = CACHE_LOCK.lock();
        if unsafe { CACHE.is_some() } {
            return;
        }
        assert_eq!(
            std::mem::size_of::<u64>(),
            std::mem::size_of::<usize>(),
            "Pixelflut is optimized for 64-bit platforms, but is run in a different configuration"
        );
        unsafe { CACHE = Some(HashMap::with_capacity(MAX_VALID_NUMBER)) };
        // Sanity check
        do_test_get_index_from_str();
        // Seed for all numbers < MAX_VALID_NUMBER
        for i in 0..MAX_VALID_NUMBER {
            let str = i.to_string();
            let index = get_index_from_str(str.as_bytes());
            unsafe {
                CACHE.as_mut().unwrap().insert(
                    index,
                    NumCacheEntry {
                        num: i as u16,
                        str_len: str.len() as u8,
                    },
                );
            }
        }
    }

    #[cfg(test)]
    pub fn parse(buff: &[u8]) -> Option<usize> {
        parse_with_len(buff).map(|(num, _)| num)
    }

    #[cfg(test)]
    pub fn parse_with_len(buff: &[u8]) -> Option<(usize, usize)> {
        #[cfg(test)]
        initialize_cache();
        for &i in &[4, 3, 2, 1] {
            let range = match buff.get(..i) {
                Some(r) => r,
                None => continue,
            };
            let index = get_index_from_str(range);
            if let Some(result) = unsafe { CACHE.as_ref().unwrap().get(&index) } {
                return Some((result.num as usize, result.str_len as usize));
            }
        }
        None
    }
}
#[test]
fn validate_parse_v2() {
    impl_v2::initialize_cache();
    for i in 0..MAX_VALID_NUMBER {
        let s = format!("{}", i);
        let result = match impl_v2::parse_with_len(s.as_bytes()) {
            Some(n) => n,
            None => panic!("Could not decode num {}", i),
        };
        assert_eq!(i, result.0);
    }
}
#[test]
fn validate_parse_v3() {
    impl_v3::initialize_cache();
    for i in 0..MAX_VALID_NUMBER {
        let s = format!("{}", i);
        let result = match impl_v3::parse_with_len(s.as_bytes()) {
            Some(n) => n,
            None => panic!("Could not decode num {}", i),
        };
        assert_eq!(i, result.0);
    }
}

#[cfg(test)]
mod benches {
    #[cfg(test)]
    use test::{black_box, Bencher};

    use super::*;

    #[bench]
    fn bench_parse_v1(b: &mut Bencher) {
        let u = b"640";
        b.iter(|| {
            let u = black_box(u);
            for _ in 0..100_000 {
                let u = impl_v1::parse(&u[..]).unwrap();
                black_box(u);
            }
        });
    }

    #[bench]
    fn bench_parse_v2(b: &mut Bencher) {
        let u = b"640";
        impl_v2::initialize_cache();
        b.iter(|| {
            let u = black_box(u);
            for _ in 0..100_000 {
                let u = impl_v2::parse(&u[..]).unwrap();
                black_box(u);
            }
        });
    }

    #[bench]
    fn bench_parse_v3(b: &mut Bencher) {
        let u = b"640";
        impl_v3::initialize_cache();
        b.iter(|| {
            let u = black_box(u);
            for _ in 0..100_000 {
                let u = impl_v3::parse(&u[..]).unwrap();
                black_box(u);
            }
        });
    }

    #[bench]
    fn bench_parse_std(b: &mut Bencher) {
        let u = b"640";
        b.iter(|| {
            let u = black_box(u);
            for _ in 0..100_000 {
                let str = ::std::str::from_utf8(u).unwrap();
                let u: usize = str.parse().unwrap();
                black_box(u);
            }
        });
    }
}

struct NumCache {
    entries: Vec<Option<NumCacheEntry>>,
}
impl NumCache {
    pub const fn new() -> Self {
        Self {
            entries: Vec::new(),
        }
    }
    pub fn allocate(&mut self) {
        let mut max_valid_number_str = MAX_VALID_NUMBER.to_string();
        for i in 1..max_valid_number_str.len() {
            max_valid_number_str.replace_range(i..=i, "9");
        }

        let expected_len = super::get_index_from_str(max_valid_number_str.as_bytes());
        self.entries.reserve(expected_len);
    }
    pub fn add(&mut self, index: usize, num: u16, str_len: u8) {
        if self.entries.capacity() < index {
            println!("We did not allocate enough, ");
            println!("Tried adding number {} at index {}", num, index);
            println!("But we only have {} spots available", self.entries.len());
            panic!();
        }
        while self.entries.len() <= index {
            self.entries.push(None);
        }
        self.entries[index] = Some(NumCacheEntry { num, str_len });
    }
    pub fn get(&self, index: usize) -> Option<NumCacheEntry> {
        self.entries.get(index).and_then(|a| *a)
    }
    pub fn memory_size(&self) -> usize {
        self.entries.len() * std::mem::size_of::<Option<NumCacheEntry>>()
    }
    pub fn is_initialized(&self) -> bool {
        !self.entries.is_empty()
    }
}
#[derive(Copy, Clone, Debug)]
struct NumCacheEntry {
    num: u16,
    str_len: u8,
}
