
pub mod lut;
use crate::fixed;
use fixed::*;

pub struct Camera {
    pub local_x: Fixed,
    pub local_y: Fixed,
    pub local_z: Fixed,
    
    pub x: Fixed,
    pub y: Fixed,
    pub z: Fixed,
    pub x_angle: Fixed,
    pub y_angle: Fixed,
    pub z_angle: Fixed,

    pub x_rotation_matrix: [[Fixed; 3]; 3],
    pub y_rotation_matrix: [[Fixed; 3]; 3],
    pub z_rotation_matrix: [[Fixed; 3]; 3],
}

impl Camera {
    pub fn default() -> Self {
        Self {
            local_x: Fixed::const_new(0),
            local_y: Fixed::const_new(0),
            local_z: Fixed::const_new(0),

            x: Fixed::const_new(0),
            y: Fixed::const_new(0),
            z: Fixed::const_new(0),

            x_angle: Fixed::const_new(0),
            y_angle: Fixed::const_new(0),
            z_angle: Fixed::const_new(0),
            
            x_rotation_matrix: [[Fixed::const_new(0); 3]; 3],
            y_rotation_matrix: [[Fixed::const_new(0); 3]; 3],
            z_rotation_matrix: [[Fixed::const_new(0); 3]; 3],
        }
    }

    pub fn set_x_rotation(&mut self, x_angle: Fixed) {
        self.x_angle = x_angle;
        self.x_rotation_matrix = [
            [Fixed::const_new(1), Fixed::const_new(0), Fixed::const_new(0)],
            [Fixed::const_new(0), self.x_angle.cos(), -self.x_angle.sin()],
            [Fixed::const_new(0), self.x_angle.sin(), self.x_angle.cos()],
        ];
    }
    pub fn set_y_rotation(&mut self, y_angle: Fixed) {
        self.y_angle = y_angle;
        self.y_rotation_matrix = [
            [self.y_angle.cos(), Fixed::const_new(0), self.y_angle.sin()],
            [Fixed::const_new(0), Fixed::const_new(1), Fixed::const_new(0)],
            [-self.y_angle.sin(), Fixed::const_new(0), self.y_angle.cos()],
        ];
    }
    pub fn set_z_rotation(&mut self, z_angle: Fixed) {
        self.z_angle = z_angle;
        self.z_rotation_matrix = [
            [self.z_angle.cos(), -self.z_angle.sin(), Fixed::const_new(0)],
            [self.z_angle.sin(), self.z_angle.cos(), Fixed::const_new(0)],
            [Fixed::const_new(0), Fixed::const_new(0), Fixed::const_new(1)],
        ];
    }
}
