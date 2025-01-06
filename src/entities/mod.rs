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
    pub fn reload_rotation_matrices(&mut self) {
        match self {
            EntityEnum::Cube(c) => c.reload_rotation_matrices(),
            EntityEnum::Rectangle(r) => r.reload_rotation_matrices(),
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
    pub fn render(&mut self, camera: &Camera, page: u32) {
        match self {
            EntityEnum::Cube(c) => c.render(camera, page),
            EntityEnum::Rectangle(r) => r.render(camera, page),
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
    pub fn get_y(&self) -> Fixed {
        match self {
            EntityEnum::Cube(c) => c.get_y(),
            EntityEnum::Rectangle(r) => r.get_y(),
            EntityEnum::Empty(_e) => Fixed::const_new(-999),
        }
    }
    pub fn set_color(&mut self, color: u8) {
        match self {
            EntityEnum::Cube(c) => c.set_color(color),
            EntityEnum::Rectangle(r) => r.set_color(color),
            EntityEnum::Empty(_e) => {}
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
    let pivot_y = entity_array[entity_render_order[high]].get_y();

    let mut i = low as isize - 1; // Use `isize` to allow `-1` for initialization

    //very jank method to avoid having to do actual per polygon zindexing
    //basically objects with largest y are drawn last, use distance to camera as fallback
    //this needs to be kept in mind when creating levels eventually
    for j in low..high {
        let y = entity_array[entity_render_order[j]].get_y();
        if y < pivot_y {
            i += 1;
            entity_render_order.swap(i as usize, j);
        } else if y > pivot_y {
            //already when we want it
        } else if entity_array[entity_render_order[j]].distance_from_camera(camera)
            >= pivot_distance
        {
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

fn rect_simple_overlap_check(first: &BoundingBox, second: &BoundingBox) -> bool {
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
    } else {
        return true;
    }
}

pub fn rect_overlap(first: &BoundingBox, second: &BoundingBox) -> bool {
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
        } else {
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

#[allow(dead_code)]
pub fn cylinder_and_rect_collision(cyl1: &BoundingCylinder, box2: &BoundingBox) -> bool {
    //can't overlap, if not sharing y coordinates (z here)
    if cyl1.y_top <= box2.y_bottom || box2.y_top <= cyl1.y_bottom {
        return false;
    }
    // Step 1: Deduce rectangle bounds
    let mut min_x: Fixed = box2.data[0][0];
    let mut max_x: Fixed = box2.data[0][0];
    let mut min_z: Fixed = box2.data[0][1];
    let mut max_z: Fixed = box2.data[0][1];

    for point in box2.data.iter() {
        let x: Fixed = point[0];
        let z: Fixed = point[1];
        if x < min_x {
            min_x = x;
        }
        if x > max_x {
            max_x = x;
        }
        if z < min_z {
            min_z = z;
        }
        if z > max_z {
            max_z = z;
        }
    }

    let clamped_x: Fixed = if cyl1.x < min_x {
        min_x
    } else if cyl1.x > max_x {
        max_x
    } else {
        cyl1.x
    };
    let clamped_y: Fixed = if cyl1.z < min_z {
        min_z
    } else if cyl1.z > max_z {
        max_z
    } else {
        cyl1.z
    };

    let dx: Fixed = clamped_x - cyl1.x;
    let dy: Fixed = clamped_y - cyl1.z;
    let distance_squared: Fixed = dx * dx + dy * dy;

    //true if there is a collision
    distance_squared <= cyl1.radius * cyl1.radius
}

pub fn cylinder_and_rotated_rect_collision(
    cyl1: &BoundingCylinder,
    box2: &BoundingBox,
) -> (Fixed, bool) {
    // Can't overlap if not sharing y coordinates (z here)
    if cyl1.y_top <= box2.y_bottom || box2.y_top <= cyl1.y_bottom {
        return (Fixed::default(), false);
    }

    // Step 1: Transform the cylinder's center into the rectangle's local space
    let (cos_theta, sin_theta) = (box2.rotation.cos(), box2.rotation.sin());
    let tx = cyl1.x - box2.center[0];
    let tz = cyl1.z - box2.center[1];
    let local_x = tx * cos_theta + tz * sin_theta;
    let local_z = -tx * sin_theta + tz * cos_theta;

    // Step 2: Deduce rectangle bounds in local space
    let half_width = box2.width / Fixed::const_new(2);
    let half_height = box2.height / Fixed::const_new(2);
    let min_x = -half_width;
    let max_x = half_width;
    let min_z = -half_height;
    let max_z = half_height;

    // Step 3: Clamp the cylinder's center to the rectangle's bounds in local space
    let clamped_x = if local_x < min_x {
        min_x
    } else if local_x > max_x {
        max_x
    } else {
        local_x
    };
    let clamped_z = if local_z < min_z {
        min_z
    } else if local_z > max_z {
        max_z
    } else {
        local_z
    };

    // Step 4: Transform the clamped point back to world space (not necessary for distance calc)
    let dx = local_x - clamped_x;
    let dz = local_z - clamped_z;
    let distance_squared = dx * dx + dz * dz;

    // True if there is a collision
    if distance_squared <= cyl1.radius * cyl1.radius {
        return (box2.rotation, true);
    }
    return (Fixed::default(), false);
}

pub fn horizontal_collision_check(
    entity_array: &[EntityEnum],
    cyl1: BoundingCylinder,
) -> (Fixed, bool) {
    for (i, e) in entity_array.iter().enumerate() {
        if i != 0 && i != 1 {
            let box2: BoundingBox = e.bounding_box();

            if box2.rotation == Fixed::const_new(0) {
                if cylinder_and_rect_collision(&cyl1, &box2) {
                    return (Fixed::const_new(0), true);
                }
            } else {
                let (wallangle, ok) = cylinder_and_rotated_rect_collision(&cyl1, &box2);
                if ok {
                    return (wallangle, true);
                }
            }
        }
    }
    return (Fixed::default(), false);
}
