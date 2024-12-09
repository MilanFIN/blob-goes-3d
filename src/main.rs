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

mod entities;
use entities::*;
use cube::Cube;
use empty::Empty;

mod camera;
use camera::*;

mod render;
mod math;

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


    let mut angle: Num<i32, 8> = NewNum(0);
    let increment: Num<i32, 8> = NewNum(1) / 256;

    //todo: use these
    let mut entityArray: [EntityEnum; 2] = [EntityEnum::Empty(Empty::default()); 2];
    for i in 0..2 {
        entityArray[i] = EntityEnum::Cube(Cube::default());
        entityArray[i].set_z_offset(NewNum(0));
        entityArray[i].set_x_rotation(NewNum(0));
        entityArray[i].set_y_rotation(NewNum(0));
        entityArray[i].set_z_rotation(NewNum(0));
        //always call after modifying rotation

    
        entityArray[i].set_size(2);
        entityArray[i].refresh_model_matrix();

    }

    entityArray[0].set_x_offset(NewNum(0));
    entityArray[1].set_x_offset(NewNum(-2));

    let mut camera: Camera = Camera::default();
    camera.set_x_rotation(NewNum(0));
    camera.set_y_rotation(NewNum(0));
    camera.set_z_rotation(NewNum(0));


    loop {
        bitmap4.clear(0);
        angle += increment;
        if (angle > NewNum(1)) {
            angle = NewNum(0);
        }
        //constant vertical offset
        camera.y = NewNum(-3);


        //todo: init into a vector in heap using the ewram external allocator
        //then split into 8, 16 etc...
        camera.set_y_rotation(NewNum(0));
        camera.x = NewNum(0);
        camera.z = NewNum(-3);

        camera.set_y_rotation(NewNum(1)/2);
        camera.x = NewNum(0);
        camera.z = NewNum(3);

        camera.set_y_rotation(NewNum(1)/4);
        camera.x = NewNum(3);
        camera.z = NewNum(0);

        camera.set_y_rotation(NewNum(3)/4);
        camera.x = NewNum(-3);
        camera.z = NewNum(0);

        for i in 0..1 {
            //entityArray[i].set_z_rotation(angle);
            //entityArray[i].set_y_rotation(angle);
            //entityArray[i].refresh_model_matrix();

            entityArray[i].render(&mut bitmap4, &camera);
        }
        
        bitmap4.flip_page();
    }
}
