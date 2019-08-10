#[cfg(test)]
use test::{black_box, Bencher};

#[cfg(test)]
pub fn parse_v1(buff: &[u8]) -> Option<u8> {
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

struct NumCache {
    entries: Vec<Option<NumCacheEntry>>,
}
impl NumCache {
    pub const fn new() -> Self {
        Self {
            entries: Vec::new(),
        }
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
static mut V2_CACHE: NumCache = NumCache::new();

pub fn initialize_v2() {
    for val in 0..=256u16 {
        let str = format!("{:02X}", val);
        let index = super::get_index_from_str(str.as_bytes());
        unsafe {
            V2_CACHE.add(index, val as u8);
        }

        let str = format!("{:02x}", val);
        let index = super::get_index_from_str(str.as_bytes());
        unsafe {
            V2_CACHE.add(index, val as u8);
        }
    }
}

pub fn parse_v2(b: &[u8]) -> Option<u8> {
    let index = super::get_index_from_str(&b[..2]);
    unsafe { V2_CACHE.get(index) }.map(|v| v.val)
}

#[cfg(test)]
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

#[cfg(test)]
#[bench]
fn bench_parse_v1(b: &mut Bencher) {
    let u = b"FF";
    b.iter(|| {
        let s = black_box(u);
        for _ in 0..10_000 {
            let u = parse_v1(&s[..]);
            black_box(u);
        }
    });
}

#[bench]
fn bench_parse_v2(b: &mut Bencher) {
    initialize_v2();
    let u = b"FF";
    b.iter(|| {
        let u = black_box(u);
        for _ in 0..10_000 {
            let u = parse_v2(&u[..]).unwrap();
            black_box(u);
        }
    });
}
