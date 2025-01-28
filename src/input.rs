use crate::{
    boundingshapes::BoundingCylinder,
    entities::utils::horizontal_collision_check,
    math::{directional_vector_2d, vector_len_2d},
    player, EntityEnum, Fixed,
};
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
    let (wallangle, collision) = horizontal_collision_check(entities, potential_position);
    if !collision {
        player.move_to(x, z);
        return true;
    }

    // from the wall angle, get the 4 directions of the wall
    // get the two directions with the smallest deviation from the player facing direction
    // then we can check if those two directions are collision free

    let movement_mount: Fixed = vector_len_2d([x, z]);

    let (mut possible_directions, speeds) = get_dirs_by_wall_angle(player.angle, wallangle);
    possible_directions[0][0] *= movement_mount * speeds[0];
    possible_directions[0][1] *= movement_mount * speeds[0];
    possible_directions[1][0] *= movement_mount * speeds[1];
    possible_directions[1][1] *= movement_mount * speeds[1];

    let potential_position: BoundingCylinder = BoundingCylinder::new_with_offset(
        body,
        possible_directions[0][0],
        possible_directions[0][1],
    );
    if !horizontal_collision_check(entities, potential_position).1 {
        player.move_to(possible_directions[0][0], possible_directions[0][1]);
        return true;
    }

    let potential_position: BoundingCylinder = BoundingCylinder::new_with_offset(
        body,
        possible_directions[1][0],
        possible_directions[1][1],
    );
    if !horizontal_collision_check(entities, potential_position).1 {
        player.move_to(possible_directions[1][0], possible_directions[1][1]);
        return true;
    }

    return false;
}

fn get_dirs_by_wall_angle(player_angle: Fixed, wall_angle: Fixed) -> ([[Fixed; 2]; 2], [Fixed; 2]) {
    let direction_options: [Fixed; 4] = [
        wall_angle.modulo(Fixed::const_new(1)),
        (wall_angle + Fixed::from_raw(64)).modulo(Fixed::const_new(1)),
        (wall_angle + Fixed::from_raw(128)).modulo(Fixed::const_new(1)),
        (wall_angle + Fixed::from_raw(192)).modulo(Fixed::const_new(1)),
    ];
    let mut diffs: [Fixed; 4] = [
        (player_angle - direction_options[0]).abs(),
        (player_angle - direction_options[1]).abs(),
        (player_angle - direction_options[2]).abs(),
        (player_angle - direction_options[3]).abs(),
    ];

    for diff in diffs.iter_mut() {
        if *diff > Fixed::from_raw(128) {
            *diff = Fixed::const_new(1) - *diff;
        }
    }

    // Initialize with indices, not values
    let mut closest_indices = [0, 1];

    for (i, &diff) in diffs.iter().enumerate() {
        if diff < diffs[closest_indices[0]] {
            closest_indices[1] = closest_indices[0];
            closest_indices[0] = i;
        } else if diff < diffs[closest_indices[1]] {
            closest_indices[1] = i;
        }
    }

    //now, a parallel movement (0 deg) should be 100% of the original speed,
    //and perpendicular (90deg or 64/256 Fixed) should be 0%,
    // 45 degrees should be 50% etc.
    //calculate the corresponding speed multipliers

    let mut a: Fixed = Fixed::const_new(0);
    let mut b: Fixed = Fixed::const_new(0);

    if diffs[closest_indices[0]] < Fixed::from_raw(64) {
        a = (Fixed::from_raw(64) - diffs[closest_indices[0]]) * 4;
    }
    if diffs[closest_indices[1]] < Fixed::from_raw(64) {
        b = (Fixed::from_raw(64) - diffs[closest_indices[1]]) * 4;
    }

    return (
        [
            directional_vector_2d(direction_options[closest_indices[0]]),
            directional_vector_2d(direction_options[closest_indices[1]]),
        ],
        [a, b],
    );
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
        player.jump();
    }
    if input.is_pressed(Button::A) {
        player.keep_jumping();
    }

    if input.is_just_pressed(Button::SELECT) {
        player.autorotate_camera = !player.autorotate_camera;
    }

    if input.is_just_pressed(Button::B) {
        player.action = true;
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
