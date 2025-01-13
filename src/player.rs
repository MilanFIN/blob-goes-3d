use lut::CAMERALOCATIONS;

use crate::camera;
use camera::*;

use crate::fixed;
use fixed::*;

const GRAVITY: Fixed = Fixed::from_raw(32);
const MOVEAMOUNT: Fixed = Fixed::from_raw(64);
pub const JUMPPOWER: Fixed = Fixed::from_raw(256);

pub struct Player {
    pub x: Fixed,
    pub y: Fixed,
    pub z: Fixed,
    pub yspeed: Fixed,
    pub action: bool,

    pub angle: Fixed,
    camera_angle: usize,
    pub camera: Camera,
}

impl Player {
    pub fn default() -> Self {
        Self {
            x: Fixed::const_new(0),
            y: Fixed::const_new(0),
            z: Fixed::const_new(0),
            angle: Fixed::const_new(0),
            yspeed: Fixed::const_new(0),
            camera_angle: 0,
            camera: Camera::default(),
            action: false,
        }
    }

    pub fn move_to(&mut self, x: Fixed, z: Fixed) {
        self.x = self.x + x;
        self.z = self.z + z;
    }

    pub fn forward(&mut self) -> (Fixed, Fixed) {
        let mut view_dir: usize = self.camera_angle + 64;
        if view_dir > 255 {
            view_dir -= 255;
        }
        self.angle = CAMERALOCATIONS[view_dir][2];
        let x: Fixed = self.angle.cos() * MOVEAMOUNT;
        let z: Fixed = self.angle.sin() * MOVEAMOUNT;
        return (x, z);
    }

    pub fn forward_left(&mut self) -> (Fixed, Fixed) {
        let mut view_dir: usize = self.camera_angle + 32;
        if view_dir > 255 {
            view_dir -= 255;
        }
        self.angle = CAMERALOCATIONS[view_dir][2];
        let x = self.angle.cos() * MOVEAMOUNT;
        let z = self.angle.sin() * MOVEAMOUNT;
        return (x, z);
    }

    pub fn forward_right(&mut self) -> (Fixed, Fixed) {
        let mut view_dir: usize = self.camera_angle + 96;
        if view_dir > 255 {
            view_dir -= 255;
        }
        self.angle = CAMERALOCATIONS[view_dir][2];
        let x = self.angle.cos() * MOVEAMOUNT;
        let z = self.angle.sin() * MOVEAMOUNT;
        return (x, z);
    }

    pub fn back(&mut self) -> (Fixed, Fixed) {
        let mut view_dir: usize = self.camera_angle + 192;
        if view_dir > 255 {
            view_dir -= 255;
        }
        self.angle = CAMERALOCATIONS[view_dir][2];
        let x = self.angle.cos() * MOVEAMOUNT;
        let z = self.angle.sin() * MOVEAMOUNT;
        return (x, z);
    }

    pub fn back_left(&mut self) -> (Fixed, Fixed) {
        let mut view_dir: usize = self.camera_angle + 224;
        if view_dir > 255 {
            view_dir -= 255;
        }
        self.angle = CAMERALOCATIONS[view_dir][2];
        let x = self.angle.cos() * MOVEAMOUNT;
        let z = self.angle.sin() * MOVEAMOUNT;
        return (x, z);
    }

    pub fn back_right(&mut self) -> (Fixed, Fixed) {
        let mut view_dir: usize = self.camera_angle + 160;
        if view_dir > 255 {
            view_dir -= 255;
        }
        self.angle = CAMERALOCATIONS[view_dir][2];
        let x = self.angle.cos() * MOVEAMOUNT;
        let z = self.angle.sin() * MOVEAMOUNT;
        return (x, z);
    }

    pub fn left(&mut self) -> (Fixed, Fixed) {
        let mut view_dir: usize = self.camera_angle + 0;
        if view_dir > 255 {
            view_dir -= 255;
        }
        self.angle = CAMERALOCATIONS[view_dir][2];
        let x = self.angle.cos() * MOVEAMOUNT;
        let z = self.angle.sin() * MOVEAMOUNT;
        return (x, z);
    }
    pub fn right(&mut self) -> (Fixed, Fixed) {
        let mut view_dir: usize = self.camera_angle + 128;
        if view_dir > 255 {
            view_dir -= 255;
        }
        self.angle = CAMERALOCATIONS[view_dir][2];
        let x = self.angle.cos() * MOVEAMOUNT;
        let z = self.angle.sin() * MOVEAMOUNT;
        return (x, z);
    }
    pub fn camera_left(&mut self, mut amount: usize) {
        if self.camera_angle < amount {
            amount -= self.camera_angle;
            self.camera_angle = 256;
        }
        self.camera_angle -= amount;
        self.camera
            .set_y_rotation(CAMERALOCATIONS[self.camera_angle][2]);
        self.camera
            .set_x_rotation(CAMERALOCATIONS[self.camera_angle][3]);
        self.camera
            .set_z_rotation(CAMERALOCATIONS[self.camera_angle][4]);

        self.camera.local_x = CAMERALOCATIONS[self.camera_angle][0];
        self.camera.local_z = CAMERALOCATIONS[self.camera_angle][1];
    }
    pub fn camera_right(&mut self, amount: usize) {
        self.camera_angle += amount;
        if self.camera_angle >= 256 {
            self.camera_angle -= 256;
        }
        self.camera
            .set_y_rotation(CAMERALOCATIONS[self.camera_angle][2]);
        self.camera
            .set_x_rotation(CAMERALOCATIONS[self.camera_angle][3]);
        self.camera
            .set_z_rotation(CAMERALOCATIONS[self.camera_angle][4]);

        self.camera.local_x = CAMERALOCATIONS[self.camera_angle][0];
        self.camera.local_z = CAMERALOCATIONS[self.camera_angle][1];
    }

    pub fn update_camera_position(&mut self) {
        self.camera.x = self.camera.local_x + self.x;
        self.camera.y = self.camera.local_y + self.y;
        self.camera.z = self.camera.local_z + self.z;
    }

    pub fn land(&mut self) {
        self.yspeed = Fixed::const_new(0);
    }

    pub fn fall(&mut self, ylimit: Fixed) {
        if self.y > ylimit {
            self.y += self.yspeed;
            if self.y < ylimit {
                self.y = ylimit;
                self.land();
            }
            self.yspeed -= GRAVITY;
        } else {
            self.land();
        }
    }

    pub fn float(&mut self, ylimit: Fixed) {
        //todo: replace 192 with the actual player height, when that starts varying
        let y = self.y + Fixed::from_raw(192);
        if y < ylimit {
            self.y += self.yspeed;
            if self.y + Fixed::from_raw(192) > ylimit {
                self.y = ylimit - Fixed::from_raw(192);
                self.land();
            }
            self.yspeed -= GRAVITY;
        }
    }
}

