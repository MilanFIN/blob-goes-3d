use crate::{camera, Fixed};
use camera::*;

use super::{BoundingBox, BoundingCylinder};

pub trait Entity {
    fn render(&mut self, bitmap4: &mut agb::display::bitmap4::Bitmap4, camera: &Camera);

    fn set_x_offset(&mut self, x_offset: Fixed);
    fn set_y_offset(&mut self, y_offset: Fixed);
    fn set_z_offset(&mut self, z_offset: Fixed);

    fn set_x_rotation(&mut self, x_rotation: Fixed);
    fn set_y_rotation(&mut self, y_rotation: Fixed);
    fn set_z_rotation(&mut self, z_rotation: Fixed);
    fn recalculate_points(&mut self);
    fn refresh_model_matrix(&mut self);

    fn set_size(&mut self, size: Fixed);
    fn set_vertex(&mut self, point: [Fixed; 3], index: i32);
    fn distance_from_camera(&self, camera: &Camera) -> Fixed;
    fn bounding_box(&self) -> BoundingBox;
    fn bounding_cylinder(&self) -> BoundingCylinder;

}
