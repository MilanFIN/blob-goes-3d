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
use agb::input::*;

mod entities;
use entities::*;
use cube::Cube;
use empty::Empty;

mod camera;
use camera::*;
use lut::CAMERALOCATIONS;

mod render;
mod math;
mod utils;
use utils::*;

mod player;
use player::*;


// The main function must take 1 arguments and never return. The agb::entry decorator
// ensures that everything is in order. `agb` will call this after setting up the stack
// and interrupt handlers correctly. It will also handle creating the `Gba` struct for you.
#[agb::entry]
fn main(mut gba: agb::Gba) -> ! {

    let mut input = ButtonController::new();
    

    let mut bitmap4: agb::display::bitmap4::Bitmap4 = gba.display.video.bitmap4();

    // Set a palette entries
    bitmap4.set_palette_entry(1, 0x001F);
    bitmap4.set_palette_entry(2, 0x3E0);
    bitmap4.set_palette_entry(3, 0x7C00);


    let mut angle: Num<i32, 8> = NewNum(0);
    let increment: Num<i32, 8> = NewNum(1) / 256;
    let mut a = 0;

    //todo: use these
    let mut entityArray: [EntityEnum; 4] = [EntityEnum::Empty(Empty::default()); 4];
    let mut entityRenderOrder: [usize; 4] = [0;4];

    for i in 0..4 {
        entityArray[i] = EntityEnum::Cube(Cube::default());
        entityArray[i].set_z_offset(NewNum(0));
        entityArray[i].set_x_rotation(NewNum(0));
        entityArray[i].set_y_rotation(NewNum(0));
        entityArray[i].set_z_rotation(NewNum(0));
        //always call after modifying rotation

        entityArray[i].set_size(NewNum(2));
        entityArray[i].refresh_model_matrix();

        entityRenderOrder[i] = i;
    }

    //player entities
    entityArray[0].set_size(NewNum(1));
    entityArray[0].set_y_offset(NewNum(0));
    entityArray[0].set_y_rotation(Num::from_raw(64));

    entityArray[0].refresh_model_matrix();

    entityArray[1].set_size(Num::from_f32(0.5));
    entityArray[1].set_y_offset(Num::from_raw(-192));
    entityArray[1].set_y_rotation(Num::from_raw(64));

    entityArray[1].refresh_model_matrix();



    //rest of the blocks
    entityArray[2].set_x_offset(NewNum(5));
    entityArray[3].set_x_offset(NewNum(-5));
    let mut player = Player::default();

    player.camera.set_x_rotation(NewNum(0));
    player.camera.set_y_rotation(NewNum(0));
    player.camera.set_z_rotation(NewNum(0));
    player.camera.local_y = NewNum(-3);

    player.x = NewNum(5);

    loop {
        input.update();

        if (input.is_pressed(Button::UP)) {
            player.forward();
        }
        else if (input.is_pressed(Button::DOWN)) {
            player.back();
        }
        if (input.is_pressed(Button::LEFT)) {
            player.left();
        }
        else if (input.is_pressed(Button::RIGHT)) {
            player.right();
        }
        if (input.is_pressed(Button::L)) {
            player.camera_left(1);
        }
        else if (input.is_pressed(Button::R)) {
            player.camera_right(1);
        }

        bitmap4.clear(0);
        angle += increment;
        if (angle > NewNum(1)) {
            angle = NewNum(0);
        }

        player.update_camera_position();

        //rotate player body blocks and move them where the player is
        for i in 0..2 {
            entityArray[i].set_x_offset(player.x);
            entityArray[i].set_z_offset(player.z);
            entityArray[i].set_y_rotation(-player.angle);
            entityArray[i].refresh_model_matrix();
        }

        quick_sort(&mut entityRenderOrder, &entityArray, 0, 3, &player.camera);
        for i in 0..4 {
            entityArray[entityRenderOrder[i]].render(&mut bitmap4, &player.camera);
        }
        
        bitmap4.flip_page();
    }
}
