
use crate::Fixed;

#[allow(dead_code)]
#[derive(PartialEq)]
pub enum GameState {
    //TODO: use menu option, and make player load the next level after finishing
    //canceling from pause menu should return player to main menu, so MENU would be set
    Menu,
    Playing,
    Finished,
    Failed,
    Paused,
}

#[allow(dead_code)]
pub fn clamp<T: PartialOrd + Copy>(n: &mut T, min: T, max: T) {
    if *n < min {
        *n = min;
    } else if *n > max {
        *n = max;
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

pub fn x_rotation_matrix(angle: Fixed) -> [[Fixed; 3]; 3] {

    
    let cos = angle.cos();
    let sin = angle.sin();

    [
        [Fixed::const_new(1), Fixed::const_new(0), Fixed::const_new(0)],
        [Fixed::const_new(0), cos, -sin],
        [Fixed::const_new(0), sin, cos],
    ]
}

pub fn y_rotation_matrix(angle: Fixed) -> [[Fixed; 3]; 3] {

    let cos = angle.cos();
    let sin = angle.sin();

    [
        [cos, Fixed::const_new(0), sin],
        [Fixed::const_new(0), Fixed::const_new(1), Fixed::const_new(0)],
        [-sin, Fixed::const_new(0), cos],
    ]
}

pub fn z_rotation_matrix(angle: Fixed) -> [[Fixed; 3]; 3] {

    let cos = angle.cos();
    let sin = angle.sin();

    [
        [cos, -sin, Fixed::const_new(0)],
        [sin, cos, Fixed::const_new(0)],
        [Fixed::const_new(0), Fixed::const_new(0), Fixed::const_new(1)],
    ]
}

pub fn fixed_array_binary_search(target: Fixed, array: &[Fixed], len: i32) -> i32 {
    let mut low: i32 = 0;
    let mut high: i32 = len - 1;
    while low < high {
        let mid: i32 = (low + high) / 2;
        let mid_val: Fixed = array[mid as usize];

        if mid_val < target {
            low = mid + 1;
        } else if mid_val > target {
            high = mid - 1;
        } else {
            return mid;
        }
    }
    return high;
}

