use crate::fixed::Fixed;

pub struct Polygon {
	pub distance_from_camera: Fixed,
	pub shape: Shape,
	pub color: u16,
    pub draw_always: bool,
}

impl Polygon {
    pub fn as_triangle(&self) -> Option<[[Fixed; 2]; 3]> {
        if let Shape::Triangle(vertices) = self.shape {
            Some(vertices)
        } else {
            None
        }
    }
    pub fn as_line(&self) -> Option<[[Fixed; 2]; 2]> {
        if let Shape::Line(vertices) = self.shape {
            Some(vertices)
        } else {
            None
        }
    }
}

pub enum Shape {
	Triangle([[Fixed; 2]; 3]),
	Line([[Fixed; 2]; 2]),
}