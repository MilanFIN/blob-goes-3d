
extern crate alloc;
use alloc::vec::Vec;

pub fn convert_to_tiles(s: &str) -> Vec<u8> {
	let indices: Vec<u8> = s.chars().map(|c| match c.to_lowercase().next().unwrap() {
		'0'..='9' => (c as u8) - b'0',          // '0' -> 0, '1' -> 1, ..., '9' -> 9
		'a'..='z' => (c as u8) - b'a' + 10,     // 'a' -> 10, 'b' -> 11, ..., 'z' -> 35
		'-' => 38,
		_ => 36,                                // Any other character -> 36
	}).collect();
	return indices;
}
