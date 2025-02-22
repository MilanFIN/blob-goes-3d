use crate::{boundingshapes::{BoundingBox, BoundingCylinder}, Fixed};

pub struct InputGameState<'a> {
	pub support_below_id: i16,
	pub bounding_box: &'a BoundingBox,
	pub bounding_cylinder: &'a BoundingCylinder,
	pub player_distance_from_ground: Fixed,
	pub action_requested: bool,
	pub yspeed: Fixed,
}



pub enum OutputEvents {
	PlayerEvent(MoveXYZ),
	GameFinish(Finished),
	SwitchAction(SwitchFlip),
	BounceEvent(Bounce),
	Sliding(Sliding),
}

pub struct MoveXYZ {
	pub move_x: Fixed,
	pub move_y: Fixed,
	pub move_z: Fixed,
}

pub struct Finished {
	//pub finished: bool,
}

pub struct SwitchFlip {
	//pub switch_flip: bool,
}

pub struct Bounce {
	pub power: Fixed,
}

pub struct Sliding {
	pub acceleration: Fixed,
}