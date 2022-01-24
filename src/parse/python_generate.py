# Run with:
# `python python_generate.py > python_generated.rs`

print("// Auto generated with python_generate.py")
print("// See that file for more info")
print("")
print("use std::convert::TryInto;")
print("")
print("pub fn initialize() {")
print("    // nothing to be done")
print("}")
print("")
print("pub fn parse_coordinate(buffer: &[u8]) -> Option<(u16, &[u8])> {")
print("    if let Some(b' ') = buffer.get(4) {")
print("        let slice: &[u8; 4] = unsafe { buffer.get_unchecked(..4).try_into().unwrap_unchecked() };")
print("        let remaining = unsafe { buffer.get_unchecked(5..) };")
print("        match slice {")
for i in range(1000, 1921):
	print("            b\"" + str(i) + "\" => return Some((" + str(i) + ", remaining)),")
print("            _ => {},")
print("        }")
print("    }")
print("    if let Some(b' ') = buffer.get(3) {")
print("        let slice: &[u8; 3] = unsafe { buffer.get_unchecked(..3).try_into().unwrap_unchecked() };")
print("        let remaining = unsafe { buffer.get_unchecked(4..) };")
print("        match slice {")
for i in range(100, 1000):
	print("            b\"" + str(i) + "\" => return Some((" + str(i) + ", remaining)),")
print("            _ => {},")
print("        }")
print("    }")
print("    if let Some(b' ') = buffer.get(2) {")
print("        let slice: &[u8; 2] = unsafe { buffer.get_unchecked(..2).try_into().unwrap_unchecked() };")
print("        let remaining = unsafe { buffer.get_unchecked(3..) };")
print("        match slice {")
for i in range(10, 100):
	print("            b\"" + str(i) + "\" => return Some((" + str(i) + ", remaining)),")
print("            _ => {},")
print("        }")
print("    }")
print("    if let Some(b' ') = buffer.get(1) {")
print("        let slice: &[u8; 1] = unsafe { buffer.get_unchecked(..1).try_into().unwrap_unchecked() };")
print("        let remaining = unsafe { buffer.get_unchecked(2..) };")
print("        match slice {")
for i in range(0, 10):
	print("            b\"" + str(i) + "\" => return Some((" + str(i) + ", remaining)),")
print("            _ => {},")

print("        }")
print("    }")
print("    None")
print("}")
print("")
print("pub fn parse_color(buffer: &[u8]) -> Option<(u8, u8, u8)> {")
print("    let buffer = buffer.get(0..6)?;")
print("    let r: [u8; 2] = unsafe { buffer.get_unchecked(0..2).try_into().unwrap_unchecked() };")
print("    let g: [u8; 2] = unsafe { buffer.get_unchecked(2..4).try_into().unwrap_unchecked() };")
print("    let b: [u8; 2] = unsafe { buffer.get_unchecked(4..6).try_into().unwrap_unchecked() };")
print("")
print("    #[inline]")
print("    fn parse_hex(val: [u8; 2]) -> Option<u8> {")
print("        match &val {")

for i in range(0, 0x100):
	lowercase_hex = hex(i)[2:].zfill(2)
	case = ""
	if lowercase_hex != lowercase_hex.upper():
		case = "b\"" + lowercase_hex + "\" | b\"" + lowercase_hex.upper() + "\""
	else:
		case = "b\"" + lowercase_hex + "\""
	print("            " + case + " => Some(" + str(i) + "),")

print("            _ => None,")
print("        }")
print("    }")
print("    let r = parse_hex(r)?;")
print("    let g = parse_hex(g)?;")
print("    let b = parse_hex(b)?;")
print("    Some((r, g, b))")
print("}")
