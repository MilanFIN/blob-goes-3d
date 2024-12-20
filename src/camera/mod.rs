use agb::fixnum::Num;

use crate::new_num;

pub mod lut;

pub struct Camera {
    pub local_x: Num<i32, 8>,
    pub local_y: Num<i32, 8>,
    pub local_z: Num<i32, 8>,
    
    pub x: Num<i32, 8>,
    pub y: Num<i32, 8>,
    pub z: Num<i32, 8>,
    pub x_angle: Num<i32, 8>,
    pub y_angle: Num<i32, 8>,
    pub z_angle: Num<i32, 8>,

    pub x_rotation_matrix: [[Num<i32, 8>; 3]; 3],
    pub y_rotation_matrix: [[Num<i32, 8>; 3]; 3],
    pub z_rotation_matrix: [[Num<i32, 8>; 3]; 3],
}

impl Camera {
    pub fn default() -> Self {
        Self {
            local_x: new_num(0),
            local_y: new_num(0),
            local_z: new_num(0),

            x: Num::new(0),
            y: Num::new(0),
            z: Num::new(0),

            x_angle: Num::new(0),
            y_angle: Num::new(0),
            z_angle: Num::new(0),
            
            x_rotation_matrix: [[Num::new(0); 3]; 3],
            y_rotation_matrix: [[Num::new(0); 3]; 3],
            z_rotation_matrix: [[Num::new(0); 3]; 3],
        }
    }

    pub fn set_x_rotation(&mut self, x_angle: Num<i32, 8>) {
        self.x_angle = x_angle;
        self.x_rotation_matrix = [
            [Num::new(1), Num::new(0), Num::new(0)],
            [Num::new(0), self.x_angle.cos(), -self.x_angle.sin()],
            [Num::new(0), self.x_angle.sin(), self.x_angle.cos()],
        ];
    }
    pub fn set_y_rotation(&mut self, y_angle: Num<i32, 8>) {
        self.y_angle = y_angle;
        self.y_rotation_matrix = [
            [self.y_angle.cos(), Num::new(0), self.y_angle.sin()],
            [Num::new(0), Num::new(1), Num::new(0)],
            [-self.y_angle.sin(), Num::new(0), self.y_angle.cos()],
        ];
    }
    pub fn set_z_rotation(&mut self, z_angle: Num<i32, 8>) {
        self.z_angle = z_angle;
        self.z_rotation_matrix = [
            [self.z_angle.cos(), -self.z_angle.sin(), Num::new(0)],
            [self.z_angle.sin(), self.z_angle.cos(), Num::new(0)],
            [Num::new(0), Num::new(0), Num::new(1)],
        ];
    }
}
