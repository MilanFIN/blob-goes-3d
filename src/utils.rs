
use crate::Fixed;

pub const fn new_num(m:i32) -> Fixed {
    return Fixed::from_raw(m << 8);
}

#[allow(dead_code)]
pub fn clamp(n: &mut Fixed, min: &Fixed, max: &Fixed) {
    if *n < *min {
        *n = *min;
    } else if *n > *max {
        *n = *max;
    }
}

pub fn calculate_center(points: &[[Fixed; 2]; 4]) -> [Fixed; 2] {
    let p1: [Fixed; 2] = points[0]; // First point
    let p3: [Fixed; 2] = points[2]; // Diagonal point (opposite corner)

    let center_x: Fixed = (p1[0] + p3[0]) / Fixed::const_new(2);
    let center_z: Fixed = (p1[1] + p3[1]) / Fixed::const_new(2);

    [center_x, center_z]
}


pub fn rectangle_model_points(xsize: Fixed, ysize: Fixed, zsize: Fixed) -> [[Fixed; 3]; 8] {
    let halfx: Fixed = xsize / Fixed::const_new(2);
    let halfy: Fixed = ysize / Fixed::const_new(2);
    let halfz: Fixed = zsize / Fixed::const_new(2);

    [
        [(halfx), (halfy), (halfz)],
        [(-halfx), (halfy), (halfz)],
        [(-halfx), (-halfy), (halfz)],
        [(halfx), (-halfy), (halfz)],
        [(halfx), (halfy), (-halfz)],
        [(-halfx), (halfy), (-halfz)],
        [(-halfx), (-halfy), (-halfz)],
        [(halfx), (-halfy), (-halfz)],
    ]
}

// let angle_diff = utils::angle_diff(player1.camera.y_angle, player1.angle);
pub fn angle_diff(a: Fixed, b: Fixed) -> (i16, Fixed) {

    let b = (b - Fixed::from_raw(64)) % Fixed::const_new(1);

    // Normalize angles to [0, 360)

    // Calculate the clockwise and counterclockwise distances
    let clockwise_distance = (b - a + Fixed::const_new(1)) % Fixed::const_new(1);
    let counterclockwise_distance = (a - b + Fixed::const_new(1)) % Fixed::const_new(1);

    // Determine the shorter direction
    if clockwise_distance <= counterclockwise_distance {
        return (1, clockwise_distance);
    } else {
        return (-1, counterclockwise_distance);
    }
}