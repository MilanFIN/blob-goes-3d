
use crate::fixed;
use fixed::*;

pub struct BoundingRect{
    pub data: [[Fixed; 2]; 4],
    pub z: Fixed,
}

impl BoundingRect {
    pub fn default() -> Self {
        BoundingRect {
            data: [[Fixed::const_new(0); 2]; 4],
            z: Fixed::const_new(0),
        }
	}
    pub fn new(data: [[Fixed; 2]; 4], z: Fixed) -> Self {
        BoundingRect { data, z }
    }
}

