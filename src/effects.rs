use crate::{boundingshapes::BoundingBox, Fixed};

pub struct InputGameState<'a> {
	pub support_below_id: i16,
	pub bounding_box: &'a BoundingBox,
	pub action_requested: bool,
}



pub enum OutputEvents {
	PlayerEvent(MoveXYZ),
	GameFinish(Finished),
	SwitchAction(SwitchFlip),
}

pub struct MoveXYZ {
	pub move_x: Fixed,
	pub move_y: Fixed,
	pub move_z: Fixed,
}

pub struct Finished {
	pub finished: bool,
}

pub struct SwitchFlip {
	pub switch_flip: bool,
}


/*
pub struct OutputEvents {
	pub move_x: Fixed,
	pub move_y: Fixed,
	pub move_z: Fixed,
	pub finished: bool,
	pub switch_flip: bool,
}
*/