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

use agb::input::*;

/*
use serde_json_core;
use serde_json_core::*;
use serde::{Deserialize, Serialize};
*/

mod entities;
use cube::Cube;
use empty::Empty;
use entities::*;

mod camera;

mod math;
mod render;
mod utils;
use utils::*;

mod player;
use player::*;

mod input;

mod fixed;
use fixed::*;

mod levels;
use serde_json_core::from_slice;


/*
The main function must take 1 arguments and never return. The agb::entry decorator
ensures that everything is in order. `agb` will call this after setting up the stack
and interrupt handlers correctly. It will also handle creating the `Gba` struct for you.
*/
#[agb::entry]
fn main(mut gba: agb::Gba) -> ! {

    let mut input = ButtonController::new();

    let mut bitmap4: agb::display::bitmap4::Bitmap4 = gba.display.video.bitmap4();

    // Set a palette entries
    bitmap4.set_palette_entry(1, 0x001F);
    bitmap4.set_palette_entry(2, 0x3E0);
    bitmap4.set_palette_entry(3, 0x7C00);

    let mut angle: Fixed = Fixed::const_new(0);
    let increment: Fixed = Fixed::const_new(1) / 256;

    //todo: use these
    let mut entity_array: [EntityEnum; 4] = [EntityEnum::Empty(Empty::default()); 4];
    let mut entity_render_order: [usize; 4] = [0; 4];


    let message_bytes = levels::LEVELS[1].trim().as_bytes();
    let (cubes, _): ([EntityEnum; 2], _) = from_slice(message_bytes).unwrap();


    for i in 0..2 {
        entity_array[i] = EntityEnum::Cube(Cube::default());
        entity_array[i].set_z_offset(new_num(0));
        entity_array[i].set_x_rotation(new_num(0));
        entity_array[i].set_y_rotation(new_num(0));
        entity_array[i].set_z_rotation(new_num(0));
        //always call after modifying rotation

        entity_array[i].set_size(new_num(2));
        entity_array[i].refresh_model_matrix();

        entity_render_order[i] = i;
    }

    for i in 2..4 {
        entity_array[i] = cubes[i-2];
        entity_array[i].set_x_rotation(new_num(0));
        entity_array[i].set_y_rotation(new_num(0));
        entity_array[i].set_z_rotation(new_num(0));
        //always call after modifying rotation

        entity_array[i].recalculate_points();
        entity_array[i].refresh_model_matrix();

        entity_render_order[i] = i;

    }

    //player entities
    entity_array[0].set_size(new_num(1));
    entity_array[0].set_y_offset(new_num(0));
    entity_array[0].set_y_rotation(Fixed::from_raw(64));

    entity_array[0].refresh_model_matrix();

    entity_array[1].set_size(Fixed::from_f32(0.5));
    entity_array[1].set_y_offset(Fixed::from_raw(-192));
    entity_array[1].set_y_rotation(Fixed::from_raw(64));

    entity_array[1].refresh_model_matrix();

    //rest of the blocks
    //entity_array[2].set_x_offset(new_num(5));
    //entity_array[3].set_x_offset(new_num(-5));
    let mut player = Player::default();

    player.camera.set_x_rotation(Fixed::from_raw(0));
    player.camera.set_y_rotation(Fixed::from_raw(0));
    player.camera.set_z_rotation(Fixed::from_raw(0));
    player.camera.local_y = new_num(3);

    player.y = new_num(3);//new_num(5);
    player.z = new_num(0);
    player.camera_left(0);



    loop {
        input.update();

        input::handle_input(&mut player, &input);

        bitmap4.clear(0);
        angle += increment;
        if angle > Fixed::const_new(1) {
            angle = Fixed::const_new(0);
        }

        if player.yspeed <= Fixed::const_new(0) {
            let groundlevel: Fixed = check_support_below(&entity_array, 0);
            player.fall(groundlevel);
        }
        else if player.yspeed > Fixed::const_new(0) {
            let rooflevel: Fixed = check_block_above(&entity_array, 0);
            player.float(rooflevel);    
        }



        player.update_camera_position();

        //rotate player body blocks and move them where the player is
        for i in 0..2 {
            entity_array[i].set_x_offset(player.x);
            entity_array[i].set_y_offset(player.y + Fixed::from_raw(128) + Fixed::from_raw(192) * i);
            entity_array[i].set_z_offset(player.z);

            entity_array[i].set_y_rotation(-player.angle);
            entity_array[i].refresh_model_matrix();
        }

        quick_sort(
            &mut entity_render_order,
            &entity_array,
            0,
            3,
            &player.camera,
        );
        for i in 0..4 {
            entity_array[entity_render_order[i]].render(&mut bitmap4, &player.camera);
        }

        bitmap4.flip_page();
    }
}
