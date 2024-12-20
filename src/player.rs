use agb::fixnum::Num;
use lut::CAMERALOCATIONS;

use crate::{camera, new_num};
use camera::*;

pub struct Player {
    pub x: Num<i32, 8>,
    pub y: Num<i32, 8>,
    pub z: Num<i32, 8>,

    pub angle: Num<i32, 8>,
    camera_angle: usize,
    pub camera: Camera,
}

impl Player {
    pub fn default() -> Self {
        Self {
            x: new_num(0),
            y: new_num(0),
            z: new_num(0),
            angle: new_num(0),
            camera_angle: 0,
            camera: Camera::default(),
        }
    }
    pub fn forward(&mut self) {
        let mut view_dir: usize = self.camera_angle + 64;
        if view_dir > 255 {
            view_dir -= 255;
        }
        self.angle = CAMERALOCATIONS[view_dir][2];
        self.x += self.angle.cos();
        self.z += self.angle.sin();
    }

    pub fn forward_left(&mut self) {
        let mut view_dir: usize = self.camera_angle + 96;
        if view_dir > 255 {
            view_dir -= 255;
        }
        self.angle = CAMERALOCATIONS[view_dir][2];
        self.x += self.angle.cos();
        self.z += self.angle.sin();
    }

    pub fn forward_right(&mut self) {
        let mut view_dir: usize = self.camera_angle + 32;
        if view_dir > 255 {
            view_dir -= 255;
        }
        self.angle = CAMERALOCATIONS[view_dir][2];
        self.x += self.angle.cos();
        self.z += self.angle.sin();
    }

    pub fn back(&mut self) {
        let mut view_dir: usize = self.camera_angle + 192;
        if view_dir > 255 {
            view_dir -= 255;
        }
        self.angle = CAMERALOCATIONS[view_dir][2];
        self.x += self.angle.cos();
        self.z += self.angle.sin();
    }

    pub fn back_left(&mut self) {
        let mut view_dir: usize = self.camera_angle + 160;
        if view_dir > 255 {
            view_dir -= 255;
        }
        self.angle = CAMERALOCATIONS[view_dir][2];
        self.x += self.angle.cos();
        self.z += self.angle.sin();
    }

	pub fn back_right(&mut self) {
        let mut view_dir: usize = self.camera_angle + 224;
        if view_dir > 255 {
            view_dir -= 255;
        }
        self.angle = CAMERALOCATIONS[view_dir][2];
        self.x += self.angle.cos();
        self.z += self.angle.sin();
    }

    pub fn left(&mut self) {
        self.angle = CAMERALOCATIONS[self.camera_angle][2];
        self.x -= self.angle.cos();
        self.z -= self.angle.sin();
    }
    pub fn right(&mut self) {
        self.angle = CAMERALOCATIONS[self.camera_angle][2];
        self.x += self.angle.cos();
        self.z += self.angle.sin();
    }
    pub fn camera_left(&mut self, amount: usize) {
        self.camera_angle += amount;
        if self.camera_angle >= 256 {
            self.camera_angle -= 256;
        }
        self.camera
            .set_y_rotation(CAMERALOCATIONS[self.camera_angle][2]);
        self.camera.local_x = CAMERALOCATIONS[self.camera_angle][0];
        self.camera.local_z = CAMERALOCATIONS[self.camera_angle][1];
    }
    pub fn camera_right(&mut self, mut amount: usize) {
        if self.camera_angle < amount {
            amount -= self.camera_angle;
            self.camera_angle = 256;
        }
        self.camera_angle -= amount;
        self.camera
            .set_y_rotation(CAMERALOCATIONS[self.camera_angle][2]);
        self.camera.local_x = CAMERALOCATIONS[self.camera_angle][0];
        self.camera.local_z = CAMERALOCATIONS[self.camera_angle][1];
    }

    pub fn update_camera_position(&mut self) {
        self.camera.x = self.camera.local_x + self.x;
        self.camera.y = self.camera.local_y + self.y;
        self.camera.z = self.camera.local_z + self.z;
    }
}
