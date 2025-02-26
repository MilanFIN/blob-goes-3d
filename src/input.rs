use crate::{
    player, utils::GameState, Fixed
};
use agb::input::{Button, ButtonController};
use player::*;


pub fn handle_input(
    player: &mut Player,
    input: &ButtonController,
    game_state: GameState,
) -> GameState {
    let mut new_game_state = game_state;
    if input.is_pressed(Button::L) {
        player.camera_left(2);
    }
    if input.is_pressed(Button::R) {
        player.camera_right(2);
    }

    // if input.is_just_pressed(Button::A) {
    //     player.jump();
    // }
    if input.is_pressed(Button::A) {
        player.try_jumping();
    }
    else {
        player.cancel_jump();
    }

    if input.is_just_pressed(Button::SELECT) {
        player.autorotate_camera = !player.autorotate_camera;
    }

    if input.is_just_pressed(Button::B) {
        player.action = true;
    }

    if input.is_just_pressed(Button::START) {
        new_game_state = GameState::Paused;
    }


    if input.is_pressed(Button::UP) && input.is_pressed(Button::LEFT) {
        let (x, z) = player.forward_left();
        player.move_toward(x, z);
        /*

        if attempt_move(player, x, z, entities, body) {
            return;
        }*/
    }
    else if input.is_pressed(Button::DOWN) && input.is_pressed(Button::LEFT) {
        let (x, z) = player.back_left();
        player.move_toward(x, z);
        /*
        if attempt_move(player, x, z, entities, body) {
            return;
        }
        */
    }
    else if input.is_pressed(Button::UP) && input.is_pressed(Button::RIGHT) {
        let (x, z) = player.forward_right();
        player.move_toward(x, z);
        /*
        if attempt_move(player, x, z, entities, body) {
            return;
        }
        */
    }
    else if input.is_pressed(Button::DOWN) && input.is_pressed(Button::RIGHT) {
        let (x, z) = player.back_right();
        player.move_toward(x, z);
        /*
        if attempt_move(player, x, z, entities, body) {
            return;
        }
        */
    }
    else if input.is_pressed(Button::UP) {
        let (x, z) = player.forward();

        player.move_toward(x, z);
        /*
        if attempt_move(player, x, z, entities, body) {
            return;
        }
        */
    }
    else if input.is_pressed(Button::DOWN) {
        let (x, z) = player.back();
        player.move_toward(x, z);
        /*
        if attempt_move(player, x, z, entities, body) {
            return;
        }
        */
    }
    else if input.is_pressed(Button::LEFT) {
        let (x, z) = player.left();
        player.move_toward(x, z);
        /*
        if attempt_move(player, x, z, entities, body) {
            return;
        }
        */
    }
    else if input.is_pressed(Button::RIGHT) {
        let (x, z) = player.right();
        player.move_toward(x, z);
        /*
        if attempt_move(player, x, z, entities, body) {
            return;
        }
        */
    }
    else {
        player.move_toward(Fixed::const_new(0), Fixed::const_new(0));
    }
    return new_game_state
}
