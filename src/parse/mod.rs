pub mod bytewise;
pub mod python_generated;
pub mod std;

pub const MAX_VALID_NUMBER: u16 = 1920;

pub use self::bytewise::*;

#[test]
fn validate() {
    for i in 0..=MAX_VALID_NUMBER {
        let str = format!("{} ", i);
        assert_eq!(
            python_generated::parse_coordinate(str.as_bytes()),
            Some((i, [].as_slice()))
        );
        assert_eq!(
            std::parse_coordinate(str.as_bytes()),
            Some((i, [].as_slice()))
        );
        assert_eq!(
            bytewise::parse_coordinate(str.as_bytes()),
            Some((i, [].as_slice()))
        );
    }

    let i = MAX_VALID_NUMBER + 1;
    let str = format!("{} ", i);
    assert_eq!(python_generated::parse_coordinate(str.as_bytes()), None);
    assert_eq!(std::parse_coordinate(str.as_bytes()), None);
    assert_eq!(bytewise::parse_coordinate(str.as_bytes()), None);
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

    for case in cases {
        bytewise::parse_coordinate(case);
        std::parse_coordinate(case);
    }
}
