use crate::camera::Camera;
use crate::fixed;
use crate::math;
use agb::InternalAllocator;
use fixed::*;
use math::*;
use super::draw;
use super::polygon::Polygon;
use super::polygon::Shape;

extern crate alloc;
use alloc::vec::Vec;

use super::utils;

#[allow(dead_code)]
pub fn render_face_outline(
    screen_points: &[[Fixed; 2]],
    world_points: &[[Fixed; 3]],
    p1: usize,
    p2: usize,
    p3: usize,
    p4: usize,
    color: u16,
    polygons: &mut Vec<Polygon, InternalAllocator>,
    draw_always: bool,
) {
    let near = Fixed::from_raw(16);

    if world_points[p1][2] > near && world_points[p2][2] > near {
        polygons.push(Polygon {
            distance_from_camera: utils::polygon_avg_z_2(world_points, p1, p2),
            shape: Shape::Line([screen_points[p1], screen_points[p2]]),
            color,
            draw_always,
        });

    }
    if world_points[p2][2] > near && world_points[p3][2] > near {
        polygons.push(Polygon {
            distance_from_camera: utils::polygon_avg_z_2(world_points, p2, p3),
            shape: Shape::Line([screen_points[p2], screen_points[p3]]),
            color,
            draw_always,
        });

    }

    if world_points[p3][2] > near && world_points[p4][2] > near {
        polygons.push(Polygon {
            distance_from_camera: utils::polygon_avg_z_2(world_points, p3, p4),
            shape: Shape::Line([screen_points[p3], screen_points[p4]]),
            color,
            draw_always,

        });

    }
    if world_points[p4][2] > near && world_points[p1][2] > near {
        polygons.push(Polygon {
            distance_from_camera: utils::polygon_avg_z_2(world_points, p4, p1),
            shape: Shape::Line([screen_points[p4], screen_points[p1]]),
            color,
            draw_always,

        });

    }
}

//return true if visible, presume points to be defined in counter clockwise direction
pub fn back_face_culling(points: &[[Fixed; 3]], p1: usize, p2: usize, p3: usize) -> bool {
    //checking if some of the points are behind the camera?
    //then dont draw
    if points[p1][2] <= 0 || points[p2][2] <= 0 || points[p3][2] <= 0 {
        return false;
    }

    let v12: [Fixed; 3] = vector_sub(points[p2], points[p1]);
    let v23: [Fixed; 3] = vector_sub(points[p3], points[p2]);

    let normal: [Fixed; 3] = vector_cross_3d(v12, v23);

    //get center of the three polygons
    let polygon_center: [Fixed; 3] = [
        (points[p1][0] + points[p2][0] + points[p3][0]) / 3,
        (points[p1][1] + points[p2][1] + points[p3][1]) / 3,
        (points[p1][2] + points[p2][2] + points[p3][2]) / 3,
    ];

    //calculate view direction towards the center of the polygon
    let view_dir: [Fixed; 3] = normalize(polygon_center);

    let dot_prod: Fixed = vector_dot(normal, view_dir);
    return dot_prod < 0;
}


#[inline(always)]
pub fn render_rect(
    model_rotated_points: &[[Fixed; 3]; 8],
    x: Fixed,
    y: Fixed,
    z: Fixed,
    _y_rotation: Fixed,
    camera_ptr: &Camera,
    color: u16,
    polygons: &mut Vec<Polygon, InternalAllocator>,
    draw_always: bool,
) {
    let mut screen_points: [[Fixed; 2]; 8] = [[Fixed::const_new(0), Fixed::const_new(0)]; 8];
    let mut translated_points: [[Fixed; 3]; 8] = [[
        Fixed::const_new(0),
        Fixed::const_new(0),
        Fixed::const_new(0),
    ]; 8];

    for i in 0..(*model_rotated_points).len() {
        (translated_points[i], screen_points[i]) =
            translate_point(&model_rotated_points[i], camera_ptr, x, y, z);
    }

    let visible: bool = back_face_culling(&translated_points, 0, 1, 2);
    if visible {
        let color: u16 = utils::get_color(color, 1);
        //todo, calculate the average distance from camera, possibly as the average of the 3 z coordinates
        //get via a function to make it easy to change later on
        let distance0 = utils::polygon_avg_z(&translated_points, 0, 1, 2);
        polygons.push(Polygon {
            distance_from_camera: distance0,
            shape: Shape::Triangle([screen_points[0], screen_points[1], screen_points[2]]),
            color,
            draw_always,
        });
        let distance1 = utils::polygon_avg_z(&translated_points, 0, 2, 3);
        polygons.push(Polygon {
            distance_from_camera: distance1,
            shape: Shape::Triangle([screen_points[0], screen_points[2], screen_points[3]]),
            color,
            draw_always,

        });

    }
    let visible = back_face_culling(&translated_points, 7, 6, 5);
    if visible {
        let color = utils::get_color(color, 1);
        let distance0 = utils::polygon_avg_z(&translated_points, 7, 6, 5);
        polygons.push(Polygon {
            distance_from_camera: distance0,
            shape: Shape::Triangle([screen_points[7], screen_points[6], screen_points[5]]),
            color,
            draw_always,

        });
        let distance1 = utils::polygon_avg_z(&translated_points, 7, 5, 4);
        polygons.push(Polygon {
            distance_from_camera: distance1,
            shape: Shape::Triangle([screen_points[7], screen_points[5], screen_points[4]]),
            color,
            draw_always,

        });

    }
    let visible = back_face_culling(&translated_points, 0, 3, 7);

    if visible {
        let color = utils::get_color(color, 2);
        let distance0 = utils::polygon_avg_z(&translated_points, 0, 3, 7);
        polygons.push(Polygon {
            distance_from_camera: distance0,
            shape: Shape::Triangle([screen_points[0], screen_points[3], screen_points[7]]),
            color,
            draw_always,

        });
        let distance1 = utils::polygon_avg_z(&translated_points, 0, 7, 4);
        polygons.push(Polygon {
            distance_from_camera: distance1,
            shape: Shape::Triangle([screen_points[0], screen_points[7], screen_points[4]]),
            color,
            draw_always,

        });

    }
    let visible = back_face_culling(&translated_points, 1, 5, 6);
    if visible {
        let color = utils::get_color(color, 2);
        let distance0 = utils::polygon_avg_z(&translated_points, 1, 5, 6);
        polygons.push(Polygon {
            distance_from_camera: distance0,
            shape: Shape::Triangle([screen_points[1], screen_points[5], screen_points[6]]),
            color,
            draw_always,

        });
        let distance1 = utils::polygon_avg_z(&translated_points, 1, 6, 2);
        polygons.push(Polygon {
            distance_from_camera: distance1,
            shape: Shape::Triangle([screen_points[1], screen_points[6], screen_points[2]]),
            color,
            draw_always,

        });

    }
    let visible = back_face_culling(&translated_points, 7, 3, 2);
    if visible {
        let color = utils::get_color(color, 0);
        let distance0 = utils::polygon_avg_z(&translated_points, 7, 3, 2);
        polygons.push(Polygon {
            distance_from_camera: distance0,
            shape: Shape::Triangle([screen_points[7], screen_points[3], screen_points[2]]),
            color,
            draw_always,

        });
        let distance1 = utils::polygon_avg_z(&translated_points, 7, 2, 6);
        polygons.push(Polygon {
            distance_from_camera: distance1,
            shape: Shape::Triangle([screen_points[7], screen_points[2], screen_points[6]]),
            color,
            draw_always,

        });

    }
    let visible = back_face_culling(&translated_points, 0, 4, 5);
    if visible {
        let color = utils::get_color(color, 0);
        let distance0 = utils::polygon_avg_z(&translated_points, 0, 4, 5);
        polygons.push(Polygon {
            distance_from_camera: distance0,
            shape: Shape::Triangle([screen_points[0], screen_points[4], screen_points[5]]),
            color,
            draw_always,

        });
        let distance1 = utils::polygon_avg_z(&translated_points, 0, 5, 1);
        polygons.push(Polygon {
            distance_from_camera: distance1,
            shape: Shape::Triangle([screen_points[0], screen_points[5], screen_points[1]]),
            color,
            draw_always,

        });

    }
}

#[inline(always)]
pub fn render_wireframe_rect(
    model_rotated_points: &[[Fixed; 3]; 8],
    x: Fixed,
    y: Fixed,
    z: Fixed,
    _y_rotation: Fixed,
    camera_ptr: &Camera,
    color: u16,
    polygons: &mut Vec<Polygon, InternalAllocator>,
) {
    let mut screen_points: [[Fixed; 2]; 8] = [[Fixed::const_new(0); 2]; 8];
    let mut translated_points: [[Fixed; 3]; 8] = [[Fixed::const_new(0); 3]; 8];

    for i in 0..(*model_rotated_points).len() {
        let screen_point: [Fixed; 2];
        (translated_points[i], screen_point) =
            translate_point(&model_rotated_points[i], camera_ptr, x, y, z);
        screen_points[i] = [screen_point[0], screen_point[1]];
    }

    let wire_color = color * 8 + 7;

    render_face_outline(
        &screen_points,
        &translated_points,
        0,
        1,
        2,
        3,
        wire_color,
        polygons,
        false
    );
    render_face_outline(
        &screen_points,
        &translated_points,
        4,
        5,
        6,
        7,
        wire_color,
        polygons,
        false
    );
    render_face_outline(
        &screen_points,
        &translated_points,
        3,
        2,
        6,
        7,
        wire_color,
        polygons,
        false

    );
    render_face_outline(
        &screen_points,
        &translated_points,
        0,
        1,
        5,
        4,
        wire_color,
        polygons,
        false

    );
}

pub fn translate_point(
    model_rotated_point: &[Fixed; 3],
    camera_ptr: &Camera,
    x: Fixed,
    y: Fixed,
    z: Fixed,
) -> ([Fixed; 3], [Fixed; 2]) {
    let width: i32 = 240;
    let height: i32 = 160;
    let middle: [Fixed; 2] = [Fixed::const_new(width / 2), Fixed::const_new(height / 2)]; // x, y

    let mut translated_point: [Fixed; 4] = [
        (*model_rotated_point)[0] + (x - (*camera_ptr).x),
        (*model_rotated_point)[1] + (y - (*camera_ptr).y),
        (*model_rotated_point)[2] + (z - (*camera_ptr).z),
        Fixed::const_new(1),
    ];

    translated_point = matmul_4((*camera_ptr).y_rotation_matrix, translated_point);
    translated_point = matmul_4((*camera_ptr).x_rotation_matrix, translated_point);
    translated_point = matmul_4((*camera_ptr).z_rotation_matrix, translated_point);

    // Apply projection matrix
    let projected_point = matmul_4(utils::PROJECTION_MATRIX, translated_point);

    let screen_point: [Fixed; 2];

    // Perform perspective divide (convert to 2D)
    if projected_point[3] != Fixed::const_new(0) {
        let x: Fixed = projected_point[0] / projected_point[3];
        let y: Fixed = projected_point[1] / projected_point[3];
        // Convert to screen space
        screen_point = [
            (x * Fixed::const_new(width) / Fixed::const_new(2)) + middle[0],
            (y * Fixed::const_new(height) / Fixed::const_new(2)) + middle[1],
        ];
    } else {
        screen_point = [middle[0], middle[1]];
    }

    let translated_point: [Fixed; 3] = [
        translated_point[0],
        translated_point[1],
        translated_point[2],
    ];

    return (translated_point, screen_point);
}

pub fn render_polygons(
    polygons: &Vec<Polygon, InternalAllocator>,
    polygon_indices: &[usize],
    start: usize,
    page: u16,
) {
    for p in 0..polygons.len() {
        if p < start && !polygons[polygon_indices[p]].draw_always {
            continue;
        }
        let polygon: &Polygon = &polygons[polygon_indices[p]];
        if let Some(vertices) = polygon.as_triangle() {
            draw::draw_triangle(
                vertices[0],
                vertices[1],
                vertices[2],
                polygon.color,
                page,
            );
        }
        if let Some(vertices) = polygon.as_line() {
            draw::draw_line_fixed(
                vertices[0][0],
                vertices[0][1],
                vertices[1][0],
                vertices[1][1],
                polygon.color,
                page,
            );
        }
    }
}