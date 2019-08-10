#[cfg(test)]
use hashbrown::HashMap;
#[cfg(test)]
use test::{black_box, Bencher};

const MAX_VALID_NUMBER: usize = 1920;

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

static mut V2_CACHE: NumCache = NumCache::new();

pub fn initialize_v2_cache() {
    if unsafe { V2_CACHE.is_initialized() } {
        return;
    }
    assert_eq!(
        std::mem::size_of::<u64>(),
        std::mem::size_of::<usize>(),
        "Pixelflut is optimized for 64-bit platforms, but is run in a different configuration"
    );
    unsafe { V2_CACHE.allocate() };
    // Sanity check
    super::do_test_get_index_from_str();
    // Seed for all numbers < MAX_VALID_NUMBER
    for i in 0..MAX_VALID_NUMBER {
        let str = i.to_string();
        let index = super::get_index_from_str(str.as_bytes());
        unsafe { V2_CACHE.add(index, i as u16, str.len() as u8) }
    }
    let size = unsafe { V2_CACHE.memory_size() };
    println!("Integer parsing memory cache: {}mb", size / 1024 / 1024);
}

pub fn parse_v2(buff: &[u8]) -> Option<usize> {
    parse_with_len_v2(buff).map(|(num, _)| num)
}

pub fn parse_with_len_v2(buff: &[u8]) -> Option<(usize, usize)> {
    #[cfg(test)]
    initialize_v2_cache();
    for &i in &[4, 3, 2, 1] {
        let range = match buff.get(..i) {
            Some(r) => r,
            None => continue,
        };
        let index = super::get_index_from_str(range);
        if let Some(result) = unsafe { V2_CACHE.get(index) } {
            return Some((result.num as usize, result.str_len as usize));
        }
    }
    None
}

#[cfg(test)]
static mut V3_CACHE: Option<HashMap<usize, NumCacheEntry>> = None;

#[cfg(test)]
pub fn initialize_v3_cache() {
    if unsafe { V3_CACHE.is_some() } {
        return;
    }
    assert_eq!(
        std::mem::size_of::<u64>(),
        std::mem::size_of::<usize>(),
        "Pixelflut is optimized for 64-bit platforms, but is run in a different configuration"
    );
    unsafe { V3_CACHE = Some(HashMap::with_capacity(MAX_VALID_NUMBER)) };
    // Sanity check
    super::do_test_get_index_from_str();
    // Seed for all numbers < MAX_VALID_NUMBER
    for i in 0..MAX_VALID_NUMBER {
        let str = i.to_string();
        let index = super::get_index_from_str(str.as_bytes());
        unsafe {
            V3_CACHE.as_mut().unwrap().insert(
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
pub fn parse_v3(buff: &[u8]) -> Option<usize> {
    parse_with_len_v3(buff).map(|(num, _)| num)
}

#[cfg(test)]
pub fn parse_with_len_v3(buff: &[u8]) -> Option<(usize, usize)> {
    #[cfg(test)]
    initialize_v3_cache();
    for &i in &[4, 3, 2, 1] {
        let range = match buff.get(..i) {
            Some(r) => r,
            None => continue,
        };
        let index = super::get_index_from_str(range);
        if let Some(result) = unsafe { V3_CACHE.as_ref().unwrap().get(&index) } {
            return Some((result.num as usize, result.str_len as usize));
        }
    }
    None
}
#[test]
fn validate_parse_v2() {
    initialize_v2_cache();
    for i in 0..MAX_VALID_NUMBER {
        let s = format!("{}", i);
        let result = match parse_with_len_v2(s.as_bytes()) {
            Some(n) => n,
            None => panic!("Could not decode num {}", i),
        };
        assert_eq!(i, result.0);
    }
}
#[test]
fn validate_parse_v3() {
    initialize_v3_cache();
    for i in 0..MAX_VALID_NUMBER {
        let s = format!("{}", i);
        let result = match parse_with_len_v3(s.as_bytes()) {
            Some(n) => n,
            None => panic!("Could not decode num {}", i),
        };
        assert_eq!(i, result.0);
    }
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
#[bench]
fn bench_parse_v3(b: &mut Bencher) {
    let u = b"640";
    initialize_v3_cache();
    b.iter(|| {
        let u = black_box(u);
        for _ in 0..100_000 {
            let u = parse_v3(&u[..]).unwrap();
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
