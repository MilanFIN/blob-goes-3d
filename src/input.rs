use crate::player;
use agb::input::{Button, ButtonController};
use player::*;

pub fn handle_input(player: &mut Player, input: &ButtonController) {
    if input.is_pressed(Button::UP) && input.is_pressed(Button::LEFT) {
        player.forward_left();
    } else if input.is_pressed(Button::DOWN) && input.is_pressed(Button::LEFT) {
        player.back_left();
    } else if input.is_pressed(Button::UP) && input.is_pressed(Button::RIGHT) {
        player.forward_right();
    } else if input.is_pressed(Button::DOWN) && input.is_pressed(Button::RIGHT) {
        player.back_right();
    } else if input.is_pressed(Button::UP) {
        player.forward();
    } else if input.is_pressed(Button::DOWN) {
        player.back();
    } else if input.is_pressed(Button::LEFT) {
        player.left();
    } else if input.is_pressed(Button::RIGHT) {
        player.right();
    }

    if input.is_pressed(Button::L) {
        player.camera_left(1);
    }
    if input.is_pressed(Button::R) {
        player.camera_right(1);
    }
}
