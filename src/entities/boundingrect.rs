
use crate::fixed;
use fixed::*;

pub struct BoundingRect{
    pub data: [[Fixed; 2]; 4],
    pub y: Fixed,
}

impl BoundingRect {
    pub fn default() -> Self {
        BoundingRect {
            data: [[Fixed::const_new(0); 2]; 4],
            y: Fixed::const_new(0),
        }
	}
    #[allow(dead_code)]
    pub fn new(data: [[Fixed; 2]; 4], y: Fixed) -> Self {
        BoundingRect { data, y }
    }
}

