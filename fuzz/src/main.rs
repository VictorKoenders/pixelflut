use pixelflut::parse::*;

fn main() {
    afl::fuzz!(|data: &[u8]| {
        let result = bytewise::parse_coordinate(data);
        assert_eq!(result, std::parse_coordinate(data));
    });
}
