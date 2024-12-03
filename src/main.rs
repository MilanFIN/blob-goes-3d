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

use agb::fixnum::Num;

mod math;
use math::*;

mod render;
use render::*;

mod entities;
use entities::*;

// The main function must take 1 arguments and never return. The agb::entry decorator
// ensures that everything is in order. `agb` will call this after setting up the stack
// and interrupt handlers correctly. It will also handle creating the `Gba` struct for you.
#[agb::entry]
fn main(mut gba: agb::Gba) -> ! {
    let mut bitmap4: agb::display::bitmap4::Bitmap4 = gba.display.video.bitmap4();

    // Set a palette entries
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
    let scale: Num<i32, 8> = Num::new(30); //100;
    let middle: [Num<i32, 8>; 2] = [Num::new(width / 2), Num::new(height / 2)]; // x, y
    let mut angle: Num<i32, 8> = Num::from_f32(0.5);
    let increment: Num<i32, 8> = Num::new(1) / 256;

    let translation_z: Num<i32, 8> = Num::new(3);
    let translation_x: Num<i32, 8> = Num::new(0);
    let translation_y: Num<i32, 8> = Num::new(0);

    let backfaceCullingThreshold: Num<i32, 8> = Num::from_f32(1.2);

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

        let rotZ: [[Num<i32, 8>; 3]; 3] = [
            [angle.cos(), -angle.sin(), Num::new(0)],
            [angle.sin(), angle.cos(), Num::new(0)],
            [Num::new(0), Num::new(0), Num::new(1)],
        ];

        let mut screenPoints: [[Num<i32, 8>; 2]; 8] = [[Num::new(0), Num::new(0)]; 8];
        let mut translatedPoints: [[Num<i32, 8>; 3]; 8] =
            [[Num::new(0), Num::new(0), Num::new(0)]; 8];

        let mut i = 0;

        // loop here to not exit
        for point in &points {
            let mut rotated_point: [Num<i32, 8>; 3] = matmul(rotX, *point);
            rotated_point = matmul(rotY, rotated_point);
            rotated_point = matmul(rotZ, rotated_point);

            let mut translated_point: [Num<i32, 8>; 3] = rotated_point;
            translated_point[0] += translation_x;
            translated_point[1] += translation_y;
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
            i += 1;
        }

        for i in 0..1 {
            let angle: Num<i32, 8> = backFaceCulling(&translatedPoints, 0, 1, 2, 3);
            if (angle < -backfaceCullingThreshold) {
                //draw_face_outline(&mut bitmap4, screenPoints, 0, 1, 2, 3);
                draw_triangle(
                    &mut bitmap4,
                    screenPoints[0],
                    screenPoints[1],
                    screenPoints[2],
                    1,
                );
                draw_triangle(
                    &mut bitmap4,
                    screenPoints[0],
                    screenPoints[2],
                    screenPoints[3],
                    1,
                );
            } else if (angle > backfaceCullingThreshold) {
                //draw_face_outline(&mut bitmap4, screenPoints, 7, 6, 5, 4);
                draw_triangle(
                    &mut bitmap4,
                    screenPoints[7],
                    screenPoints[6],
                    screenPoints[5],
                    1,
                );
                draw_triangle(
                    &mut bitmap4,
                    screenPoints[7],
                    screenPoints[5],
                    screenPoints[4],
                    1,
                );
            }

            let angle: Num<i32, 8> = backFaceCulling(&translatedPoints, 0, 3, 7, 4);
            if (angle < -backfaceCullingThreshold) {
                //draw_face_outline(&mut bitmap4, screenPoints, 0, 3, 7, 4);
                draw_triangle(
                    &mut bitmap4,
                    screenPoints[0],
                    screenPoints[3],
                    screenPoints[7],
                    2,
                );
                draw_triangle(
                    &mut bitmap4,
                    screenPoints[0],
                    screenPoints[7],
                    screenPoints[4],
                    2,
                );
            }

            else if (angle > backfaceCullingThreshold) {
                //draw_face_outline(&mut bitmap4, screenPoints, 1, 5, 6, 2);
                draw_triangle(
                    &mut bitmap4,
                    screenPoints[1],
                    screenPoints[5],
                    screenPoints[6],
                    2,
                );
                draw_triangle(
                    &mut bitmap4,
                    screenPoints[1],
                    screenPoints[6],
                    screenPoints[2],
                    2,
                );
            }

            let angle: Num<i32, 8> = backFaceCulling(&translatedPoints, 7, 3, 2, 6);
            if (angle < -backfaceCullingThreshold) {
                //draw_face_outline(&mut bitmap4, screenPoints, 7, 3, 2, 6);
                draw_triangle(
                    &mut bitmap4,
                    screenPoints[7],
                    screenPoints[3],
                    screenPoints[2],
                    3,
                );
                draw_triangle(
                    &mut bitmap4,
                    screenPoints[7],
                    screenPoints[2],
                    screenPoints[6],
                    3,
                );
            }

            else if (angle > backfaceCullingThreshold) {
                //draw_face_outline(&mut bitmap4, screenPoints, 0, 4, 5, 1);
                draw_triangle(
                    &mut bitmap4,
                    screenPoints[0],
                    screenPoints[4],
                    screenPoints[5],
                    3,
                );
                draw_triangle(
                    &mut bitmap4,
                    screenPoints[0],
                    screenPoints[5],
                    screenPoints[1],
                    3,
                );
            }
        }

        bitmap4.flip_page();
    }
}
