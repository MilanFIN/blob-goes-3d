pub mod hw;
pub mod utils;
use crate::camera::Camera;
use crate::fixed;
use crate::math;
use fixed::*;
use math::*;

#[inline(always)]
fn safe_fraction_fixed(numerator: Fixed, denominator: Fixed) -> Fixed {
    if denominator == 0 {
        return Fixed::const_new(0);
    }
    return numerator / denominator;
}

pub fn draw_line_fixed(mut x1: Fixed, mut y1: Fixed, x2: Fixed, y2: Fixed, color: u16, page: u16) {
    const SCREEN_MIN_X: Fixed = Fixed::const_new(0);
    const SCREEN_MAX_X: Fixed = Fixed::const_new(239);
    const SCREEN_MIN_Y: Fixed = Fixed::const_new(0);
    const SCREEN_MAX_Y: Fixed = Fixed::const_new(159);

    let sx: Fixed = if x1 < x2 {
        Fixed::const_new(1)
    } else {
        Fixed::const_new(-1)
    };
    let sy: Fixed = if y1 < y2 {
        Fixed::const_new(1)
    } else {
        Fixed::const_new(-1)
    };

    if x1 < SCREEN_MIN_X && sx > 0 {
        let remaining_x = x2 - SCREEN_MIN_X;
        let x_part = safe_fraction_fixed(remaining_x, x2 - x1);
        x1 = SCREEN_MIN_X;
        y1 = y2 - (y2 - y1) * x_part
    } else if x1 > SCREEN_MAX_X && sx < 0 {
        let remaining_x = x1 - SCREEN_MAX_X;
        let x_part = safe_fraction_fixed(remaining_x, x1 - x2);
        x1 = SCREEN_MAX_X;
        y1 = y1 - (y1 - y2) * x_part;
    }

    if y1 < SCREEN_MIN_Y && sy > 0 {
        let remaining_y = y2 - SCREEN_MIN_Y;
        let y_part = safe_fraction_fixed(remaining_y, y2 - y1);
        y1 = SCREEN_MIN_Y;
        x1 = x2 - (x2 - x1) * y_part;
    } else if y1 > SCREEN_MAX_Y && sy < 0 {
        let remaining_y = y1 - SCREEN_MAX_X;
        let y_part = safe_fraction_fixed(remaining_y, y1 - y2);
        y1 = SCREEN_MAX_X;
        x1 = x1 - (x1 - x2) * y_part;
    }

    let dx: Fixed = (x2 - x1).abs();
    let dy: Fixed = (y2 - y1).abs();
    let mut err: Fixed = dx - dy;

    loop {
        // Check if the current point is within the screen bounds
        if x1 >= SCREEN_MIN_X && x1 <= SCREEN_MAX_X && y1 >= SCREEN_MIN_Y && y1 <= SCREEN_MAX_Y {
            hw::draw_wide_point(x1.trunc(), y1.trunc(), color, page);
        } else if (x1 < SCREEN_MIN_X && sx == -1)
            || (x1 > SCREEN_MAX_X && sx == 1)
            || (y1 < SCREEN_MIN_Y && sy == -1)
            || (y1 > SCREEN_MAX_Y && sy == 1)
        {
            // Stop the loop if moving further will remain out of bounds
            break;
        }

        //check for >< instead of > here, so x1 > x2, if sx is 1
        if (sx == Fixed::const_new(1) && x1 > x2) || (sx == Fixed::const_new(-1) && x1 < x2) {
            break;
        }
        if (sy == Fixed::const_new(1) && y1 > y2) || (sy == Fixed::const_new(-1) && y1 < y2) {
            break;
        }
        if x1.trunc() == x2.trunc() && y1.trunc() == y2.trunc() {
            break;
        }

        let e2: Fixed = err * 2;
        if e2 > -dy {
            err -= dy;
            x1 += sx;
        }
        if e2 < dx {
            err += dx;
            y1 += sy;
        }
    }
}

#[allow(dead_code)]
pub fn draw_face_outline(
    screen_points: &[[Fixed; 2]],
    world_points: &[[Fixed; 3]],
    p1: usize,
    p2: usize,
    p3: usize,
    p4: usize,
    page: u16,
    color: u16,
) {
    let near = Fixed::from_raw(16);

    if world_points[p1][2] > near && world_points[p2][2] > near {
        draw_line_fixed(
            screen_points[p1][0],
            screen_points[p1][1],
            screen_points[p2][0],
            screen_points[p2][1],
            color,
            page,
        );
    }
    if world_points[p2][2] > near && world_points[p3][2] > near {
        draw_line_fixed(
            screen_points[p2][0],
            screen_points[p2][1],
            screen_points[p3][0],
            screen_points[p3][1],
            color,
            page,
        );
    }

    if world_points[p3][2] > near && world_points[p4][2] > near {
        draw_line_fixed(
            screen_points[p3][0],
            screen_points[p3][1],
            screen_points[p4][0],
            screen_points[p4][1],
            color,
            page,
        );
    }
    if world_points[p4][2] > near && world_points[p1][2] > near {
        draw_line_fixed(
            screen_points[p4][0],
            screen_points[p4][1],
            screen_points[p1][0],
            screen_points[p1][1],
            color,
            page,
        );
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

pub fn draw_h_line(x1: i32, x2: i32, y: i32, color: u16, page: u16) {
    // Ensure x1 is less than or equal to x2 for proper iteration
    let (mut start, mut end) = if x1 <= x2 { (x1, x2) } else { (x2, x1) };

    start = if start < 0 { 0 } else { start };
    end = if end > 239 { 239 } else { end };

    // Adjust start to be the first even number in the range
    start = if start % 2 == 0 { start } else { start + 1 };

    // Iterate over even numbers using step_by(2)
    for x in (start..=end).step_by(2) {
        // Draw each point on the horizontal line
        hw::draw_wide_point(x, y, color, page);
    }
}

pub fn draw_flat_bottom_triangle(
    p1: [Fixed; 2],
    p2: [Fixed; 2],
    p3: [Fixed; 2],
    color: u16,
    page: u16,
) {
    let mut div1 = p2[1] - p1[1];
    let mut div2 = p3[1] - p1[1];

    if div1 < 3 {
        div1 = Fixed::const_new(3);
    }
    if div2 < 3 {
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
        draw_h_line(curx1.trunc(), curx2.trunc(), scanline_y, color, page);
        curx1 += invslope1;
        curx2 += invslope2;
    }
}

pub fn draw_flat_top_triangle(
    p1: [Fixed; 2],
    p2: [Fixed; 2],
    p3: [Fixed; 2],
    color: u16,
    page: u16,
) {
    let mut div1 = p3[1] - p1[1];
    let mut div2 = p3[1] - p2[1];
    if div1 < 3 {
        div1 = Fixed::const_new(3);
    }
    if div2 < 3 {
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
        draw_h_line(curx1.trunc(), curx2.trunc(), scanline_y, color, page);
        curx1 += invslope1;
        curx2 += invslope2;
    }
}

pub fn sort_points(p1: &mut [Fixed; 2], p2: &mut [Fixed; 2], p3: &mut [Fixed; 2]) {
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
    mut p1: [Fixed; 2],
    mut p2: [Fixed; 2],
    mut p3: [Fixed; 2],
    color: u16,
    page: u16,
) {
    let zero: Fixed = Fixed::const_new(0);
    let x_max: Fixed = Fixed::const_new(240);
    let y_max: Fixed = Fixed::const_new(160);

    //jank way to avoid giant polygons near zero plane
    //TODO: Fix this like with the draw_face_outline and world coordinates
    if p1[0] > Fixed::const_new(340) || p1[0] < Fixed::const_new(-100) {
        return;
    }
    if p2[0] > Fixed::const_new(340) || p2[0] < Fixed::const_new(-100) {
        return;
    }
    if p3[0] > Fixed::const_new(340) || p3[0] < Fixed::const_new(-100) {
        return;
    }
    if p1[1] > Fixed::const_new(260) || p1[1] < Fixed::const_new(-100) {
        return;
    }
    if p2[1] > Fixed::const_new(260) || p2[1] < Fixed::const_new(-100) {
        return;
    }
    if p3[1] > Fixed::const_new(260) || p3[1] < Fixed::const_new(-100) {
        return;
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
        draw_flat_top_triangle(p1, p2, p3, color, page);
    }
    //flat bottom triangle
    else if p2[1] == p3[1] {
        draw_flat_bottom_triangle(p1, p2, p3, color, page);
    } else {
        let p4x: Fixed = p1[0] + (p2[1] - p1[1]) / (p3[1] - p1[1]) * (p3[0] - p1[0]);
        draw_flat_bottom_triangle(p1, p2, [p4x, p2[1]], color, page);
        draw_flat_top_triangle(p2, [p4x, p2[1]], p3, color, page);
    }
}

#[inline(always)]
pub fn draw_rect(
    model_rotated_points: &[[Fixed; 3]; 8],
    x: Fixed,
    y: Fixed,
    z: Fixed,
    _y_rotation: Fixed,
    camera_ptr: &Camera,
    color: u16,
    page: u16,
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
        draw_triangle(
            screen_points[0],
            screen_points[1],
            screen_points[2],
            color,
            page,
        );
        draw_triangle(
            screen_points[0],
            screen_points[2],
            screen_points[3],
            color,
            page,
        );
    }
    let visible = back_face_culling(&translated_points, 7, 6, 5);
    if visible {
        let color = utils::get_color(color, 1);
        draw_triangle(
            screen_points[7],
            screen_points[6],
            screen_points[5],
            color,
            page,
        );
        draw_triangle(
            screen_points[7],
            screen_points[5],
            screen_points[4],
            color,
            page,
        );
    }
    let visible = back_face_culling(&translated_points, 0, 3, 7);

    if visible {
        let color = utils::get_color(color, 2);

        draw_triangle(
            screen_points[0],
            screen_points[3],
            screen_points[7],
            color,
            page,
        );
        draw_triangle(
            screen_points[0],
            screen_points[7],
            screen_points[4],
            color,
            page,
        );
    }
    let visible = back_face_culling(&translated_points, 1, 5, 6);
    if visible {
        let color = utils::get_color(color, 2);

        draw_triangle(
            screen_points[1],
            screen_points[5],
            screen_points[6],
            color,
            page,
        );
        draw_triangle(
            screen_points[1],
            screen_points[6],
            screen_points[2],
            color,
            page,
        );
    }
    let visible = back_face_culling(&translated_points, 7, 3, 2);
    if visible {
        let color = utils::get_color(color, 0);

        draw_triangle(
            screen_points[7],
            screen_points[3],
            screen_points[2],
            color,
            page,
        );
        draw_triangle(
            screen_points[7],
            screen_points[2],
            screen_points[6],
            color,
            page,
        );
    }
    let visible = back_face_culling(&translated_points, 0, 4, 5);
    if visible {
        let color = utils::get_color(color, 0);

        draw_triangle(
            screen_points[0],
            screen_points[4],
            screen_points[5],
            color,
            page,
        );
        draw_triangle(
            screen_points[0],
            screen_points[5],
            screen_points[1],
            color,
            page,
        );
    }
}

#[inline(always)]
pub fn draw_wireframe_rect(
    model_rotated_points: &[[Fixed; 3]; 8],
    x: Fixed,
    y: Fixed,
    z: Fixed,
    _y_rotation: Fixed,
    camera_ptr: &Camera,
    color: u16,
    page: u16,
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

    draw_face_outline(
        &screen_points,
        &translated_points,
        0,
        1,
        2,
        3,
        page,
        wire_color,
    );
    draw_face_outline(
        &screen_points,
        &translated_points,
        4,
        5,
        6,
        7,
        page,
        wire_color,
    );
    draw_face_outline(
        &screen_points,
        &translated_points,
        3,
        2,
        6,
        7,
        page,
        wire_color,
    );
    draw_face_outline(
        &screen_points,
        &translated_points,
        0,
        1,
        5,
        4,
        page,
        wire_color,
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
