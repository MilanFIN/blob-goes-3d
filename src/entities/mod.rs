
pub mod entity;
use entity::*;

pub mod cube;
use cube::*;

pub mod empty;
use empty::*;
use serde::Deserialize;


use super::math;
use super::render;

use super::camera;
use camera::*;

use crate::fixed;
use fixed::*;



#[derive(Copy, Clone, Deserialize,Debug)]
#[serde(tag = "type", content = "data")]
pub enum EntityEnum {
    #[serde(rename = "cube")]
    Cube(Cube),
    #[serde(rename = "empty")]
    Empty(Empty),
}

impl EntityEnum {
    pub fn set_x_offset(&mut self, offset: Fixed) {
        match self {
            EntityEnum::Cube(c) => c.set_x_offset(offset),
            EntityEnum::Empty(e) => e.set_x_offset(offset),
        }
    }
    pub fn set_y_offset(&mut self, offset: Fixed) {
        match self {
            EntityEnum::Cube(c) => c.set_y_offset(offset),
            EntityEnum::Empty(e) => e.set_y_offset(offset),
        }
    }
    pub fn set_z_offset(&mut self, offset: Fixed) {
        match self {
            EntityEnum::Cube(c) => c.set_z_offset(offset),
            EntityEnum::Empty(e) => e.set_z_offset(offset),
        }
    }
    pub fn set_x_rotation(&mut self, rot: Fixed) {
        match self {
            EntityEnum::Cube(c) => c.set_x_rotation(rot),
            EntityEnum::Empty(e) => e.set_x_rotation(rot),
        }
    }
    pub fn set_y_rotation(&mut self, rot: Fixed) {
        match self {
            EntityEnum::Cube(c) => c.set_y_rotation(rot),
            EntityEnum::Empty(e) => e.set_y_rotation(rot),
        }
    }
    pub fn set_z_rotation(&mut self, rot: Fixed) {
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
    pub fn set_size(&mut self, size: Fixed) {
        match self {
            EntityEnum::Cube(c) => c.set_size(size),
            EntityEnum::Empty(e) => e.set_size(size),
        }
    }
    pub fn recalculate_points(&mut self) {
        match self {
            EntityEnum::Cube(c) => c.recalculate_points(),
            EntityEnum::Empty(e) => e.recalculate_points(),
        }
    }
    #[allow(dead_code)]
    pub fn set_vertex(&mut self, point: [Fixed; 3], index: i32) {
        match self {
            EntityEnum::Cube(c) => c.set_vertex(point, index),
            EntityEnum::Empty(e) => e.set_vertex(point, index),
        }
    }
    pub fn render(&self, bitmap4: &mut agb::display::bitmap4::Bitmap4, camera: &Camera) {
        match self {
            EntityEnum::Cube(c) => c.render(bitmap4, camera),
            EntityEnum::Empty(e) => e.render(bitmap4, camera),
        }
    }
    pub fn distance_from_camera(&self, camera: &Camera) -> Fixed {
        match self {
            EntityEnum::Cube(c) => c.distance_from_camera(camera),
            EntityEnum::Empty(e) => e.distance_from_camera(camera),
        }
    }
}

fn partition(
    entity_render_order: &mut [usize],
    entity_array: &[EntityEnum],
    low: usize,
    high: usize,
    camera: &Camera,
) -> usize {
    let pivot_distance = entity_array[entity_render_order[high]].distance_from_camera(camera);
    let mut i = low as isize - 1; // Use `isize` to allow `-1` for initialization

    for j in low..high {
        if entity_array[entity_render_order[j]].distance_from_camera(camera) >= pivot_distance {
            i += 1;
            entity_render_order.swap(i as usize, j);
        }
    }

    entity_render_order.swap((i + 1) as usize, high);
    (i + 1) as usize
}

pub fn quick_sort(
    entity_render_order: &mut [usize],
    entity_array: &[EntityEnum],
    low: usize,
    high: usize,
    camera: &Camera,
) {
    if low < high {
        let pi = partition(entity_render_order, entity_array, low, high, camera);

        if pi > 0 {
            quick_sort(entity_render_order, entity_array, low, pi - 1, camera);
        }
        quick_sort(entity_render_order, entity_array, pi + 1, high, camera);
    }
}

