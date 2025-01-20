use crate::{camera, effects, renderer::polygon::Polygon, Fixed};
use agb::InternalAllocator;
use alloc::vec::Vec;
use camera::*;

use super::{BoundingBox, BoundingCylinder};

pub trait Entity {
    fn render(&mut self, camera: &Camera, page: u16) -> Option<Vec<Polygon, InternalAllocator>>;

    fn set_x_offset(&mut self, x_offset: Fixed);
    fn set_y_offset(&mut self, y_offset: Fixed);
    fn set_z_offset(&mut self, z_offset: Fixed);

    fn set_x_rotation(&mut self, x_rotation: Fixed);
    fn set_y_rotation(&mut self, y_rotation: Fixed);
    fn set_z_rotation(&mut self, z_rotation: Fixed);
    fn reload_rotation_matrices(&mut self);

    fn recalculate_points(&mut self);
    fn refresh_model_matrix(&mut self);

    fn set_size(&mut self, size: Fixed);
    fn set_vertex(&mut self, point: [Fixed; 3], index: i32);
    fn distance_from_camera(&self, camera: &Camera) -> Fixed;
    fn bounding_box(&self) -> BoundingBox;
    fn bounding_cylinder(&self) -> BoundingCylinder;
    fn get_y(&self) -> Fixed;
    fn get_height(&self) -> Fixed;
    fn set_color(&mut self, color: u16);
    fn tick(&mut self, _effects: &effects::InputGameState) -> Option<effects::OutputEvents>;
    fn get_id(&self) -> i16;
    fn set_id(&mut self, id: i16);
}
