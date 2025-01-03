use crate::{boundingshapes::BoundingCylinder, horizontal_collision_check, player, EntityEnum, Fixed};
use agb::input::{Button, ButtonController};
use player::*;

fn attempt_move(
    player: &mut Player,
    x: Fixed,
    z: Fixed,
    entities: &[EntityEnum],
    body: &BoundingCylinder,
) -> bool {
    let potential_position: BoundingCylinder = BoundingCylinder::new_with_offset(body, x, z);
    //check if we can move in both x and z dirs
    let (_wallangle, ok) = horizontal_collision_check(entities, potential_position);
    if !ok {
        player.move_to(x, z);
        return true;
    }
    
    //todo: from the wall angle, calculate two vectors for movement, then we can check if those are available as well
    /*
    let potential_position: BoundingCylinder = BoundingCylinder::new_with_offset(body, x, z);
    if !horizontal_collision_check(entities, potential_position, dir).1 {
        player.move_to(x, Fixed::const_new(0));
        return true;
    }*/

    /*
    //if not, check x and z separately
    let potential_position: BoundingCylinder = BoundingCylinder::new_with_offset(body, x, Fixed::const_new(0));
    if !horizontal_collision_check(entities, potential_position) {
        player.move_to(x, Fixed::const_new(0));
        return true;
    }
    let potential_position: BoundingCylinder = BoundingCylinder::new_with_offset(body, Fixed::const_new(0), z);
    if !horizontal_collision_check(entities, potential_position) {
        player.move_to(Fixed::const_new(0), z);
        return true;
    }
    */


    return false;
}

pub fn handle_input(
    player: &mut Player,
    input: &ButtonController,
    entities: &[EntityEnum],
    body: &BoundingCylinder,
) {

    if input.is_pressed(Button::L) {
        player.camera_left(2);
    }
    if input.is_pressed(Button::R) {
        player.camera_right(2);
    }

    if input.is_just_pressed(Button::A) {
        if player.yspeed == Fixed::const_new(0) {
            player.yspeed = JUMPPOWER;
        }
    }

    if input.is_pressed(Button::UP) && input.is_pressed(Button::LEFT) {
        let (x, z) = player.forward_left();
        if attempt_move(player, x, z, entities, body) {
            return;
        }
    }
    if input.is_pressed(Button::DOWN) && input.is_pressed(Button::LEFT) {
        let (x, z) = player.back_left();
        if attempt_move(player, x, z, entities, body) {
            return;
        }
    }
    if input.is_pressed(Button::UP) && input.is_pressed(Button::RIGHT) {
        let (x, z) = player.forward_right();
        if attempt_move(player, x, z, entities, body) {
            return;
        }
    }
    if input.is_pressed(Button::DOWN) && input.is_pressed(Button::RIGHT) {
        let (x, z) = player.back_right();
        if attempt_move(player, x, z, entities, body) {
            return;
        }
    }
    if input.is_pressed(Button::UP) {
        let (x, z) = player.forward();
        if attempt_move(player, x, z, entities, body) {
            return;
        }
    }
    if input.is_pressed(Button::DOWN) {
        let (x, z) = player.back();
        if attempt_move(player, x, z, entities, body) {
            return;
        }
    }
    if input.is_pressed(Button::LEFT) {
        let (x, z) = player.left();
        if attempt_move(player, x, z, entities, body) {
            return;
        }
    }
    if input.is_pressed(Button::RIGHT) {
        let (x, z) = player.right();
        if attempt_move(player, x, z, entities, body) {
            return;
        }
    }

}
