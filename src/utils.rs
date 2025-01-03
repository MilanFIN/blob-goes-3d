
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
