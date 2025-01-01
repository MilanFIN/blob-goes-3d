
use crate::fixed;
use fixed::*;

pub struct BoundingBox{
    pub data: [[Fixed; 2]; 4],
    pub y_top: Fixed,
    pub y_bottom: Fixed,

}

impl BoundingBox {
    pub fn default() -> Self {
        BoundingBox {
            data: [[Fixed::const_new(0); 2]; 4],
            y_top: Fixed::const_new(0),
            y_bottom: Fixed::const_new(0),
        }
	}
    #[allow(dead_code)]
    pub fn new(data: [[Fixed; 2]; 4], y_top: Fixed, y_bottom: Fixed) -> Self {
        BoundingBox { data, y_top, y_bottom }
    }
    pub fn new_with_offset(old_box: &BoundingBox, x_offset: Fixed, z_offset: Fixed) -> Self {
        let new_data = old_box
            .data
            .map(|[x, z]| [x + x_offset, z + z_offset]);

        BoundingBox {
            data: new_data,
            y_top: old_box.y_top,
            y_bottom: old_box.y_bottom,
        }
    }
}

