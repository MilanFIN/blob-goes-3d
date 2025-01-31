use crate::fixed;
use fixed::*;

pub enum BoundingShape {
	BoundingBox(BoundingBox),
	BoundingCylinder(BoundingCylinder),

}

#[derive(Clone)]
pub struct BoundingBox {
    pub data: [[Fixed; 2]; 4],
    //[x, z]
    pub center: [Fixed; 2],
    pub width: Fixed,
    pub height: Fixed,
    pub y_top: Fixed,
    pub y_bottom: Fixed,
    pub rotation: Fixed,
}

impl BoundingBox {
    pub fn default() -> Self {
        BoundingBox {
            data: [[Fixed::const_new(0); 2]; 4],
            center: [Fixed::const_new(0); 2],
            height: Fixed::const_new(0),
            width: Fixed::const_new(0),
            y_top: Fixed::const_new(0),
            y_bottom: Fixed::const_new(0),
            rotation: Fixed::const_new(0),
        }
    }
    #[allow(dead_code)]
    pub fn new(
        data: [[Fixed; 2]; 4],
        center: [Fixed; 2],
        width: Fixed,
        height: Fixed,
        y_top: Fixed,
        y_bottom: Fixed,
        rotation: Fixed,
    ) -> Self {
        BoundingBox {
            data,
            center,
            width,
            height,
            y_top,
            y_bottom,
            rotation,
        }
    }
    #[allow(dead_code)]
    pub fn new_with_offset(old_box: &BoundingBox, x_offset: Fixed, z_offset: Fixed) -> Self {
        let new_data = old_box.data.map(|[x, z]| [x + x_offset, z + z_offset]);

        BoundingBox {
            data: new_data,
            center: [old_box.center[0] + x_offset, old_box.center[1] + z_offset],
            width: old_box.width,
            height: old_box.height,
            y_top: old_box.y_top,
            y_bottom: old_box.y_bottom,
            rotation: old_box.rotation,
        }
    }

    pub fn empty() -> BoundingBox {
        BoundingBox {
            data: [[Fixed::const_new(0); 2]; 4],
            center: [Fixed::const_new(0); 2],
            width: Fixed::const_new(0),
            height: Fixed::const_new(0),
            y_top: Fixed::const_new(-999),
            y_bottom: Fixed::const_new(-999),
            rotation: Fixed::const_new(0),
        }
    }
}

pub struct BoundingCylinder {
    pub x: Fixed,
    pub z: Fixed,
    pub radius: Fixed,
    pub y_top: Fixed,
    pub y_bottom: Fixed,
}

impl BoundingCylinder {
    pub fn default() -> Self {
        BoundingCylinder {
            x: Fixed::const_new(0),
            z: Fixed::const_new(0),
            radius: Fixed::const_new(0),
            y_top: Fixed::const_new(0),
            y_bottom: Fixed::const_new(0),
        }
    }

    pub fn new_with_offset(old: &BoundingCylinder, x_offset: Fixed, z_offset: Fixed) -> Self {
        BoundingCylinder {
            x: old.x + x_offset,
            z: old.z + z_offset,
            radius: old.radius,
            y_top: old.y_top,
            y_bottom: old.y_bottom,
        }
    }

    pub fn empty() -> Self {
        BoundingCylinder {
            x: Fixed::const_new(0),
            z: Fixed::const_new(0),
            radius: Fixed::const_new(0),
            y_top: Fixed::const_new(-999),
            y_bottom: Fixed::const_new(-999),
        }
    }
}
