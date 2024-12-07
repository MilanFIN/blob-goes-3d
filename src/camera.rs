use agb::fixnum::Num;

pub struct Camera {
    pub x: Num<i32, 8>,
    pub y: Num<i32, 8>,
    pub z: Num<i32, 8>,
    pub x_angle: Num<i32, 8>,
    pub y_angle: Num<i32, 8>,
    pub z_angle: Num<i32, 8>,
}

impl Camera {
    pub fn default() -> Self {
        Self {
            x: Num::new(0),
            y: Num::new(0),
            z: Num::new(0),
            x_angle: Num::new(0),
            y_angle: Num::new(0),
            z_angle: Num::new(0),
        }
    }
}
