use agb::fixnum::Num;

pub mod entity;
use entity::*;

pub mod cube;
use cube::*;

pub mod empty;
use empty::*;

use super::math;
use math::*;

use super::render;
use render::*;

use super::camera;
use camera::*;



#[derive(Copy, Clone)]
pub enum EntityEnum {
    Cube(Cube),
    Empty(Empty),
}

impl EntityEnum {
    pub fn set_x_offset(&mut self, offset: Num<i32, 8>) {
        match self {
            EntityEnum::Cube(c) => c.set_x_offset(offset),
            EntityEnum::Empty(e) => e.set_y_offset(offset),
        }
    }
    pub fn set_y_offset(&mut self, offset: Num<i32, 8>) {
        match self {
            EntityEnum::Cube(c) => c.set_x_offset(offset),
            EntityEnum::Empty(e) => e.set_y_offset(offset),
        }
    }
    pub fn set_z_offset(&mut self, offset: Num<i32, 8>) {
        match self {
            EntityEnum::Cube(c) => c.set_z_offset(offset),
            EntityEnum::Empty(e) => e.set_z_offset(offset),
        }
    }
    pub fn set_x_rotation(&mut self, rot: Num<i32, 8>) {
        match self {
            EntityEnum::Cube(c) => c.set_x_rotation(rot),
            EntityEnum::Empty(e) => e.set_x_rotation(rot),
        }
    }
    pub fn set_y_rotation(&mut self, rot: Num<i32, 8>) {
        match self {
            EntityEnum::Cube(c) => c.set_y_rotation(rot),
            EntityEnum::Empty(e) => e.set_y_rotation(rot),
        }
    }
    pub fn set_z_rotation(&mut self, rot: Num<i32, 8>) {
        match self {
            EntityEnum::Cube(c) => c.set_z_rotation(rot),
            EntityEnum::Empty(e) => e.set_z_rotation(rot),
        }
    }
    pub fn refresh_model_matrix(&mut self) {
        match self {
            EntityEnum::Cube(c) => c.refresh_model_matrix(),
            EntityEnum::Empty(e) => e.refresh_model_matrix(),
        }
    }
    pub fn set_size(&mut self, size: i32) {
        match self {
            EntityEnum::Cube(c) => c.set_size(size),
            EntityEnum::Empty(e) => e.set_size(size),
        }
    }
    pub fn render(&self, bitmap4: &mut agb::display::bitmap4::Bitmap4, camera: &Camera) {
        match self {
            EntityEnum::Cube(c) => c.render(bitmap4, camera),
            EntityEnum::Empty(e) => e.render(bitmap4, camera),
        }
    }
}
