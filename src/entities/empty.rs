use serde::Deserialize;

use super::entity;
use entity::*;

use super::Camera;

use crate::fixed;
use fixed::*;

#[derive(Copy, Clone, Deserialize, Debug)]
pub struct Empty {}
impl Entity for Empty {
    fn render(&self, _bitmap4: &mut agb::display::bitmap4::Bitmap4, _camera: &Camera) {}
    fn set_x_offset(&mut self, _x_offset: Fixed) {}
    fn set_y_offset(&mut self, _y_offset: Fixed) {}
    fn set_z_offset(&mut self, _z_offset: Fixed) {}
    fn set_size(&mut self, _size: Fixed) {}
    fn recalculate_points(&mut self) {}
    fn set_x_rotation(&mut self, _x_rotation: Fixed) {}
    fn set_y_rotation(&mut self, _y_rotation: Fixed) {}
    fn set_z_rotation(&mut self, _z_rotation: Fixed) {}
    fn refresh_model_matrix(&mut self) {}
    fn set_vertex(&mut self, _point: [Fixed; 3], _index: i32) {}
    fn distance_from_camera(&self, _camera: &Camera) -> Fixed {
        return Fixed::const_new(999);
    }

}

impl Empty {
    pub fn default() -> Self {
        Self {}
    }
}
