#[cfg(not(feature = "memcache"))]
pub use self::impl_v1::*;
#[cfg(feature = "memcache")]
pub use self::impl_v2::*;

#[cfg(any(test, not(feature = "memcache")))]
mod impl_v1 {
    pub fn parse(buff: Option<&[u8]>) -> Option<u8> {
        let buff = buff?;
        let mut result = 0;
        for b in buff {
            let b = *b;
            if b >= b'0' && b <= b'9' {
                result = result * 16 + (b - b'0');
            } else if b >= b'A' && b <= b'F' {
                result = result * 16 + (b - b'A' + 10);
            } else if b >= b'a' && b <= b'f' {
                result = result * 16 + (b - b'a' + 10);
            } else {
                return None;
            }
        }
        Some(result)
    }
    pub fn initialize() {}
}

#[cfg(any(feature = "memcache", test))]
mod impl_v2 {
    use crate::utils::get_index_from_str;
    use std::sync::Mutex;

    struct NumCache {
        entries: Vec<Option<NumCacheEntry>>,
    }
    impl NumCache {
        pub const fn new() -> Self {
            Self {
                entries: Vec::new(),
            }
        }
        pub fn is_initialized(&self) -> bool {
            !self.entries.is_empty()
        }
        pub fn add(&mut self, index: usize, val: u8) {
            if self.entries.len() < index {
                self.entries.reserve(index - self.entries.len());
            }
            while self.entries.len() <= index {
                self.entries.push(None);
            }
            self.entries[index] = Some(NumCacheEntry { val });
        }
        pub fn get(&self, index: usize) -> Option<NumCacheEntry> {
            self.entries.get(index).and_then(|a| *a)
        }
    }

    #[derive(Copy, Clone, Debug)]
    struct NumCacheEntry {
        val: u8,
    }
    static mut CACHE: NumCache = NumCache::new();

    lazy_static::lazy_static! {
        static ref CACHE_LOCK: Mutex<()> = Mutex::new(());
    }

    pub fn initialize() {
        let _lock = CACHE_LOCK.lock();
        if unsafe { CACHE.is_initialized() } {
            return;
        }
        for val in 0..=256u16 {
            let str = format!("{:02X}", val);
            let index = get_index_from_str(str.as_bytes());
            unsafe {
                CACHE.add(index, val as u8);
            }

            let str = format!("{:02x}", val);
            let index = get_index_from_str(str.as_bytes());
            unsafe {
                CACHE.add(index, val as u8);
            }
        }
    }

    pub fn parse(b: Option<&[u8]>) -> Option<u8> {
        #[cfg(test)]
        initialize();

        let b = b?;
        debug_assert_eq!(2, b.len());
        let index = get_index_from_str(b);
        unsafe { CACHE.get(index) }.map(|v| v.val)
    }
}

#[cfg(test)]
mod benches {
    use super::*;
    use test::{black_box, Bencher};

    #[bench]
    fn bench_parse_std(b: &mut Bencher) {
        let u = b"FF";
        b.iter(|| {
            let u = black_box(u);
            for _ in 0..10_000 {
                let str = ::std::str::from_utf8(u).unwrap();
                let u = u8::from_str_radix(&str[..], 16).unwrap();
                black_box(u);
            }
        });
    }

    #[bench]
    fn bench_parse_v1(b: &mut Bencher) {
        let u = b"FF";
        b.iter(|| {
            let s = black_box(u);
            for _ in 0..10_000 {
                let u = impl_v1::parse(Some(&s[..]));
                black_box(u);
            }
        });
    }

    #[bench]
    fn bench_parse_v2(b: &mut Bencher) {
        impl_v2::initialize();
        let u = b"FF";
        b.iter(|| {
            let u = black_box(u);
            for _ in 0..10_000 {
                let u = impl_v2::parse(Some(&u[..])).unwrap();
                black_box(u);
            }
        });
    }
}
