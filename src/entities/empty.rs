use agb::fixnum::Num;

use super::{entity, new_num};
use entity::*;

use super::Camera;

#[derive(Copy, Clone)]
pub struct Empty {}
impl Entity for Empty {
    fn render(&self, _bitmap4: &mut agb::display::bitmap4::Bitmap4, _camera: &Camera) {}
    fn set_x_offset(&mut self, _x_offset: Num<i32, 8>) {}
    fn set_y_offset(&mut self, _y_offset: Num<i32, 8>) {}
    fn set_z_offset(&mut self, _z_offset: Num<i32, 8>) {}
    fn set_size(&mut self, _size: Num<i32, 8>) {}
    fn set_x_rotation(&mut self, _x_rotation: Num<i32, 8>) {}
    fn set_y_rotation(&mut self, _y_rotation: Num<i32, 8>) {}
    fn set_z_rotation(&mut self, _z_rotation: Num<i32, 8>) {}
    fn refresh_model_matrix(&mut self) {}
    fn set_vertex(&mut self, _point: [Num<i32, 8>; 3], _index: i32) {}
    fn distance_from_camera(&self, _camera: &Camera) -> Num<i32, 8> {
        return new_num(999);
    }
}
impl Empty {
    pub fn default() -> Self {
        Self {}
    }
}
