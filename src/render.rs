

use crate::math;
use math::*;
use crate::fixed;
use fixed::*;

#[allow(dead_code)]
pub fn draw_line(
    bitmap: &mut agb::display::bitmap4::Bitmap4,
    mut x1: i32,
    mut y1: i32,
    x2: i32,
    y2: i32,
    color: u8,
) {
    let dx: i32 = (x2 - x1).abs();
    let dy: i32 = (y2 - y1).abs();

    let sx: i32;
    let sy: i32;

    if x1 < x2 {
        sx = 1
    } else {
        sx = -1
    }
    if y1 < y2 {
        sy = 1
    } else {
        sy = -1;
    }

    let mut err: i32 = dx - dy;

    loop {
        bitmap.draw_point(x1, y1, color);
        if x1 == x2 && y1 == y2 {
            break;
        }

        let e2: i32 = 2 * err;
        if e2 > -dy {
            err -= dy;
            x1 += sx
        }
        if e2 < dx {
            err += dx;
            y1 += sy;
        }
    }
}

#[allow(dead_code)]
pub fn draw_face_outline(
    bitmap4: &mut agb::display::bitmap4::Bitmap4,
    screen_points: [[i32; 2]; 8],
    p1: usize,
    p2: usize,
    p3: usize,
    p4: usize,
) {
    draw_line(
        bitmap4,
        screen_points[p1][0],
        screen_points[p1][1],
        screen_points[p2][0],
        screen_points[p2][1],
        1,
    );
    draw_line(
        bitmap4,
        screen_points[p2][0],
        screen_points[p2][1],
        screen_points[p3][0],
        screen_points[p3][1],
        1,
    );
    draw_line(
        bitmap4,
        screen_points[p3][0],
        screen_points[p3][1],
        screen_points[p4][0],
        screen_points[p4][1],
        1,
    );
    draw_line(
        bitmap4,
        screen_points[p4][0],
        screen_points[p4][1],
        screen_points[p1][0],
        screen_points[p1][1],
        1,
    );
}

//return true if visible, presume points to be defined in counter clockwise direction
pub fn back_face_culling(
    &points: &[[Fixed; 3]; 8],
    p1: usize,
    p2: usize,
    p3: usize,
) -> bool {

    let v12: [Fixed; 3] = vector_sub(points[p2], points[p1]);
    let v23: [Fixed; 3] = vector_sub(points[p3], points[p2]);

    let normal: [Fixed; 3] = vector_cross(v12, v23);

    //get center of the three polygons
    let polygon_center: [Fixed; 3] = [
        (points[p1][0] + points[p2][0] + points[p3][0]) / 3,
        (points[p1][1] + points[p2][1] + points[p3][1]) / 3,
        (points[p1][2] + points[p2][2] + points[p3][2]) / 3,
    ];

    //doing this for all points instead
    //behind camera, so not visible
    if polygon_center[2] < Fixed::const_new(1) {
        return false;
    }

    //calculate view direction towards the center of the polygon
    let view_dir: [Fixed; 3] = normalize(polygon_center);

    let dot_prod: Fixed = vector_dot(normal, view_dir);
    return dot_prod < Fixed::const_new(0);
}

pub fn draw_h_line(
    bitmap4: &mut agb::display::bitmap4::Bitmap4,
    x1: i32,
    x2: i32,
    y: i32,
    color: u8,
) {
    // Ensure x1 is less than or equal to x2 for proper iteration
    let (mut start, mut end) = if x1 <= x2 { (x1, x2) } else { (x2, x1) };

    start = if start < 0 { 0 } else { start };
    end = if end > 239 { 239 } else { end };

    // Adjust start to be the first even number in the range
    start = if start % 2 == 0 { start } else { start + 1 };

    // Iterate over even numbers using step_by(2)
    for x in (start..=end).step_by(2) {
        // Draw each point on the horizontal line
        bitmap4.draw_wide_point(x, y, color);
    }
}

pub fn draw_flat_bottom_triangle(
    bitmap4: &mut agb::display::bitmap4::Bitmap4,
    p1: [Fixed; 2],
    p2: [Fixed; 2],
    p3: [Fixed; 2],
    color: u8,
) {
    let mut div1 = p2[1] - p1[1];
    let mut div2 = p3[1] - p1[1];

    if div1 < Fixed::const_new(3) {
        div1 = Fixed::const_new(3);
    }
    if div2 < Fixed::const_new(3) {
        div2 = Fixed::const_new(3);
    }

    let invslope1: Fixed = (p2[0] - p1[0]) / (div1);
    let invslope2: Fixed = (p3[0] - p1[0]) / (div2);
    let mut curx1: Fixed = p1[0];
    let mut curx2: Fixed = p1[0];

    let mut y_top: i32 = p1[1].trunc();
    let y_bottom: i32 = p3[1].trunc();

    let y_top_difference = if y_top < 0 { 0 - y_top } else { 0 };
    y_top += y_top_difference;

    curx1 += invslope1 * y_top_difference;
    curx2 += invslope2 * y_top_difference;

    // Iterate over the scanlines from the top (p1 and p2) down to p3
    for scanline_y in y_top..=y_bottom {
        if scanline_y > 159 {
            break;
        }
        draw_h_line(bitmap4, curx1.trunc(), curx2.trunc(), scanline_y, color);
        curx1 += invslope1;
        curx2 += invslope2;
    }
}

pub fn draw_flat_top_triangle(
    bitmap4: &mut agb::display::bitmap4::Bitmap4,
    p1: [Fixed; 2],
    p2: [Fixed; 2],
    p3: [Fixed; 2],
    color: u8,
) {
    let mut div1 = p3[1] - p1[1];
    let mut div2 = p3[1] - p2[1];
    if div1 < Fixed::const_new(3) {
        div1 = Fixed::const_new(3);
    }
    if div2 < Fixed::const_new(3) {
        div2 = Fixed::const_new(3);
    }
    // Calculate the slopes (invslope1 and invslope2)
    let invslope1: Fixed = (p3[0] - p1[0]) / (div1);
    let invslope2: Fixed = (p3[0] - p2[0]) / (div2);

    // Initialize the starting x-coordinates at the top vertices
    let mut curx1: Fixed = p1[0];
    let mut curx2: Fixed = p2[0];

    let mut y_top: i32 = p1[1].trunc();
    let y_bottom: i32 = p3[1].trunc();

    let y_top_difference = if y_top < 0 { 0 - y_top } else { 0 };
    y_top += y_top_difference;

    curx1 += invslope1 * y_top_difference;
    curx2 += invslope2 * y_top_difference;

    // Iterate over the scanlines from the top (p1 and p2) down to p3
    for scanline_y in y_top..=y_bottom {
        if scanline_y > 159 {
            break;
        }
        draw_h_line(bitmap4, curx1.trunc(), curx2.trunc(), scanline_y, color);
        curx1 += invslope1;
        curx2 += invslope2;
    }
}

pub fn sort_points(
    p1: &mut [Fixed; 2],
    p2: &mut [Fixed; 2],
    p3: &mut [Fixed; 2],
) {
    // Swap points to ensure p1 has the smallest y, then x
    if (p2[1] < p1[1]) || (p2[1] == p1[1] && p2[0] < p1[0]) {
        for i in 0..2 {
            let temp: Fixed = p1[i];
            p1[i] = p2[i];
            p2[i] = temp;
        }
    }
    if (p3[1] < p1[1]) || (p3[1] == p1[1] && p3[0] < p1[0]) {
        for i in 0..2 {
            let temp: Fixed = p1[i];
            p1[i] = p3[i];
            p3[i] = temp;
        }
    }
    // Ensure p2 is the middle and p3 is the largest
    if (p3[1] < p2[1]) || (p3[1] == p2[1] && p3[0] < p2[0]) {
        for i in 0..2 {
            let temp: Fixed = p2[i];
            p2[i] = p3[i];
            p3[i] = temp;
        }
    }
}


pub fn draw_triangle(
    bitmap4: &mut agb::display::bitmap4::Bitmap4,
    mut p1: [Fixed; 2],
    mut p2: [Fixed; 2],
    mut p3: [Fixed; 2],
    color: u8,
) {
    let zero: Fixed = Fixed::const_new(0);
    let x_max: Fixed = Fixed::const_new(240);
    let y_max: Fixed = Fixed::const_new(160);
    
    //jank way to avoid giant polygons near zero plane
    if p1[0] > Fixed::const_new(1000) || p1[0] < Fixed::const_new(-100) {
        return
    }
    if p2[0] > Fixed::const_new(1000) || p2[0] < Fixed::const_new(-100) {
        return
    }   
    if p3[0] > Fixed::const_new(1000) || p3[0] < Fixed::const_new(-100) {
        return
    }    
    if p1[1] > Fixed::const_new(1000) || p1[1] < Fixed::const_new(-100) {
        return
    }
    if p2[1] > Fixed::const_new(1000) || p2[1] < Fixed::const_new(-100) {
        return
    }   if p3[1] > Fixed::const_new(1000) || p3[1] < Fixed::const_new(-100) {
        return
    }

    //first check out if the triangle is completely out of view
    if p1[0] < zero && p2[0] < zero && p3[0] < zero
        || p1[1] < zero && p2[1] < zero && p3[1] < zero
        || p1[0] > x_max && p2[0] > x_max && p3[0] > x_max
        || p1[1] > y_max && p2[1] > y_max && p3[1] > y_max
    {
        return;
    }

    sort_points(&mut p1, &mut p2, &mut p3);
    //flat top triangle
    if p1[1] == p2[1] {
        draw_flat_top_triangle(bitmap4, p1, p2, p3, color);
    }
    //flat bottom triangle
    else if p2[1] == p3[1] {
        draw_flat_bottom_triangle(bitmap4, p1, p2, p3, color);
    } else {
        let p4x: Fixed = p1[0] + (p2[1] - p1[1]) / (p3[1] - p1[1]) * (p3[0] - p1[0]);
        draw_flat_bottom_triangle(bitmap4, p1, p2, [p4x, p2[1]], color);
        draw_flat_top_triangle(bitmap4, p2, [p4x, p2[1]], p3, color);
    }
}
