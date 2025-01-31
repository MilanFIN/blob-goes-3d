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
#![feature(allocator_api)]

use agb::input::*;

extern crate alloc;
use alloc::vec::Vec;

// use agb::ExternalAllocator;
use agb::InternalAllocator;
// use alloc::vec::Vec;

// use serde_json_core;
// use serde_json_core::*;
// use serde::{Deserialize, Serialize};

mod entities;
use cube::Cube;
use effects::OutputEvents;
use empty::Empty;
use entities::utils::{check_block_above, check_support_below};
use entities::*;

mod camera;

mod math;
mod renderer;
mod utils;
use levels::levelstore::LEVELSIZE;
use utils::*;
mod player;
use player::*;
mod input;
mod fixed;
use fixed::*;
mod levels;
mod effects;
mod moveutils;

const DRAWDISTANCE: Fixed = Fixed::const_new(35);
const POLYGON_LIMIT: i16 = 60;


/*
The main function must take 1 arguments and never return. The agb::entry decorator
ensures that everything is in order. `agb` will call this after setting up the stack
and interrupt handlers correctly. It will also handle creating the `Gba` struct for you.
*/
#[agb::entry]
fn main(mut gba: agb::Gba) -> ! {
    use body::Body;
    use renderer::polygon::Polygon;

    let mut input = ButtonController::new();

    let mut bitmap4: agb::display::bitmap4::Bitmap4 = gba.display.video.bitmap4();
    let mut page: u16 = 0;

    renderer::utils::init_palette(&mut bitmap4);

    let mut entity_array: [EntityEnum; LEVELSIZE + 2] =
        [EntityEnum::Empty(Empty::default()); LEVELSIZE + 2];

    //todo: if stack runs out of space, use this instead
    /*
    let mut entity_array: Vec<EntityEnum, ExternalAllocator> = Vec::new_in(ExternalAllocator);
    for i in 0..LEVELSIZE + 2 {
        entity_array.push(EntityEnum::Empty(Empty::default()));
    }*/

    let mut entity_render_order: [usize; LEVELSIZE + 2] = [0; LEVELSIZE + 2];

    let levelsize = levels::load_level(0, &mut entity_array);

    let mut player1: Player = Player::default();

    player1.camera.set_x_rotation(Fixed::from_raw(0));
    player1.camera.set_y_rotation(Fixed::from_raw(0));
    player1.camera.set_z_rotation(Fixed::from_raw(0));
    player1.camera.local_y = new_num(3);

    player1.y = new_num(3);
    player1.z = new_num(0);
    player1.camera_left(0);

    //player body
    entity_array[0] = EntityEnum::Body(Body::default());
    entity_array[0].set_x_rotation(new_num(0));
    entity_array[0].set_y_rotation(new_num(0));
    entity_array[0].set_z_rotation(new_num(0));
    entity_array[0].set_color(1);
    entity_array[0].set_size(new_num(1));
    entity_array[0].recalculate_points();
    entity_array[0].refresh_model_matrix();

    entity_array[1] = EntityEnum::Cube(Cube::default());
    entity_array[1].set_x_rotation(new_num(0));
    entity_array[1].set_y_rotation(new_num(0));
    entity_array[1].set_z_rotation(new_num(0));
    entity_array[1].set_color(1);
    entity_array[1].set_size(Fixed::from_raw(160));
    entity_array[1].recalculate_points();
    entity_array[1].refresh_model_matrix();

    for i in 0..entity_render_order.len() {
        entity_render_order[i] = i;
    }

    let mut event_loop: Vec<OutputEvents, InternalAllocator> = Vec::new_in(InternalAllocator);
    let mut polygons: Vec<Polygon, InternalAllocator> = Vec::new_in(InternalAllocator);
    let mut polygon_indices: Vec<usize, InternalAllocator> = Vec::new_in(InternalAllocator);

    loop {
        input.update();

        input::handle_input(
            &mut player1,
            &input,
        );

        moveutils::attempt_move(&mut player1, &entity_array, &entity_array[0].bounding_cylinder());


        renderer::hw::fill(page, 128);

        let mut bottom_support_id: i16 = -1;
        if player1.yspeed <= Fixed::const_new(0) {
            let (groundlevel, collider_entity) = check_support_below(&entity_array, 0);
            bottom_support_id = collider_entity;
            player1.fall(groundlevel);
        } else if player1.yspeed > Fixed::const_new(0) {
            let rooflevel: Fixed = check_block_above(&entity_array, 0);
            player1.float(rooflevel);
        }

        player1.update_camera_position();
        /*
        quick_sort(
            &mut entity_render_order,
            &entity_array,
            0,
            levelsize + 1,
            &player1.camera,
        );*/

        let game_state: effects::InputGameState = effects::InputGameState {
            support_below_id: bottom_support_id,
            bounding_box: &entity_array[0].bounding_box(),
            bounding_cylinder: &entity_array[0].bounding_cylinder(),
            action_requested: player1.action,
            yspeed: player1.yspeed,
        };
        for i in 0..levelsize + 2 {
            if let EntityEnum::Empty(_) = entity_array[i] {
                break;
            }
            if let Some(event) = entity_array[i].tick(&game_state) {
                event_loop.push(event);
            }
        }

        for event in event_loop.iter() {
            if let OutputEvents::PlayerEvent(event) = event {
                player1.x += event.move_x;
                player1.y += event.move_y;
                player1.z += event.move_z;
            } else if let OutputEvents::GameFinish(_event) = event {
                //TODO: actually implement level finishes at some point
                agb::println!("finished");
            } else if let OutputEvents::SwitchAction(_event) = event {
                for i in 2..levelsize + 2 {
                    if let EntityEnum::Wireframe(w) = &mut entity_array[i] {
                        w.toggle();
                    }
                }
            }
            else if let OutputEvents::BounceEvent(event) = event {
                player1.bounce(event.power, input.is_pressed(Button::A));
            }
            else if let OutputEvents::Sliding(event) = event {
                player1.sliding(event.acceleration);
            }
        }
        event_loop.clear();
        player1.tick();


        //update player position on screen
        entity_array[0].set_y_offset(player1.y + entity_array[0].get_height() / 2);
        entity_array[1].set_y_offset(
            player1.y + entity_array[0].get_height() + entity_array[1].get_height() / 2,
        );
        /*entity_array[0]
        .set_y_offset(player1.y + Fixed::from_raw(128) + Fixed::from_raw(192) * i);*/

        //rotate player body blocks and move them where the player is
        for i in 0..2 {
            entity_array[i].set_x_offset(player1.x);
            entity_array[i].set_z_offset(player1.z);

            entity_array[i].set_y_rotation(-player1.angle);
            entity_array[i].refresh_model_matrix();
        }

        for i in 0..levelsize + 2 {
            if let EntityEnum::Empty(_) = entity_array[i] {
                break;
            }
            entity_array[entity_render_order[i]].render(&player1.camera, &mut polygons, DRAWDISTANCE);
        }
        for i in 0..polygons.len() {
            polygon_indices.push(i);
        }
        polygon_indices.sort_by(|&a, &b| {
            polygons[b]
                .distance_from_camera
                .cmp(&polygons[a].distance_from_camera)
        });


        let mut start:i16 = polygon_indices.len() as i16 - POLYGON_LIMIT ;
        if start < 0 {
            start = 0;
        }

        for p in start as usize..polygon_indices.len() {
            let polygon = &polygons[polygon_indices[p]];
            if let Some(vertices) = polygon.as_triangle() {
                renderer::draw::draw_triangle(vertices[0], vertices[1], vertices[2], polygon.color, page);
            }
            if let Some(vertices) = polygon.as_line() {
                renderer::draw::draw_line_fixed(
                    vertices[0][0],
                    vertices[0][1],
                    vertices[1][0],
                    vertices[1][1],
                    polygon.color,
                    page,
                );
            }
        }
        polygons.clear();
        polygon_indices.clear();

        //utils::angle_diff(player1.camera.y_angle, player1.angle);

        renderer::hw::flip(&mut page);
    }
}
