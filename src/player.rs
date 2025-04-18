use lut::CAMERALOCATIONS;

use crate::{
    audio, camera,
    math::vector_len_2d,
    utils::{self, GameState},
};
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

#[derive(PartialEq)]
enum JumpState {
    Jumping,
    OnGround,
}

#[derive(PartialEq)]
enum JumpGoalState {
    Queued,
    Idle,
    Cleared,
}

pub struct Player<'a> {
    pub x: Fixed,
    pub y: Fixed,
    pub z: Fixed,
    pub yspeed: Fixed,
    pub action: bool,

    pub angle: Fixed,
    camera_angle: usize,
    pub camera: Camera,
    pub autorotate_camera: bool,

    jump_state: JumpState,
    jump_goal_state: JumpGoalState,
    pub jumping: bool,
    forced_jump: bool,
    in_air: bool,

    sliding: bool,

    pub move_x: Fixed,
    pub move_z: Fixed,
    pub activeaccel: Fixed,
    vblank: Option<&'a agb::interrupt::VBlank>,
    sound: Option<&'a agb::sound::dmg::Sound>,
    finish_animation_frames_left: u16,
}

impl<'a> Player<'a> {
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
            in_air: false,
            sliding: false,
            vblank: None,
            sound: None,
            jump_state: JumpState::Jumping,
            jump_goal_state: JumpGoalState::Cleared,
            finish_animation_frames_left: 0,
        }
    }

    pub fn init(&mut self, vblank: &'a agb::interrupt::VBlank, sound: &'a agb::sound::dmg::Sound) {
        self.vblank = Some(vblank);
        self.sound = Some(sound);
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
        self.yspeed = Fixed::const_new(0);
        self.in_air = false;
        self.jump_state = JumpState::OnGround;
    }

    pub fn fall(&mut self, ylimit: Fixed) {
        if self.y > ylimit {
            self.y += self.yspeed;
            if self.y < ylimit {
                self.y = ylimit;
                self.land();
            }
            self.yspeed -= BASEGRAVITY;
            self.in_air = true;
        } else {
            if self.in_air {
                audio::play_sound(3, self.vblank.unwrap(), self.sound.unwrap());
            }
            self.land();
        }
    }

    pub fn float(&mut self, ylimit: Fixed) {
        let y = self.y + Fixed::from_raw(192);
        if y < ylimit {
            self.y += self.yspeed;
            self.in_air = true;
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
        if self.yspeed == Fixed::const_new(0) && !self.in_air {
            self.yspeed = JUMPPOWER;
            self.forced_jump = false;
            self.activeaccel = AIRACCEL;
            audio::play_sound(2, self.vblank.unwrap(), self.sound.unwrap());
            self.jump_state = JumpState::Jumping;
            self.jump_goal_state = JumpGoalState::Cleared;
        }
    }

    pub fn try_jumping(&mut self) {
        if self.jump_goal_state == JumpGoalState::Idle {
            self.jump_goal_state = JumpGoalState::Queued;
        }
        if self.jump_goal_state == JumpGoalState::Queued {
            self.jump();
        }
        if self.jump_state == JumpState::Jumping {
            self.jumping = true;
        }
    }

    pub fn cancel_jump(&mut self) {
        if self.jump_state == JumpState::Jumping {
            self.jumping = false;
        }
        if self.jump_goal_state == JumpGoalState::Cleared
            || self.jump_goal_state == JumpGoalState::Queued
        {
            self.jump_goal_state = JumpGoalState::Idle;
        }
    }

    //set active to true, when player also jumps when contacting the platform
    pub fn bounce(&mut self, power: Fixed, active_bounce: bool) {
        self.yspeed = power;
        self.forced_jump = active_bounce;
        self.activeaccel = AIRACCEL;
        audio::play_sound(2, self.vblank.unwrap(), self.sound.unwrap());
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

    pub fn sliding(&mut self, accel: Fixed) {
        self.activeaccel = accel;
        self.sliding = true;
    }

    pub fn tick(&mut self) {
        self.action = false;
        if !self.in_air && !self.sliding {
            self.activeaccel = GROUNDACCEL;
        }
        self.sliding = false;
    }

    pub fn finish_animation(&mut self) {
        self.finish_animation_frames_left = 60;
    }

    pub fn next_animation_frame(&mut self) -> GameState {
        if self.finish_animation_frames_left > 0 {
            self.finish_animation_frames_left -= 1;
            self.angle += Fixed::from_raw(16);
            self.y += Fixed::from_raw(32);

            return GameState::CompleteAnimation;
        }

        return GameState::Finished;
    }
}
