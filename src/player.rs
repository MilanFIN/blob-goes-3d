use lut::CAMERALOCATIONS;

use crate::{camera, math::vector_len_2d, utils};
use camera::*;

use crate::fixed;
use fixed::*;

const MOVECAP: Fixed = Fixed::from_raw(64); //64
const GROUNDACCEL: Fixed = Fixed::from_raw(32);
const AIRACCEL: Fixed = Fixed::from_raw(8);
//slows the player down after they let go of Button::A
const FLOATGRAVITY: Fixed = Fixed::from_raw(128);
const BASEGRAVITY: Fixed = Fixed::from_raw(32);
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
    pub autorotate_camera: bool,
    jumping: bool,
    forced_jump: bool,

    pub move_x: Fixed,
    pub move_z: Fixed,
    pub activeaccel: Fixed,
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
            autorotate_camera: true,
            jumping: false,
            forced_jump: false,
            move_x: Fixed::const_new(0),
            move_z: Fixed::const_new(0),
            activeaccel: Fixed::const_new(0),
        }
    }

    pub fn move_to(&mut self, x: Fixed, z: Fixed) {
        self.x = self.x + x;
        self.z = self.z + z;

        //self.move_x = self.x - x;
        //self.move_z = self.z - z;

        if self.autorotate_camera {
            let (dir, diff) = utils::angle_diff(self.camera.y_angle, self.angle);
            if diff > Fixed::from_raw(16) {
                if dir > 0 {
                    self.camera_right(3);
                } else if dir < 0 {
                    self.camera_left(3);
                }
            }
        }
    }

    pub fn forward(&mut self) -> (Fixed, Fixed) {
        let mut view_dir: usize = self.camera_angle + 64;
        if view_dir > 255 {
            view_dir -= 255;
        }
        self.angle = CAMERALOCATIONS[view_dir][2];
        let x: Fixed = self.angle.cos();
        let z: Fixed = self.angle.sin();
        return (x, z);
    }

    pub fn forward_left(&mut self) -> (Fixed, Fixed) {
        let mut view_dir: usize = self.camera_angle + 32;
        if view_dir > 255 {
            view_dir -= 255;
        }
        self.angle = CAMERALOCATIONS[view_dir][2];
        let x = self.angle.cos();
        let z = self.angle.sin();
        return (x, z);
    }

    pub fn forward_right(&mut self) -> (Fixed, Fixed) {
        let mut view_dir: usize = self.camera_angle + 96;
        if view_dir > 255 {
            view_dir -= 255;
        }
        self.angle = CAMERALOCATIONS[view_dir][2];
        let x = self.angle.cos();
        let z = self.angle.sin();
        return (x, z);
    }

    pub fn back(&mut self) -> (Fixed, Fixed) {
        let mut view_dir: usize = self.camera_angle + 192;
        if view_dir > 255 {
            view_dir -= 255;
        }
        self.angle = CAMERALOCATIONS[view_dir][2];
        let x = self.angle.cos();
        let z = self.angle.sin();
        return (x, z);
    }

    pub fn back_left(&mut self) -> (Fixed, Fixed) {
        let mut view_dir: usize = self.camera_angle + 224;
        if view_dir > 255 {
            view_dir -= 255;
        }
        self.angle = CAMERALOCATIONS[view_dir][2];
        let x = self.angle.cos();
        let z = self.angle.sin();
        return (x, z);
    }

    pub fn back_right(&mut self) -> (Fixed, Fixed) {
        let mut view_dir: usize = self.camera_angle + 160;
        if view_dir > 255 {
            view_dir -= 255;
        }
        self.angle = CAMERALOCATIONS[view_dir][2];
        let x = self.angle.cos();
        let z = self.angle.sin();
        return (x, z);
    }

    pub fn left(&mut self) -> (Fixed, Fixed) {
        let mut view_dir: usize = self.camera_angle + 0;
        if view_dir > 255 {
            view_dir -= 255;
        }
        self.angle = CAMERALOCATIONS[view_dir][2];
        let x = self.angle.cos();
        let z = self.angle.sin();
        return (x, z);
    }
    pub fn right(&mut self) -> (Fixed, Fixed) {
        let mut view_dir: usize = self.camera_angle + 128;
        if view_dir > 255 {
            view_dir -= 255;
        }
        self.angle = CAMERALOCATIONS[view_dir][2];
        let x = self.angle.cos();
        let z = self.angle.sin();
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
        self.activeaccel = GROUNDACCEL;
        self.yspeed = Fixed::const_new(0);
    }

    pub fn fall(&mut self, ylimit: Fixed) {
        if self.y > ylimit {
            self.y += self.yspeed;
            if self.y < ylimit {
                self.y = ylimit;
                self.land();
            }
            self.yspeed -= BASEGRAVITY;
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
            if self.jumping || self.forced_jump {
                self.yspeed -= BASEGRAVITY;
            } else {
                self.yspeed -= FLOATGRAVITY;
            }
        }
        self.jumping = false;
    }

    pub fn jump(&mut self) {
        if self.yspeed == Fixed::const_new(0) {
            self.yspeed = JUMPPOWER;
            self.forced_jump = false;
            self.activeaccel = AIRACCEL;
        }
    }

    pub fn keep_jumping(&mut self) {
        self.jumping = true;
    }

    //set active to true, when player also jumps when contacting the platform
    pub fn bounce(&mut self, power: Fixed, active_bounce: bool) {
        self.yspeed = power;
        self.forced_jump = active_bounce;
        self.activeaccel = AIRACCEL;
    }

    pub fn move_toward(&mut self, x: Fixed, z: Fixed) {
        let x_cap = x * MOVECAP;
        let z_cap = z * MOVECAP;


        if self.move_x > x_cap {
            self.move_x -= self.activeaccel;
            if self.move_x < x_cap {
                self.move_x = x_cap;
            }
        } else if self.move_x < x_cap {
            self.move_x += self.activeaccel;
            if self.move_x > x_cap {
                self.move_x = x_cap;
            }
        }

        if self.move_z > z_cap {
            self.move_z -= self.activeaccel;
            if self.move_z < z_cap {
                self.move_z = z_cap;
            }
        } else if self.move_z < z_cap {
            self.move_z += self.activeaccel;
            if self.move_z > z_cap {
                self.move_z = z_cap;
            }
        }

        let len = vector_len_2d([self.move_x, self.move_z]);
        if len > MOVECAP {
            let scale = MOVECAP / len;
            self.move_x *= scale;
            self.move_z *= scale;
        }
    }
}
