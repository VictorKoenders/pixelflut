print("// Auto generated with gen_basic.py")
print("// See that file for more info")
print("")
print("use std::convert::TryInto;")
print("")
print("pub fn parse_coordinate(buffer: &[u8]) -> Option<(u16, &[u8])> {")
print("    match (buffer.get(0), buffer.get(1), buffer.get(2), buffer.get(3), buffer.get(4)) {")
for i in range(0, 1800):
	nums = str(i)
	first = "None"
	second = "_"
	third = "_"
	fourth = "_"
	fifth = "_"
	if len(nums) > 0:
		first = "Some(b'" + nums[0] + "')"
	if len(nums) > 1:
		second = "Some(b'" + nums[1] + "')"
	if len(nums) > 2:
		third = "Some(b'" + nums[2] + "')"
	if len(nums) > 3:
		fourth = "Some(b'" + nums[3] + "')"
	
	if len(nums) == 1:
		second = "Some(b' ')"
	elif len(nums) == 2:
		third = "Some(b' ')"
	elif len(nums) == 3:
		fourth = "Some(b' ')"
	elif len(nums) == 4:
		fifth = "Some(b' ')"
	
	print("        (" + first + ", " + second + ", " + third + ", " + fourth + ", " + fifth + ") => Some((" + nums + ", unsafe { buffer.get_unchecked(" + str(len(nums) + 1) + "..) })),")

print("        _ => None,")
print("    }")
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
