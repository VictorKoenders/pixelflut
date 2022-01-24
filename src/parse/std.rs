pub fn parse_coordinate(buffer: &[u8]) -> Option<(u16, &[u8])> {
    let idx = buffer.iter().position(|b| *b == b' ')?;
    let num = unsafe { buffer.get_unchecked(..idx) };
    let remaining = unsafe { buffer.get_unchecked(idx + 1..) };
    let x: u16 = std::str::from_utf8(num).ok()?.parse().ok()?;
    if x > super::MAX_VALID_NUMBER {
        return None;
    }
    Some((x, remaining))
}

pub fn parse_color(buffer: &[u8]) -> Option<(u8, u8, u8)> {
    let mut parts = buffer.get(0..6)?.chunks(2);
    let r = u8::from_str_radix(std::str::from_utf8(parts.next()?).ok()?, 16).ok()?;
    let g = u8::from_str_radix(std::str::from_utf8(parts.next()?).ok()?, 16).ok()?;
    let b = u8::from_str_radix(std::str::from_utf8(parts.next()?).ok()?, 16).ok()?;
    Some((r, g, b))
}

pub fn initialize() {
    // do nothing
}
