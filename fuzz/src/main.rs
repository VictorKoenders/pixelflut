use pixelflut::parse::*;

fn main() {
    afl::fuzz!(|data: &[u8]| {
        bytewise::parse_coordinate(data);
        std::parse_coordinate(data);
    });
}
