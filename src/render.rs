use agb::fixnum::Num;

use math::*;

use crate::math;

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

    let mut sx: i32;
    let mut sy: i32;

    if (x1 < x2) {
        sx = 1
    } else {
        sx = -1
    }
    if (y1 < y2) {
        sy = 1
    } else {
        sy = -1;
    }

    let mut err: i32 = dx - dy;

    while (true) {
        bitmap.draw_point(x1, y1, color);
        if (x1 == x2 && y1 == y2) {
            break;
        }

        let e2: i32 = 2 * err;
        if (e2 > -dy) {
            err -= dy;
            x1 += sx
        }
        if (e2 < dx) {
            err += dx;
            y1 += sy;
        }
    }
}

pub fn draw_face_outline(
    bitmap4: &mut agb::display::bitmap4::Bitmap4,
    screenPoints: [[i32; 2]; 8],
    p1: usize,
    p2: usize,
    p3: usize,
    p4: usize,
) {
    draw_line(
        bitmap4,
        screenPoints[p1][0],
        screenPoints[p1][1],
        screenPoints[p2][0],
        screenPoints[p2][1],
        1,
    );
    draw_line(
        bitmap4,
        screenPoints[p2][0],
        screenPoints[p2][1],
        screenPoints[p3][0],
        screenPoints[p3][1],
        1,
    );
    draw_line(
        bitmap4,
        screenPoints[p3][0],
        screenPoints[p3][1],
        screenPoints[p4][0],
        screenPoints[p4][1],
        1,
    );
    draw_line(
        bitmap4,
        screenPoints[p4][0],
        screenPoints[p4][1],
        screenPoints[p1][0],
        screenPoints[p1][1],
        1,
    );
}

//return true if visible, presume points to be defined in counter clockwise direction
pub fn backFaceCulling(&points: &[[Num<i32, 8>; 3]; 8], p1: usize, p2: usize, p3: usize) -> bool {
    let v12: [Num<i32, 8>; 3] = vectorSub(points[p2], points[p1]);
    let v23: [Num<i32, 8>; 3] = vectorSub(points[p3], points[p2]);

    let normal: [Num<i32, 8>; 3] = vectorCross(v12, v23);

    let viewDir: [Num<i32, 8>; 3] = [Num::new(0), Num::new(0), Num::new(1)];
    //get center of the three polygons
    let centroid: [Num<i32, 8>; 3] = [
        (points[p1][0] + points[p2][0] + points[p3][0]) / Num::new(3),
        (points[p1][1] + points[p2][1] + points[p3][1]) / Num::new(3),
        (points[p1][2] + points[p2][2] + points[p3][2]) / Num::new(3),
    ];
    //calculate view direction towards the center of the polygon
    let viewDir: [Num<i32, 8>; 3] = normalize(centroid);

    let dotProd: Num<i32, 8> = vectorDot(normal, viewDir).change_base();
    return dotProd < Num::new(0);
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
    end = if (end > 239) { 239 } else { end };

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
    p1: [Num<i32, 8>; 2],
    p2: [Num<i32, 8>; 2],
    p3: [Num<i32, 8>; 2],
    color: u8,
) {
    let mut div1 = p2[1] - p1[1];
    let mut div2 = p3[1] - p1[1];

    if (div1 < Num::new(3)) {
        div1 = Num::new(3);
    }
    if (div2 < Num::new(3)) {
        div2 = Num::new(3);
    }

    let invslope1: Num<i32, 8> = (p2[0] - p1[0]) / (div1);
    let invslope2: Num<i32, 8> = (p3[0] - p1[0]) / (div2);
    let mut curx1: Num<i32, 8> = (p1[0]);
    let mut curx2: Num<i32, 8> = (p1[0]);

    let mut yTop: i32 = p1[1].trunc();
    let yBottom: i32 = p3[1].trunc();

    let yTopDifference = if yTop < 0 { 0 - yTop } else { 0 };
    yTop += yTopDifference;

    curx1 += invslope1 * yTopDifference;
    curx2 += invslope2 * yTopDifference;

    // Iterate over the scanlines from the top (p1 and p2) down to p3
    for scanline_y in yTop..=yBottom {
        if (scanline_y > 159) {
            break;
        }
        draw_h_line(bitmap4, curx1.trunc(), curx2.trunc(), scanline_y, color);
        curx1 += invslope1;
        curx2 += invslope2;
    }
}

pub fn draw_flat_top_triangle(
    bitmap4: &mut agb::display::bitmap4::Bitmap4,
    p1: [Num<i32, 8>; 2],
    p2: [Num<i32, 8>; 2],
    p3: [Num<i32, 8>; 2],
    color: u8,
) {
    let mut div1 = p3[1] - p1[1];
    let mut div2 = p3[1] - p2[1];
    if (div1 < Num::new(3)) {
        div1 = Num::new(3);
    }
    if (div2 < Num::new(3)) {
        div2 = Num::new(3);
    }
    // Calculate the slopes (invslope1 and invslope2)
    let invslope1: Num<i32, 8> = (p3[0] - p1[0]) / (div1);
    let invslope2: Num<i32, 8> = (p3[0] - p2[0]) / (div2);

    // Initialize the starting x-coordinates at the top vertices
    let mut curx1: Num<i32, 8> = (p1[0]);
    let mut curx2: Num<i32, 8> = (p2[0]);

    let mut yTop: i32 = p1[1].trunc();
    let yBottom: i32 = p3[1].trunc();

    let yTopDifference = if yTop < 0 { 0 - yTop } else { 0 };
    yTop += yTopDifference;

    curx1 += invslope1 * yTopDifference;
    curx2 += invslope2 * yTopDifference;

    // Iterate over the scanlines from the top (p1 and p2) down to p3
    for scanline_y in yTop..=yBottom {
        if (scanline_y > 159) {
            break;
        }
        draw_h_line(bitmap4, curx1.trunc(), curx2.trunc(), scanline_y, color);
        curx1 += invslope1;
        curx2 += invslope2;
    }
}

pub fn sort_points(
    p1: &mut [Num<i32, 8>; 2],
    p2: &mut [Num<i32, 8>; 2],
    p3: &mut [Num<i32, 8>; 2],
) {
    // Swap points to ensure p1 has the smallest y, then x
    if (p2[1] < p1[1]) || (p2[1] == p1[1] && p2[0] < p1[0]) {
        for i in 0..2 {
            let temp: Num<i32, 8> = p1[i];
            p1[i] = p2[i];
            p2[i] = temp;
        }
    }
    if (p3[1] < p1[1]) || (p3[1] == p1[1] && p3[0] < p1[0]) {
        for i in 0..2 {
            let temp: Num<i32, 8> = p1[i];
            p1[i] = p3[i];
            p3[i] = temp;
        }
    }
    // Ensure p2 is the middle and p3 is the largest
    if (p3[1] < p2[1]) || (p3[1] == p2[1] && p3[0] < p2[0]) {
        for i in 0..2 {
            let temp: Num<i32, 8> = p2[i];
            p2[i] = p3[i];
            p3[i] = temp;
        }
    }
}

pub fn draw_triangle(
    bitmap4: &mut agb::display::bitmap4::Bitmap4,
    mut p1: [Num<i32, 8>; 2],
    mut p2: [Num<i32, 8>; 2],
    mut p3: [Num<i32, 8>; 2],
    color: u8,
) {
    let zero: Num<i32, 8> = Num::new(0);
    let xMax: Num<i32, 8> = Num::new(240);
    let yMax: Num<i32, 8> = Num::new(160);

    //first check out if the triangle is completely out of view
    if (p1[0] < zero && p2[0] < zero && p3[0] < zero
        || p1[1] < zero && p2[1] < zero && p3[1] < zero
        || p1[0] > xMax && p2[0] > xMax && p3[0] > xMax
        || p1[1] > yMax && p2[1] > yMax && p3[1] > yMax)
    {
        return;
    }

    sort_points(&mut p1, &mut p2, &mut p3);
    //flat top triangle
    if (p1[1] == p2[1]) {
        draw_flat_top_triangle(bitmap4, p1, p2, p3, color);
    }
    //flat bottom triangle
    else if (p2[1] == p3[1]) {
        draw_flat_bottom_triangle(bitmap4, p1, p2, p3, color);
    } else {
        let p4x: Num<i32, 8> = p1[0] + (p2[1] - p1[1]) / (p3[1] - p1[1]) * (p3[0] - p1[0]);
        draw_flat_bottom_triangle(bitmap4, p1, p2, [p4x, p2[1]], color);
        draw_flat_top_triangle(bitmap4, p2, [p4x, p2[1]], p3, color);
    }
}
