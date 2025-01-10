use crate::{boundingshapes::BoundingBox, Fixed};

pub struct InputPlayerEffects<'a> {
	pub support_below_id: i16,
	pub bounding_box: &'a BoundingBox,
}

pub struct OutputPlayerEffects {
	pub move_x: Fixed,
	pub move_y: Fixed,
	pub move_z: Fixed,
	pub finished: bool,
}