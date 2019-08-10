#[cfg(test)]
use test::{black_box, Bencher};

#[cfg(test)]
pub fn parse_v1(buff: &[u8]) -> Option<usize> {
    let mut result = 0;
    for b in buff {
        let b = *b;
        if b >= b'0' && b <= b'9' {
            result = result * 10 + (b - b'0') as usize;
        } else {
            return None;
        }

        // We assume we'll never got 1 bil
        if result > 1_000_000_000 {
            return None;
        }
    }
    Some(result)
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
    pub fn add(&mut self, index: usize, num: u16, str_len: u8) {
        if self.entries.len() < index {
            self.entries.reserve(index - self.entries.len());
        }
        while self.entries.len() <= index {
            self.entries.push(None);
        }
        self.entries[index] = Some(NumCacheEntry { num, str_len });
    }
    pub fn get(&self, index: usize) -> Option<NumCacheEntry> {
        self.entries.get(index).and_then(|a| *a)
    }
}
#[derive(Copy, Clone, Debug)]
struct NumCacheEntry {
    num: u16,
    str_len: u8,
}
static mut V2_CACHE_1_CHARS: NumCache = NumCache::new();
static mut V2_CACHE_2_CHARS: NumCache = NumCache::new();
static mut V2_CACHE_3_CHARS: NumCache = NumCache::new();

fn get_index_from_str(s: &[u8]) -> usize {
    let len = s.len();
    let bit_len = len * 8;
    if cfg!(debug_assertions) && len > 8 {
        panic!(
            "Could not get index from str with a len larger than 8 (requested {})",
            len
        );
    }
    let mut val: u64 = 0;
    unsafe {
        std::ptr::copy_nonoverlapping(s.as_ptr(), &mut val as *mut u64 as *mut u8, 8);
    }

    // make a bitwise mask for the last `len` chars
    let mask = (1 << bit_len) - 1;

    (val & mask) as usize
}

#[test]
fn test_get_index_from_str() {
    do_test_get_index_from_str();
}
fn do_test_get_index_from_str() {
    // "abc" is 0x616263
    assert_eq!(get_index_from_str(b"abc"), 0x0063_6261);
    assert_eq!(get_index_from_str(&b"abcdef"[0..3]), 0x0063_6261);
    assert_eq!(get_index_from_str(b"640"), 0x0030_3436);
}

pub fn initialize_v2_cache() {
    assert_eq!(
        std::mem::size_of::<u64>(),
        std::mem::size_of::<usize>(),
        "Pixelflut is optimized for 64-bit platforms, but is run in a different configuration"
    );
    // Sanity check
    do_test_get_index_from_str();
    // Seed for all numbers < 999
    for i in 0..=999 {
        let str = i.to_string();
        let index = get_index_from_str(str.as_bytes());

        if i <= 9 {
            unsafe { V2_CACHE_1_CHARS.add(index, i, str.len() as u8) }
        }

        if i <= 99 {
            unsafe { V2_CACHE_2_CHARS.add(index, i, str.len() as u8) }
        }

        unsafe { V2_CACHE_3_CHARS.add(index, i, str.len() as u8) }
    }
}

pub fn parse_v2(buff: &[u8]) -> Option<usize> {
    for (i, cache) in &[
        (3, unsafe { &V2_CACHE_3_CHARS }),
        (2, unsafe { &V2_CACHE_2_CHARS }),
        (1, unsafe { &V2_CACHE_1_CHARS }),
    ] {
        let i = *i;
        if buff.len() <= i {
            continue;
        }
        let index = get_index_from_str(&buff[..i]);
        if let Some(result) = cache.get(index) {
            return Some(result.num as usize);
        }
    }
    None
}

#[bench]
fn bench_parse_v1(b: &mut Bencher) {
    let u = b"640";
    b.iter(|| {
        let u = black_box(u);
        for _ in 0..100_000 {
            let u = parse_v1(&u[..]).unwrap();
            black_box(u);
        }
    });
}

#[bench]
fn bench_parse_v2(b: &mut Bencher) {
    let u = b"640";
    initialize_v2_cache();
    b.iter(|| {
        let u = black_box(u);
        for _ in 0..100_000 {
            let u = parse_v2(&u[..]).unwrap();
            black_box(u);
        }
    });
}
#[cfg(test)]
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
