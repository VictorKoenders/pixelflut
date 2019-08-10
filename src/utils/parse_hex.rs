#[cfg(test)]
use test::{black_box, Bencher};

#[cfg(test)]
#[bench]
fn bench_parse_v1(b: &mut Bencher) {
    let u = b"FF";
    b.iter(|| {
        let u = black_box(u);
        for _ in 0..10_000 {
            let u = parse_v1(&u[..]).unwrap();
            black_box(u);
        }
    });
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
