use crate::renderer;

use super::chartoindex::convert;
use super::letters::LETTERTILES;

pub fn write_line(
	x: u16,
	y: u16,
	text: &str,
	color: u16,
	page: u16,
) {
	let indices: alloc::vec::Vec<u8> = convert(text);
	for i in 0..indices.len() {
		write_tile(x + i as u16 * 16, y, indices[i] as usize, color, page);
	}
}

pub fn write_tile(
	x: u16,
	y: u16,
	tile: usize,
	color: u16,
	page: u16,
) {
	
	for i in 0..8 {
		let mut mask = 0x80;
		for j in 0..8 {
			if LETTERTILES[tile][i] & mask != 0 {
				renderer::hw::draw_point((x as usize + j) as i32, (y as usize + i) as i32, color, page);
				//renderer::hw::draw_wide_point((x as usize + 2*j) as i32, (y as usize + 2*i+1) as i32, color, page);
			}
			mask >>= 1;
		}
	}
}