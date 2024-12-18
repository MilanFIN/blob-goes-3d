use agb::fixnum::Num;
use crate::camera;
use camera::*;

pub trait Entity {
    fn render(&self, bitmap4: &mut agb::display::bitmap4::Bitmap4, camera: &Camera);

    fn set_x_offset(&mut self, x_offset: Num<i32, 8>);
    fn set_y_offset(&mut self, y_offset: Num<i32, 8>);
    fn set_z_offset(&mut self, z_offset: Num<i32, 8>);

    fn set_x_rotation(&mut self, x_rotation: Num<i32, 8>);
    fn set_y_rotation(&mut self, y_rotation: Num<i32, 8>);
    fn set_z_rotation(&mut self, z_rotation: Num<i32, 8>);
    fn refresh_model_matrix(&mut self);

    fn set_size(&mut self, size: Num<i32, 8>);
    fn set_vertex(&mut self, point: [Num<i32, 8>; 3], index: i32);
}
