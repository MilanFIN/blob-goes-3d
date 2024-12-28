pub mod entity;
use entity::*;

pub mod cube;
use cube::*;

pub mod rectangle;
use rectangle::*;

pub mod empty;
use empty::*;

pub mod boundingrect;
use boundingrect::*;

use serde::Deserialize;

use super::math;
use super::render;

use super::camera;
use camera::*;

use crate::fixed;
use fixed::*;


#[derive(Copy, Clone, Deserialize, Debug)]
#[serde(tag = "type", content = "data")]
pub enum EntityEnum {
    #[serde(rename = "cube")]
    Cube(Cube),
    #[serde(rename = "rectangle")]
    Rectangle(Rectangle),
    #[serde(rename = "empty")]
    Empty(Empty),
}

impl EntityEnum {
    pub fn set_x_offset(&mut self, offset: Fixed) {
        match self {
            EntityEnum::Cube(c) => c.set_x_offset(offset),
            EntityEnum::Rectangle(r) => r.set_x_offset(offset),
            EntityEnum::Empty(_e) => {},
        }
    }
    pub fn set_y_offset(&mut self, offset: Fixed) {
        match self {
            EntityEnum::Cube(c) => c.set_y_offset(offset),
            EntityEnum::Rectangle(r) => r.set_y_offset(offset),
            EntityEnum::Empty(_e) => {},
        }
    }
    pub fn set_z_offset(&mut self, offset: Fixed) {
        match self {
            EntityEnum::Cube(c) => c.set_z_offset(offset),
            EntityEnum::Rectangle(r) => r.set_z_offset(offset),
            EntityEnum::Empty(_e) => {},
        }
    }
    pub fn set_x_rotation(&mut self, rot: Fixed) {
        match self {
            EntityEnum::Cube(c) => c.set_x_rotation(rot),
            EntityEnum::Rectangle(r) => r.set_x_rotation(rot),
            EntityEnum::Empty(_e) => {},
        }
    }
    pub fn set_y_rotation(&mut self, rot: Fixed) {
        match self {
            EntityEnum::Cube(c) => c.set_y_rotation(rot),
            EntityEnum::Rectangle(r) => r.set_y_rotation(rot),
            EntityEnum::Empty(_e) => {},
        }
    }
    pub fn set_z_rotation(&mut self, rot: Fixed) {
        match self {
            EntityEnum::Cube(c) => c.set_z_rotation(rot),
            EntityEnum::Rectangle(r) => r.set_z_rotation(rot),
            EntityEnum::Empty(_e) => {},
        }
    }
    pub fn refresh_model_matrix(&mut self) {
        match self {
            EntityEnum::Cube(c) => c.refresh_model_matrix(),
            EntityEnum::Rectangle(r) => r.refresh_model_matrix(),
            EntityEnum::Empty(_e) => {},
        }
    }
    //todo: rename to set_scale at some point to recalculate points at a different scale from original size
    pub fn set_size(&mut self, size: Fixed) {
        match self {
            EntityEnum::Cube(c) => c.set_size(size),
            //not implemented
            EntityEnum::Rectangle(_r) => {},
            EntityEnum::Empty(_e) => {},
        }
    }
    pub fn recalculate_points(&mut self) {
        match self {
            EntityEnum::Cube(c) => c.recalculate_points(),
            EntityEnum::Rectangle(r) => r.recalculate_points(),
            EntityEnum::Empty(_e) => {},
        }
    }
    #[allow(dead_code)]
    pub fn set_vertex(&mut self, point: [Fixed; 3], index: i32) {
        match self {
            EntityEnum::Cube(c) => c.set_vertex(point, index),
            EntityEnum::Rectangle(r) => r.set_vertex(point, index),
            EntityEnum::Empty(_e) => {},
        }
    }
    pub fn render(&mut self, bitmap4: &mut agb::display::bitmap4::Bitmap4, camera: &Camera) {
        match self {
            EntityEnum::Cube(c) => c.render(bitmap4, camera),
            EntityEnum::Rectangle(r) => r.render(bitmap4, camera),
            EntityEnum::Empty(_e) => {},
        }
    }
    pub fn distance_from_camera(&self, camera: &Camera) -> Fixed {
        match self {
            EntityEnum::Cube(c) => c.distance_from_camera(camera),
            EntityEnum::Rectangle(r) => r.distance_from_camera(camera),
            EntityEnum::Empty(_e) => {Fixed::const_new(999)},
        }
    }
    pub fn bottom_bounding_rect(&self) -> BoundingRect {
        match self {
            EntityEnum::Cube(c) => c.bottom_bounding_rect(),
            EntityEnum::Rectangle(r) => r.bottom_bounding_rect(),
            EntityEnum::Empty(_e) => {BoundingRect::default()},
        }
    }
    pub fn peak_rect_overlap(&self, rect: &BoundingRect) -> Fixed {
        match self {
            EntityEnum::Cube(c) => c.peak_rect_overlap(rect),
            EntityEnum::Rectangle(r) => r.peak_rect_overlap(rect),
            EntityEnum::Empty(_e) => {Fixed::const_new(0)},
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

//determine if the element in the entiry array has an object below it
pub fn check_support_below(entity_array: &[EntityEnum], element: usize) -> Fixed{
    let rect: BoundingRect = entity_array[element].bottom_bounding_rect();
    let mut distance = Fixed::const_new(999);
    for (i, e) in entity_array.iter().enumerate() {
        if i != 0 && i != 1 {
            let d: Fixed = e.peak_rect_overlap(&rect);
            if d < distance {
                distance = d;
            }
        }
    }
    //distance = Fixed::const_new(10);
    return distance;
}