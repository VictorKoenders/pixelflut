pub mod bytewise;
#[cfg(feature = "memory-cache")]
pub mod memcache;
pub mod python_generated;
pub mod std;

pub const MAX_VALID_NUMBER: u16 = 1920;

// bytewise is the fastest way to parse coordinates
pub use self::bytewise::parse_coordinate;

// memory-cache is the fastest way to parse color
// bytewise::parse_color_unwrapped is 2nd fastest
cfg_if::cfg_if! {
    if #[cfg(feature = "memory-cache")] {
        pub use self::memcache::parse_color;
    } else {
        pub use self::bytewise::parse_color_unwrapped as parse_color;
    }
}

pub fn initialize() {
    self::bytewise::initialize();
    #[cfg(feature = "memory-cache")]
    self::memcache::initialize_color();
}

#[test]
fn validate_coordinate() {
    #[allow(clippy::type_complexity)]
    let mut cases: Vec<(Vec<u8>, Option<(u16, &'static [u8])>)> = Vec::new();
    for i in 0..=MAX_VALID_NUMBER {
        let str = format!("{} ", i);
        cases.push((str.as_bytes().to_vec(), Some((i, [].as_slice()))));
    }
    cases.push((
        format!("{} ", MAX_VALID_NUMBER + 1).as_bytes().to_vec(),
        None,
    ));

    for (input, expected) in cases {
        assert_eq!(python_generated::parse_coordinate(&input), expected);
        assert_eq!(std::parse_coordinate(&input), expected);
        assert_eq!(bytewise::parse_coordinate(&input), expected);
    }
}

#[test]
fn validate_hex() {
    use rand::{thread_rng, Rng};

    // Testing all cases takes too long (256*256*256*2 = 33.5 mil cases)
    // So we randomly pick 10k of them

    #[allow(clippy::type_complexity)]
    let mut cases: Vec<(Vec<u8>, Option<(u8, u8, u8)>)> = Vec::new();
    let mut rng = thread_rng();
    for _ in 0..10_000 {
        let mut gen_color = || {
            let val: u8 = rng.gen();
            (
                val,
                if rng.gen::<bool>() {
                    format!("{:02x}", val)
                } else {
                    format!("{:02X}", val)
                },
            )
        };

        let (r, r_str) = gen_color();
        let (g, g_str) = gen_color();
        let (b, b_str) = gen_color();

        cases.push((
            format!("{}{}{}", r_str, g_str, b_str).as_bytes().to_vec(),
            Some((r, g, b)),
        ))
    }

    for (input, expected) in cases {
        let py = python_generated::parse_color(&input);
        assert_eq!(
            py, expected,
            "Python could not parse {:?}, expected {:?}, got {:?}",
            input, expected, py
        );
        let std = std::parse_color(&input);
        assert_eq!(
            std, expected,
            "STD could not parse {:?}, expected {:?}, got {:?}",
            input, expected, std
        );
        let bytewise = bytewise::parse_color(&input);
        assert_eq!(
            bytewise, expected,
            "bytewise could not parse {:?}, expected {:?}, got {:?}",
            input, expected, bytewise
        );
        let bytewise_unwrapped = bytewise::parse_color_unwrapped(&input);
        assert_eq!(
            bytewise_unwrapped,
            expected,
            "bytewise_unwrapped could not parse {}, expected {:?}, got {:?}",
            ::std::str::from_utf8(&input).unwrap(),
            expected,
            bytewise_unwrapped
        );
    }
}

#[test]
fn crashes() {
    // crashes found with running the following commint in the ./fuzz/ folder:
    // `cargo afl fuzz -i in -o out target/debug/pixelflut_fuzzer`
    let cases = [
        "mmmmm ".as_bytes(),
        "10 \0".as_bytes(),
        " \\".as_bytes(),
        " > \n".as_bytes(),
        " ".as_bytes(),
        "00000000000000000000000000000000000000010\n".as_bytes(),
        "90\n\0".as_bytes(),
        &[0x30, 0x30, 0x30, 0x30, 0xdf, 0x20],
        "+00 ".as_bytes(),
    ];

    #[cfg(feature = "memory-cache")]
    let memcache = {
        let mut cache = memcache::NumCache::new();
        cache.init();
        cache
    };

    for case in cases {
        bytewise::parse_coordinate(case);
        std::parse_coordinate(case);

        #[cfg(feature = "memory-cache")]
        memcache.parse_coordinate(case);
    }
}
