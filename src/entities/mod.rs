pub mod entity;
use entity::*;

pub mod cube;
use cube::*;

pub mod rectangle;
use rectangle::*;

pub mod empty;
use empty::*;

pub mod boundingshapes;
use boundingshapes::*;

use serde::Deserialize;

use super::math;
use super::render;

use super::camera;
use camera::*;

use crate::fixed;
use crate::math::cross_product;
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
            EntityEnum::Empty(_e) => {}
        }
    }
    pub fn set_y_offset(&mut self, offset: Fixed) {
        match self {
            EntityEnum::Cube(c) => c.set_y_offset(offset),
            EntityEnum::Rectangle(r) => r.set_y_offset(offset),
            EntityEnum::Empty(_e) => {}
        }
    }
    pub fn set_z_offset(&mut self, offset: Fixed) {
        match self {
            EntityEnum::Cube(c) => c.set_z_offset(offset),
            EntityEnum::Rectangle(r) => r.set_z_offset(offset),
            EntityEnum::Empty(_e) => {}
        }
    }
    pub fn set_x_rotation(&mut self, rot: Fixed) {
        match self {
            EntityEnum::Cube(c) => c.set_x_rotation(rot),
            EntityEnum::Rectangle(r) => r.set_x_rotation(rot),
            EntityEnum::Empty(_e) => {}
        }
    }
    pub fn set_y_rotation(&mut self, rot: Fixed) {
        match self {
            EntityEnum::Cube(c) => c.set_y_rotation(rot),
            EntityEnum::Rectangle(r) => r.set_y_rotation(rot),
            EntityEnum::Empty(_e) => {}
        }
    }
    pub fn set_z_rotation(&mut self, rot: Fixed) {
        match self {
            EntityEnum::Cube(c) => c.set_z_rotation(rot),
            EntityEnum::Rectangle(r) => r.set_z_rotation(rot),
            EntityEnum::Empty(_e) => {}
        }
    }
    pub fn refresh_model_matrix(&mut self) {
        match self {
            EntityEnum::Cube(c) => c.refresh_model_matrix(),
            EntityEnum::Rectangle(r) => r.refresh_model_matrix(),
            EntityEnum::Empty(_e) => {}
        }
    }
    //todo: rename to set_scale at some point to recalculate points at a different scale from original size
    pub fn set_size(&mut self, size: Fixed) {
        match self {
            EntityEnum::Cube(c) => c.set_size(size),
            //not implemented
            EntityEnum::Rectangle(_r) => {}
            EntityEnum::Empty(_e) => {}
        }
    }
    pub fn recalculate_points(&mut self) {
        match self {
            EntityEnum::Cube(c) => c.recalculate_points(),
            EntityEnum::Rectangle(r) => r.recalculate_points(),
            EntityEnum::Empty(_e) => {}
        }
    }
    #[allow(dead_code)]
    pub fn set_vertex(&mut self, point: [Fixed; 3], index: i32) {
        match self {
            EntityEnum::Cube(c) => c.set_vertex(point, index),
            EntityEnum::Rectangle(r) => r.set_vertex(point, index),
            EntityEnum::Empty(_e) => {}
        }
    }
    pub fn render(&mut self, bitmap4: &mut agb::display::bitmap4::Bitmap4, camera: &Camera) {
        match self {
            EntityEnum::Cube(c) => c.render(bitmap4, camera),
            EntityEnum::Rectangle(r) => r.render(bitmap4, camera),
            EntityEnum::Empty(_e) => {}
        }
    }
    pub fn distance_from_camera(&self, camera: &Camera) -> Fixed {
        match self {
            EntityEnum::Cube(c) => c.distance_from_camera(camera),
            EntityEnum::Rectangle(r) => r.distance_from_camera(camera),
            EntityEnum::Empty(_e) => Fixed::const_new(999),
        }
    }
    pub fn bounding_box(&self) -> BoundingBox {
        match self {
            EntityEnum::Cube(c) => c.bounding_box(),
            EntityEnum::Rectangle(r) => r.bounding_box(),
            EntityEnum::Empty(_e) => BoundingBox::default(),
        }
    }
    pub fn bounding_cylinder(&self) -> BoundingCylinder {
        match self {
            EntityEnum::Cube(c) => c.bounding_cylinder(),
            EntityEnum::Rectangle(r) => r.bounding_cylinder(),
            EntityEnum::Empty(_e) => BoundingCylinder::default(),
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

fn rect_simple_overlap_check(first: &BoundingBox, second: &BoundingBox) -> bool{
    let mut first_smallest_x = first.data[0][0];
    let mut first_largest_x = first.data[0][0];
    let mut first_smallest_y = first.data[0][1];
    let mut first_largest_y = first.data[0][1];

    let mut second_smallest_x = second.data[0][0];
    let mut second_largest_x = second.data[0][0];
    let mut second_smallest_y = second.data[0][1];
    let mut second_largest_y = second.data[0][1];

    for point in &first.data {
        if point[0] < first_smallest_x {
            first_smallest_x = point[0];
        }
        if point[0] > first_largest_x {
            first_largest_x = point[0];
        }
        if point[1] < first_smallest_y {
            first_smallest_y = point[1];
        }
        if point[1] > first_largest_y {
            first_largest_y = point[1];
        }
    }

    for point in &second.data {
        if point[0] < second_smallest_x {
            second_smallest_x = point[0];
        }
        if point[0] > second_largest_x {
            second_largest_x = point[0];
        }
        if point[1] < second_smallest_y {
            second_smallest_y = point[1];
        }
        if point[1] > second_largest_y {
            second_largest_y = point[1];
        }
    }

    if (first_largest_x < second_smallest_x || first_smallest_x > second_largest_x)
        || (first_largest_y < second_smallest_y || first_smallest_y > second_largest_y)
    {
        return false;
    }
    else {
        return true;
    }
}


pub fn rect_overlap(first: &BoundingBox, second: &BoundingBox) -> bool{
    for i in 0..4 {
        let cross1 = cross_product(second.data[0], second.data[1], first.data[i]);
        let cross2 = cross_product(second.data[1], second.data[2], first.data[i]);
        let cross3 = cross_product(second.data[2], second.data[3], first.data[i]);
        let cross4 = cross_product(second.data[3], second.data[0], first.data[i]);
        const Z: Fixed = Fixed::const_new(0);
        if (cross1 >= Z && cross2 >= Z && cross3 >= Z && cross4 >= Z)
            || (cross1 <= Z && cross2 <= Z && cross3 <= Z && cross4 <= Z)
        {
            return true;
        }
    }
    for i in 0..4 {
        let cross1 = cross_product(first.data[0], first.data[1], second.data[i]);
        let cross2 = cross_product(first.data[1], first.data[2], second.data[i]);
        let cross3 = cross_product(first.data[2], first.data[3], second.data[i]);
        let cross4 = cross_product(first.data[3], first.data[0], second.data[i]);

        const Z: Fixed = Fixed::const_new(0);
        if (cross1 >= Z && cross2 >= Z && cross3 >= Z && cross4 >= Z)
            || (cross1 <= Z && cross2 <= Z && cross3 <= Z && cross4 <= Z)
        {
            return true;
        }
    }
    return false;
}

pub fn vertical_room_check(first: &BoundingBox, second: &BoundingBox, limit: Fixed) -> Fixed {
    if (limit < Fixed::const_new(0) && first.y_top > second.y_bottom)
        || (limit > Fixed::const_new(0) && first.y_bottom < second.y_top)
    {
        return limit;
    }

    if !rect_simple_overlap_check(first, second) {
        return limit;
    }

    if rect_overlap(first, second) {
        if limit < Fixed::const_new(0) {
            return first.y_top;
        }
        else {
            return first.y_bottom;
        }
    }

    return limit;
}

//determine if the element in the entiry array is below us and how far
pub fn check_support_below(entity_array: &[EntityEnum], element: usize) -> Fixed {
    let bottom: BoundingBox = entity_array[element].bounding_box();
    let mut distance: Fixed = Fixed::const_new(-999);
    for (i, e) in entity_array.iter().enumerate() {
        if i != 0 && i != 1 {
            let top: BoundingBox = e.bounding_box();
            let d: Fixed = vertical_room_check(&top, &bottom, Fixed::const_new(-999));
            if d > distance {
                distance = d;
            }
        }
    }
    return distance;
}

//todo also check for the top of the "head" of the player.
//can run checks as a duplicate rect_overlap call, but for the head instead
//if that - head height > max_height -> override
pub fn check_block_above(entity_array: &[EntityEnum], element: usize) -> Fixed {
    let top: BoundingBox = entity_array[element].bounding_box();
    let mut max_height: Fixed = Fixed::const_new(999);
    for (i, e) in entity_array.iter().enumerate() {
        if i != 0 && i != 1 {
            let bottom = e.bounding_box();
            let overlap_height: Fixed = vertical_room_check(&bottom, &top, Fixed::const_new(999));
            if overlap_height < max_height {
                max_height = overlap_height;
            }
        }
    }
    return max_height;
}


pub fn overlap_3d(box1: &BoundingBox, box2: &BoundingBox) -> bool {
    
    //can't overlap, if not sharing y coordinates (z here)
    if box1.y_top <= box2.y_bottom || box2.y_top <= box1.y_bottom {
        return false
    }
    
    if rect_simple_overlap_check(box1, box2) {
        return true;
    }
/*
    if rect_overlap(box1, box2) {
        return true;
    }
*/
    return false;
}

//todo: call this when we've figured out ow to get the new position of the player in the world
//might get top_bounding with an x & y offset
pub fn horizontal_collision_check(entity_array: &[EntityEnum], box1: BoundingBox) -> bool {

    for (i, e) in entity_array.iter().enumerate() {
        if i != 0 && i != 1 {
            let box2 = e.bounding_box();

            if overlap_3d(&box1, &box2) {
                return true;
            }
        }
    }
    return false;

}
