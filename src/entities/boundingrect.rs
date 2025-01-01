
use crate::fixed;
use fixed::*;

pub struct BoundingBox{
    pub data: [[Fixed; 2]; 4],
    pub yTop: Fixed,
    pub yBottom: Fixed,

}

impl BoundingBox {
    pub fn default() -> Self {
        BoundingBox {
            data: [[Fixed::const_new(0); 2]; 4],
            yTop: Fixed::const_new(0),
            yBottom: Fixed::const_new(0),
        }
	}
    #[allow(dead_code)]
    pub fn new(data: [[Fixed; 2]; 4], yTop: Fixed, yBottom: Fixed) -> Self {
        BoundingBox { data, yTop, yBottom }
    }
    pub fn new_with_offset(old_box: &BoundingBox, x_offset: Fixed, z_offset: Fixed) -> Self {
        let new_data = old_box
            .data
            .map(|[x, z]| [x + x_offset, z + z_offset]);

        BoundingBox {
            data: new_data,
            yTop: old_box.yTop,
            yBottom: old_box.yBottom,
        }
    }
}

