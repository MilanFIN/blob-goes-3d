// Games made using `agb` are no_std which means you don't have access to the standard
// rust library. This is because the game boy advance doesn't really have an operating
// system, so most of the content of the standard library doesn't apply.
//
// Provided you haven't disabled it, agb does provide an allocator, so it is possible
// to use both the `core` and the `alloc` built in crates.
#![no_std]
// `agb` defines its own `main` function, so you must declare your game's main function
// using the #[agb::entry] proc macro. Failing to do so will cause failure in linking
// which won't be a particularly clear error message.
#![no_main]
// This is required to allow writing tests
#![cfg_attr(test, feature(custom_test_frameworks))]
#![cfg_attr(test, reexport_test_harness_main = "test_main")]
#![cfg_attr(test, test_runner(agb::test_runner::test_runner))]

use agb::fixnum::num;
use agb::fixnum::Num;
//use agb::fixnum;

fn matmul(matrix: [[Num<i32, 8>; 3]; 3], vector: [Num<i32, 8>; 3]) -> [Num<i32, 8>; 3] {
    let mut result: [Num<i32, 8>; 3] = [Num::new(0); 3];

    for i in 0..3 {
        result[i] = matrix[i][0] * vector[0] + matrix[i][1] * vector[1] + matrix[i][2] * vector[2];
    }

    return result;
}

fn vectorCross(vec1: [Num<i32, 8>; 3], vec2: [Num<i32, 8>; 3]) -> [Num<i32, 8>; 3] {
    let mut result: [Num<i32, 8>; 3] = [Num::new(0); 3];

    // Cross product formula
    result[0] = vec1[1] * vec2[2] - vec1[2] * vec2[1]; // x component
    result[1] = vec1[2] * vec2[0] - vec1[0] * vec2[2]; // y component
    result[2] = vec1[0] * vec2[1] - vec1[1] * vec2[0]; // z component

    return result;
}

fn vectorDot(vec1: [Num<i32, 8>; 3], vec2: [Num<i32, 8>; 3]) -> Num<i32, 8> {
    let mut result: Num<i32, 8> = Num::new(0);

    for i in 0..3 {
        result = result + vec1[i] * vec2[i];
    }

    return result;
}

fn vectorAdd(vec1: [Num<i32, 8>; 3], vec2: [Num<i32, 8>; 3]) -> [Num<i32, 8>; 3] {
    let mut result: [Num<i32, 8>; 3] = [Num::new(0); 3];

    for i in 0..3 {
        result[i] = vec1[i] + vec2[i];
    }

    return result;
}

fn vectorSub(vec1: [Num<i32, 8>; 3], vec2: [Num<i32, 8>; 3]) -> [Num<i32, 8>; 3] {
    let mut result: [Num<i32, 8>; 3] = [Num::new(0); 3];

    for i in 0..3 {
        result[i] = vec1[i] - vec2[i];
    }

    return result;
}

fn draw_line(
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

fn draw_face_outline(
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
fn backFaceCulling(
    points: [[Num<i32, 8>; 3]; 8],
    p1: usize,
    p2: usize,
    p3: usize,
    p4: usize,
) -> bool {
    let v12: [Num<i32, 8>; 3] = vectorSub(points[p2], points[p1]);
    let v23: [Num<i32, 8>; 3] = vectorSub(points[p3], points[p2]);

    let normal: [Num<i32, 8>; 3] = vectorCross(v12, v23);

    let viewDir: [Num<i32, 8>; 3] = [Num::new(0), Num::new(0), Num::new(1)];

    let dotProd: Num<i32, 8> = vectorDot(normal, viewDir);
    if (dotProd < Num::new(-1)) {
        //using threshold other than 0, to account for inaccuracies
        return true;
    } else {
        return false;
    }
}

fn draw_h_line(bitmap4: &mut agb::display::bitmap4::Bitmap4, x1: i32, x2: i32, y: i32, color: u8) {
    // Ensure x1 is less than or equal to x2 for proper iteration
    let (start, end) = if x1 <= x2 { (x1, x2) } else { (x2, x1) };

    for x in start..=end {
        // Draw each point on the horizontal line
        bitmap4.draw_point(x, y, color);
    }
}

fn draw_flat_bottom_triangle(
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

    for scanline_y in p1[1].trunc()..=p2[1].trunc() {
        draw_h_line(bitmap4, curx1.trunc(), curx2.trunc(), scanline_y,color);
        curx1 += invslope1;
        curx2 += invslope2;
    }
}

fn draw_flat_top_triangle(
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

    // Iterate over the scanlines from the top (p1 and p2) down to p3
    for scanline_y in p1[1].trunc()..=p3[1].trunc() {
        draw_h_line(bitmap4, curx1.trunc(), curx2.trunc(), scanline_y, color);
        curx1 += invslope1;
        curx2 += invslope2;
    }
}
fn sort_points(
    mut p1: [Num<i32, 8>; 2],
    mut p2: [Num<i32, 8>; 2],
    mut p3: [Num<i32, 8>; 2],
) -> [[Num<i32, 8>; 2]; 3] {
    // Swap points to ensure p1 has the smallest y, then x
    if (p2[1] < p1[1]) || (p2[1] == p1[1] && p2[0] < p1[0]) {
        let temp: [Num<i32, 8>; 2] = p1;
        p1 = p2;
        p2 = temp;
    }
    if (p3[1] < p1[1]) || (p3[1] == p1[1] && p3[0] < p1[0]) {
        let temp: [Num<i32, 8>; 2] = p1;
        p1 = p3;
        p3 = temp;
    }

    // Ensure p2 is the middle and p3 is the largest
    if (p3[1] < p2[1]) || (p3[1] == p2[1] && p3[0] < p2[0]) {
        let temp: [Num<i32, 8>; 2] = p2;
        p2 = p3;
        p3 = temp;
    }

    // Return the sorted points
    [p1, p2, p3]
}

fn draw_triangle(
    bitmap4: &mut agb::display::bitmap4::Bitmap4,
    p1: [Num<i32, 8>; 2],
    p2: [Num<i32, 8>; 2],
    p3: [Num<i32, 8>; 2],
    color: u8,
) {
    let points: [[Num<i32, 8>; 2]; 3] = sort_points(p1, p2, p3);
    //flat top triangle
    if (points[0][1] == points[1][1]) {
        draw_flat_top_triangle(bitmap4, points[0], points[1], points[2], color);
    }
    //flat bottom triangle
    else if (points[1][1] == points[2][1]) {
        draw_flat_bottom_triangle(bitmap4, points[0], points[1], points[2], color);
    } else {
        //points[1][1] on y sijainti
        let p4x: Num<i32, 8> = points[0][0]
            + (points[1][1] - points[0][1]) / (points[2][1] - points[0][1])
                * (points[2][0] - points[0][0]);
        draw_flat_bottom_triangle(bitmap4, points[0], points[1], [p4x, points[1][1]], color);
        draw_flat_top_triangle(bitmap4, points[1], [p4x, points[1][1]], points[2], color);

    }
}

// The main function must take 1 arguments and never return. The agb::entry decorator
// ensures that everything is in order. `agb` will call this after setting up the stack
// and interrupt handlers correctly. It will also handle creating the `Gba` struct for you.
#[agb::entry]
fn main(mut gba: agb::Gba) -> ! {
    let mut bitmap4: agb::display::bitmap4::Bitmap4 = gba.display.video.bitmap4();

    // Set a palette entry 1
    bitmap4.set_palette_entry(1, 0x001F);
    bitmap4.set_palette_entry(2, 0x3E0);
    bitmap4.set_palette_entry(3, 0x7C00);

    

    let points: [[Num<i32, 8>; 3]; 8] = [
        [Num::new(1), Num::new(1), Num::new(1)],
        [Num::new(-1), Num::new(1), Num::new(1)],
        [Num::new(-1), Num::new(-1), Num::new(1)],
        [Num::new(1), Num::new(-1), Num::new(1)],
        [Num::new(1), Num::new(1), Num::new(-1)],
        [Num::new(-1), Num::new(1), Num::new(-1)],
        [Num::new(-1), Num::new(-1), Num::new(-1)],
        [Num::new(1), Num::new(-1), Num::new(-1)],
    ];

    //constants
    let width: i32 = 240;
    let height: i32 = 160;
    let mut scale: Num<i32, 8> = Num::new(30); //100;
    let middle: [Num<i32, 8>; 2] = [Num::new(width / 2), Num::new(height / 2)]; // x, y
    let mut angle: Num<i32, 8> = Num::from_f32(0.5);
    let increment: Num<i32, 8> = Num::new(1) / 256;

    let translation_z: Num<i32, 8> = Num::new(3);
    let translation_x: Num<i32, 8> = Num::new(0);

    loop {
        bitmap4.clear(0);

        angle += increment;

        if (angle > Num::new(1)) {
            angle = Num::new(0);
        }

        let rotX: [[Num<i32, 8>; 3]; 3] = [
            [Num::new(1), Num::new(0), Num::new(0)],
            [Num::new(0), angle.cos(), -angle.sin()],
            [Num::new(0), angle.sin(), angle.cos()],
        ];

        let rotY: [[Num<i32, 8>; 3]; 3] = [
            [angle.cos(), Num::new(0), angle.sin()],
            [Num::new(0), Num::new(1), Num::new(0)],
            [-angle.sin(), Num::new(0), angle.cos()],
        ];

        let mut screenPoints: [[Num<i32, 8>; 2]; 8] = [[Num::new(0), Num::new(0)]; 8];
        let mut translatedPoints: [[Num<i32, 8>; 3]; 8] =
            [[Num::new(0), Num::new(0), Num::new(0)]; 8];

        let mut i = 0;

        // loop here to not exit
        for point in &points {
            let rotated_point: [Num<i32, 8>; 3] = matmul(rotX, *point);
            let rotated_point: [Num<i32, 8>; 3] = matmul(rotY, rotated_point);

            let mut translated_point: [Num<i32, 8>; 3] = rotated_point;
            translated_point[0] += translation_x;
            translated_point[2] += translation_z;

            //perspective
            let z: Num<i32, 8> = translated_point[2];
            let zero: Num<i32, 8> = Num::new(0);
            let x: Num<i32, 8>;
            let y: Num<i32, 8>;

            if (z != zero) {
                let perspective_scale: Num<i32, 8> = scale / z;
                x = (translated_point[0] * perspective_scale) + middle[0];
                y = (translated_point[1] * perspective_scale) + middle[1];
            } else {
                x = middle[0];
                y = middle[1];
            }

            screenPoints[i] = [x, y];
            translatedPoints[i] = translated_point;
            //bitmap4.draw_point(x.trunc(), y.trunc(), 1);
            i += 1;
        }

        for i in 0..10 {
            let visible: bool = backFaceCulling(translatedPoints, 0, 1, 2, 3);
            if (visible) {
                //draw_face_outline(&mut bitmap4, screenPoints, 0, 1, 2, 3);
                draw_triangle(
                    &mut bitmap4,
                    screenPoints[0],
                    screenPoints[1],
                    screenPoints[2],
                    1
                );
                draw_triangle(
                    &mut bitmap4,
                    screenPoints[0],
                    screenPoints[2],
                    screenPoints[3],
                    1
                );
            }
    
            let visible: bool = backFaceCulling(translatedPoints, 7, 6, 5, 4);
            if (visible) {
                //draw_face_outline(&mut bitmap4, screenPoints, 7, 6, 5, 4);
                draw_triangle(
                    &mut bitmap4,
                    screenPoints[7],
                    screenPoints[6],
                    screenPoints[5],
                    1
                );
                draw_triangle(
                    &mut bitmap4,
                    screenPoints[7],
                    screenPoints[5],
                    screenPoints[4],
                    1
                );
            }
    
            let visible: bool = backFaceCulling(translatedPoints, 0, 3, 7, 4);
            if (visible) {
                //draw_face_outline(&mut bitmap4, screenPoints, 0, 3, 7, 4);
                draw_triangle(
                    &mut bitmap4,
                    screenPoints[0],
                    screenPoints[3],
                    screenPoints[7],
                    2
                );
                draw_triangle(
                    &mut bitmap4,
                    screenPoints[0],
                    screenPoints[7],
                    screenPoints[4],
                    2
                );
            }
    
            let visible: bool = backFaceCulling(translatedPoints, 1, 5, 6, 2);
            if (visible) {
                //draw_face_outline(&mut bitmap4, screenPoints, 1, 5, 6, 2);
                draw_triangle(
                    &mut bitmap4,
                    screenPoints[1],
                    screenPoints[5],
                    screenPoints[6],
                    2
                );
                draw_triangle(
                    &mut bitmap4,
                    screenPoints[1],
                    screenPoints[6],
                    screenPoints[2],
                    2
                );
            }
    
            let visible: bool = backFaceCulling(translatedPoints, 7, 3, 2, 6);
            if (visible) {
                //draw_face_outline(&mut bitmap4, screenPoints, 7, 3, 2, 6);
                draw_triangle(
                    &mut bitmap4,
                    screenPoints[7],
                    screenPoints[3],
                    screenPoints[2],
                    3
                );
                draw_triangle(
                    &mut bitmap4,
                    screenPoints[7],
                    screenPoints[2],
                    screenPoints[6],
                    3
                );
            }
    
            let visible: bool = backFaceCulling(translatedPoints, 0, 4, 5, 1);
            if (visible) {
                //draw_face_outline(&mut bitmap4, screenPoints, 0, 4, 5, 1);
                draw_triangle(
                    &mut bitmap4,
                    screenPoints[0],
                    screenPoints[4],
                    screenPoints[5],
                    2
                );
                draw_triangle(
                    &mut bitmap4,
                    screenPoints[0],
                    screenPoints[5],
                    screenPoints[1],
                    2
                );
            }
    
    
        }
        /*
        draw_triangle(
            &mut bitmap4,
            [Num::new(0), Num::new(0)],
            [Num::new(50), Num::new(50)],
            [Num::new(30), Num::new(70)],
        );*/

        //draw_triangle(&mut bitmap4, [0, 0], [0, 100], [100, 100]);
        //draw_flat_top_triangle(&mut bitmap4, [0, 0], [100, 0], [100, 100]);

        bitmap4.flip_page();
    }
}
