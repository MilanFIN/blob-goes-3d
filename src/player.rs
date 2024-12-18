use agb::fixnum::{Num};
use lut::CAMERALOCATIONS;

use crate::{camera, NewNum};
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
			x: NewNum(0),
			y: NewNum(0),
			z: NewNum(0),
			angle: NewNum(0),
			camera_angle: 0,
			camera: Camera::default()
		}
	}
	pub fn forward(&mut self) {
		let mut viewDir: usize = self.camera_angle + 64;
		if (viewDir > 255) {
			viewDir -= 255;
		}
		self.angle = CAMERALOCATIONS[viewDir][2];
		self.x += self.angle.cos();
		self.z += self.angle.sin();
	}
	
	pub fn back(&mut self) {
		let mut viewDir: usize = self.camera_angle + 64;
		if (viewDir > 255) {
			viewDir -= 255;
		}
		self.angle = CAMERALOCATIONS[viewDir][2];

		self.angle = CAMERALOCATIONS[viewDir][2];
		self.x -= self.angle.cos();
		self.z -= self.angle.sin();
	}
	pub fn left(&mut self) {
		self.angle = CAMERALOCATIONS[self.camera_angle][2];		
		self.x -=self.angle.cos();
		self.z -= self.angle.sin();
	}
	pub fn right(&mut self) {
		self.angle = CAMERALOCATIONS[self.camera_angle][2];		
		self.x +=self.angle.cos();
		self.z += self.angle.sin();
	}
	pub fn camera_left(&mut self, amount: usize) {
		self.camera_angle += amount;
		if self.camera_angle >= 256 {
			self.camera_angle -= 256;
		}
		self.camera.set_y_rotation(CAMERALOCATIONS[self.camera_angle][2]);
        self.camera.local_x = CAMERALOCATIONS[self.camera_angle][0];
        self.camera.local_z = CAMERALOCATIONS[self.camera_angle][1];

	}
	pub fn camera_right(&mut self, mut amount: usize) {
		if (self.camera_angle < amount) {
			amount -= self.camera_angle ;
			self.camera_angle = 256;
		}
		self.camera_angle -= amount;
		self.camera.set_y_rotation(CAMERALOCATIONS[self.camera_angle][2]);
        self.camera.local_x = CAMERALOCATIONS[self.camera_angle][0];
        self.camera.local_z = CAMERALOCATIONS[self.camera_angle][1];

	}

	pub fn update_camera_position(&mut self) {
		self.camera.x = self.camera.local_x + self.x;
		self.camera.y = self.camera.local_y + self.y;
		self.camera.z = self.camera.local_z + self.z;
	}
}