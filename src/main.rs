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

use agb::InternalAllocator;

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
mod fixed;
mod input;
use fixed::*;
mod audio;
mod effects;
mod levels;
mod mathlut;
mod menu;
mod moveutils;
mod save;
mod textengine;
use body::Body;
use entities::boundingshapes::{BoundingBox, BoundingShape};
use renderer::polygon::Polygon;

const DRAWDISTANCE: Fixed = Fixed::const_new(35);
const POLYGON_LIMIT: i16 = 60;
//IMPORTANT: if flashing to real hardware, set save type to match the memory type of the cartridge
const SAVE_TYPE: save::SaveType = save::SaveType::Flash64K;//None;
const FLOOR_LEVEL: Fixed = Fixed::const_new(-500);

/*
The main function must take 1 arguments and never return. The agb::entry decorator
ensures that everything is in order. `agb` will call this after setting up the stack
and interrupt handlers correctly. It will also handle creating the `Gba` struct for you.
*/
#[agb::entry]
fn main(mut gba: agb::Gba) -> ! {
    save::init_save(&mut gba, SAVE_TYPE);

    const LEVEL_COUNT: usize = levels::levelstore::LEVELS.len();

    let mut completed_levels: Vec<bool, InternalAllocator> =
        Vec::with_capacity_in(LEVEL_COUNT, InternalAllocator);
    completed_levels.resize(LEVEL_COUNT, true);
    //save::store_save(&mut gba, &mut completed_levels);

    match save::read_save(&mut gba, levels::levelstore::LEVELS.len(), SAVE_TYPE) {
        Ok(saved_data) => {
            completed_levels = saved_data;
        }
        Err(_) => {
            agb::println!("Failed to read save data");
        }
    }

    let mut input = ButtonController::new();

    let mut bitmap4: agb::display::bitmap4::Bitmap4 = gba.display.video.bitmap4();
    let mut page: u16 = 0;
    renderer::utils::init_palette(&mut bitmap4);
    gba.sound.enable();
    let vblank: agb::interrupt::VBlank = agb::interrupt::VBlank::get();
    let mut entity_array: [EntityEnum; LEVELSIZE + 2] =
        [EntityEnum::Empty(Empty::default()); LEVELSIZE + 2];
    let mut entity_render_order: [usize; LEVELSIZE + 2] = [0; LEVELSIZE + 2];

    menu::presstart(&mut input, &mut page);
    audio::play_sound(6, &vblank, &gba.sound);

    let mut selected_level: usize = 0;
    let mut canceled: bool;

    let mut game_state = GameState::Menu;
    let mut camera_follow = true;

    //TODO: enable this when the game is finished
    loop {

        if selected_level >= LEVEL_COUNT {
            selected_level = 0;
            game_state = GameState::Menu;
        }

        if game_state == GameState::Menu {
            let option = menu::mainmenu(&mut input, &mut page, &vblank, &gba.sound);
            if option == 1 {
                audio::play_sound(6, &vblank, &gba.sound);
                menu::info(&mut input, &mut page);
                audio::play_sound(4, &vblank, &gba.sound);
                continue;
            } else {
                //pass
                audio::play_sound(6, &vblank, &gba.sound);
            }

            (selected_level, canceled) = menu::levelmenu(
                selected_level,
                &mut input,
                &mut page,
                &vblank,
                &gba.sound,
                &completed_levels,
            );
            if canceled {
                audio::play_sound(4, &vblank, &gba.sound);
                continue;
            }
            audio::play_sound(6, &vblank, &gba.sound);
        }
    //disable this to use the actual level selected in the menu
        //selected_level = 4;

        let levelsize = levels::load_level(selected_level, &mut entity_array);

        let mut player1: Player = Player::default();
        player1.autorotate_camera = camera_follow;
        player1.init(&vblank, &gba.sound);

        player1.camera.set_x_rotation(Fixed::from_raw(0));
        player1.camera.set_y_rotation(Fixed::from_raw(0));
        player1.camera.set_z_rotation(Fixed::from_raw(0));
        player1.camera.local_y = Fixed::const_new(3);

        player1.y = Fixed::const_new(3);
        player1.z = Fixed::const_new(0);
        player1.camera_left(0);

        //player body consists of entities 0 and 1
        entity_array[0] = EntityEnum::Body(Body::default());
        entity_array[0].set_x_rotation(Fixed::const_new(0));
        entity_array[0].set_y_rotation(Fixed::const_new(0));
        entity_array[0].set_z_rotation(Fixed::const_new(0));
        entity_array[0].set_color(1);
        entity_array[0].set_size(Fixed::const_new(1));
        entity_array[0].recalculate_points();
        entity_array[0].refresh_model_matrix();

        entity_array[1] = EntityEnum::Cube(Cube::default());
        entity_array[1].set_x_rotation(Fixed::const_new(0));
        entity_array[1].set_y_rotation(Fixed::const_new(0));
        entity_array[1].set_z_rotation(Fixed::const_new(0));
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

        game_state = GameState::Playing;

        while game_state == GameState::Playing || game_state == GameState::CompleteAnimation {

            camera_follow = player1.autorotate_camera;
            
            if game_state == GameState::Playing {
                input.update();

                game_state = input::handle_input(&mut player1, &input, game_state);

                moveutils::attempt_move(
                    &mut player1,
                    &entity_array,
                    &entity_array[0].bounding_cylinder(),
                );


                let mut player_box = BoundingBox::default();
                let shape: &Option<boundingshapes::BoundingShape> =
                    &entity_array[0].bounding_shape();
                if let BoundingShape::BoundingBox(shape) = shape.as_ref().unwrap() {
                    player_box = (*shape).clone();
                }

                let player_cylinder = entity_array[0].bounding_cylinder();

                let mut bottom_support_id: i16 = -1;
                if player1.yspeed <= Fixed::const_new(0) {
                    let (groundlevel, collider_entity) =
                        check_support_below(&entity_array, &player_box, &player_cylinder);
                    bottom_support_id = collider_entity;
                    player1.fall(groundlevel);
                } else if player1.yspeed > Fixed::const_new(0) {
                    let rooflevel: Fixed =
                        check_block_above(&entity_array, &player_box, &player_cylinder);
                    player1.float(rooflevel);
                }

                player1.update_camera_position();

                let input_game_state: effects::InputGameState = effects::InputGameState {
                    support_below_id: bottom_support_id,
                    bounding_box: &player_box,
                    bounding_cylinder: &player_cylinder,
                    action_requested: player1.action,
                    yspeed: player1.yspeed,
                };
                for i in 0..levelsize + 2 {
                    if let EntityEnum::Empty(_) = entity_array[i] {
                        break;
                    }
                    if let Some(event) = entity_array[i].tick(&input_game_state) {
                        event_loop.push(event);
                    }
                }

                for event in event_loop.iter() {
                    if let OutputEvents::PlayerEvent(event) = event {
                        player1.x += event.move_x;
                        player1.y += event.move_y;
                        player1.z += event.move_z;
                    } else if let OutputEvents::GameFinish(_event) = event {
                        audio::play_sound(5, &vblank, &gba.sound);
                        game_state = GameState::CompleteAnimation;
                        player1.finish_animation();
                        completed_levels[selected_level] = true;
                        selected_level += 1;
                        audio::play_sound(7, &vblank, &gba.sound);
                    } else if let OutputEvents::SwitchAction(_event) = event {
                        for i in 2..levelsize + 2 {
                            if let EntityEnum::Wireframe(w) = &mut entity_array[i] {
                                w.toggle();
                            }
                            audio::play_sound(1, &vblank, &gba.sound);
                        }
                    } else if let OutputEvents::BounceEvent(event) = event {
                        player1.bounce(event.power, input.is_pressed(Button::A));
                    } else if let OutputEvents::Sliding(event) = event {
                        player1.sliding(event.acceleration);
                    }
                }

                if player1.y < FLOOR_LEVEL {
                    game_state = GameState::Failed;
                }

                event_loop.clear();
                player1.tick();
            }

            else if game_state == GameState::CompleteAnimation {
                game_state = player1.next_animation_frame();
            }

            //update player model position on screen
            entity_array[0].set_y_offset(player1.y + entity_array[0].get_height() / 2);
            entity_array[1].set_y_offset(
                player1.y + entity_array[0].get_height() + entity_array[1].get_height() / 2,
            );
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
                entity_array[entity_render_order[i]].render(
                    &player1.camera,
                    &mut polygons,
                    DRAWDISTANCE,
                );
            }
            for i in 0..polygons.len() {
                polygon_indices.push(i);
            }
            polygon_indices.sort_by(|&a, &b| {
                polygons[b]
                    .distance_from_camera
                    .cmp(&polygons[a].distance_from_camera)
            });

            let mut start: i16 = polygon_indices.len() as i16 - POLYGON_LIMIT;
            if start < 0 {
                start = 0;
            }


            renderer::hw::fill(page, 128);
            renderer::render::render_polygons(&polygons, &polygon_indices, start as usize, page);

            if game_state == GameState::Paused {
                renderer::hw::flip(&mut page);
                //must draw again to update both screens to match
                renderer::render::render_polygons(
                    &polygons,
                    &polygon_indices,
                    start as usize,
                    page,
                );
                renderer::hw::flip(&mut page);
                game_state = menu::pause(&mut input, &mut page, &vblank, &gba.sound);
            }

            polygons.clear();
            polygon_indices.clear();

            renderer::hw::flip(&mut page);
        }
        let _ = save::store_save(&mut gba, &mut completed_levels, SAVE_TYPE);
    }
}
