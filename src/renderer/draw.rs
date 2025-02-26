use crate::renderer::hw;

use super::{utils, Fixed};

use utils::safe_fraction_fixed;


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
    //TODO: Fix this like with the render_face_outline and world coordinates
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

