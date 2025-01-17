use serde::Deserialize;

use super::math;
use super::BoundingBox;
use super::BoundingCylinder;
use super::Camera;
use super::Entity;
use crate::effects;
use crate::renderer;
use math::*;

use crate::fixed;
use fixed::*;

use crate::utils;

#[derive(Copy, Clone, Deserialize, Debug)]
pub struct Switch {
    #[serde(default = "default_i16")]
    id: i16,
    #[serde(default = "default_fixed")]
    x: Fixed,
    #[serde(default = "default_fixed")]
    y: Fixed,
    #[serde(default = "default_fixed")]
    z: Fixed,

    #[serde(default = "default_fixed")]
    xsize: Fixed,
    #[serde(default = "default_fixed")]
    ysize: Fixed,
    #[serde(default = "default_fixed")]
    zsize: Fixed,

    #[serde(default = "default_fixed")]
    x_rotation: Fixed,
    #[serde(rename = "rotation", default = "default_fixed")]
    y_rotation: Fixed,
    #[serde(default = "default_fixed")]
    z_rotation: Fixed,

    #[serde(default = "default_fixed_3_8")]
    points: [[Fixed; 3]; 8],
    #[serde(default = "default_fixed_3_8")]
    model_rotated_points: [[Fixed; 3]; 8],

    #[serde(default = "default_fixed_3_3")]
    x_rotation_matrix: [[Fixed; 3]; 3],
    #[serde(default = "default_fixed_3_3")]
    y_rotation_matrix: [[Fixed; 3]; 3],
    #[serde(default = "default_fixed_3_3")]
    z_rotation_matrix: [[Fixed; 3]; 3],

    #[serde(default = "default_u16")]
    color: u16,

    #[serde(default = "default_bool")]
    state: bool,
}

impl Switch {
    pub fn position_offset_from_state(&self) -> (Fixed, Fixed) {
        //90 derived from: LENGTH * cos(45) / 2
        //where the length is the height of the stick in y dir
        let mut result: Fixed = (self.points[1][1] - self.points[2][1]).abs() * Fixed::from_raw(90);
        if !self.state {
            result = -result;
        }

        let x_add = result * self.y_rotation.sin();
        let z_add = result * self.y_rotation.cos();
        return (x_add, z_add);
    }

    pub fn flip(&mut self) {
        self.state = !self.state;
        self.reload_rotation_matrices();
        self.refresh_model_matrix();
    }
}

impl Entity for Switch {
    fn set_x_offset(&mut self, x_offset: Fixed) {
        self.x = x_offset;
    }

    fn set_y_offset(&mut self, y_offset: Fixed) {
        self.y = y_offset;
    }

    fn set_z_offset(&mut self, z_offset: Fixed) {
        self.z = z_offset;
    }

    fn set_size(&mut self, _size: Fixed) {}

    fn recalculate_points(&mut self) {
        self.xsize = Fixed::from_raw(48);
        self.ysize = Fixed::from_raw(300);
        self.zsize = Fixed::from_raw(48);

        let halfx: Fixed = self.xsize / 2;
        let halfy: Fixed = self.ysize / 2;
        let halfz: Fixed = self.zsize / 2;
        self.points = [
            [(halfx), (halfy), (halfz)],
            [(-halfx), (halfy), (halfz)],
            [(-halfx), (-halfy), (halfz)],
            [(halfx), (-halfy), (halfz)],
            [(halfx), (halfy), (-halfz)],
            [(-halfx), (halfy), (-halfz)],
            [(-halfx), (-halfy), (-halfz)],
            [(halfx), (-halfy), (-halfz)],
        ];
    }

    fn set_x_rotation(&mut self, x_rotation: Fixed) {
        self.x_rotation = x_rotation;
        self.x_rotation_matrix = [
            [
                Fixed::const_new(1),
                Fixed::const_new(0),
                Fixed::const_new(0),
            ],
            [
                Fixed::const_new(0),
                self.x_rotation.cos(),
                -self.x_rotation.sin(),
            ],
            [
                Fixed::const_new(0),
                self.x_rotation.sin(),
                self.x_rotation.cos(),
            ],
        ];
    }

    fn set_y_rotation(&mut self, y_rotation: Fixed) {
        self.y_rotation = y_rotation;
        self.y_rotation_matrix = [
            [
                self.y_rotation.cos(),
                Fixed::const_new(0),
                self.y_rotation.sin(),
            ],
            [
                Fixed::const_new(0),
                Fixed::const_new(1),
                Fixed::const_new(0),
            ],
            [
                -self.y_rotation.sin(),
                Fixed::const_new(0),
                self.y_rotation.cos(),
            ],
        ];
    }

    fn set_z_rotation(&mut self, z_rotation: Fixed) {
        self.z_rotation = z_rotation;
        self.z_rotation_matrix = [
            [
                self.z_rotation.cos(),
                -self.z_rotation.sin(),
                Fixed::const_new(0),
            ],
            [
                self.z_rotation.sin(),
                self.z_rotation.cos(),
                Fixed::const_new(0),
            ],
            [
                Fixed::const_new(0),
                Fixed::const_new(0),
                Fixed::const_new(1),
            ],
        ];
    }

    fn reload_rotation_matrices(&mut self) {
        if self.state {
            self.x_rotation = Fixed::from_raw(32);
        } else {
            self.x_rotation = Fixed::from_raw(224);
        }

        self.set_x_rotation(self.x_rotation);
        self.set_y_rotation(self.y_rotation);
        self.set_z_rotation(self.z_rotation);
    }

    fn refresh_model_matrix(&mut self) {
        for i in 0..self.points.len() {
            let point: &[Fixed; 3] = &self.points[i];

            let mut rotated_point: [Fixed; 3] = matmul(self.x_rotation_matrix, *point);
            rotated_point = matmul(self.y_rotation_matrix, rotated_point);
            rotated_point = matmul(self.z_rotation_matrix, rotated_point);

            self.model_rotated_points[i] = rotated_point;
        }
    }

    fn set_vertex(&mut self, _point: [Fixed; 3], _index: i32) {
        //not implemented
    }

    fn render(&mut self, camera: &Camera, page: u16) {
        let (x_add, z_add) = self.position_offset_from_state();
        renderer::draw_rect(
            &self.model_rotated_points,
            self.x + x_add,
            self.y,
            self.z + z_add,
            self.y_rotation,
            camera,
            self.color,
            page,
        );
    }

    fn distance_from_camera(&self, camera: &Camera) -> Fixed {
        return (self.x - camera.x).abs() + (self.y - camera.y).abs() + (self.z - camera.z).abs();
    }

    fn bounding_box(&self) -> BoundingBox {
        let (x_add, z_add) = self.position_offset_from_state();
        let points: [[Fixed; 2]; 4] = [
            [
                self.model_rotated_points[0][0] + self.x - x_add,
                self.model_rotated_points[0][2] + self.z - z_add,
            ],
            [
                self.model_rotated_points[1][0] + self.x - x_add,
                self.model_rotated_points[1][2] + self.z - z_add,
            ],
            [
                self.model_rotated_points[5][0] + self.x - x_add,
                self.model_rotated_points[5][2] + self.z - z_add,
            ],
            [
                self.model_rotated_points[4][0] + self.x - x_add,
                self.model_rotated_points[4][2] + self.z - z_add,
            ],
        ];

        BoundingBox {
            data: points,
            center: utils::calculate_center(&points),
            width: (self.model_rotated_points[0][0] + self.x
                - (self.model_rotated_points[1][0] + self.x))
                .abs(),
            height: (self.model_rotated_points[1][2] + self.z
                - (self.model_rotated_points[5][2] + self.z))
                .abs(),
            y_top: self.model_rotated_points[0][1] + self.y,
            y_bottom: self.model_rotated_points[2][1] + self.y,
            rotation: self.y_rotation,
        }
    }

    fn bounding_cylinder(&self) -> BoundingCylinder {
        BoundingCylinder {
            x: self.x,
            z: self.z,
            radius: self.xsize / 2,
            y_top: self.model_rotated_points[0][1] + self.y,
            y_bottom: self.model_rotated_points[2][1] + self.y,
        }
    }

    fn get_y(&self) -> Fixed {
        return self.y;
    }

    fn set_color(&mut self, color: u16) {
        self.color = color;
    }

    fn tick(&mut self, effects: &effects::InputGameState) -> Option<effects::OutputEvents> {
        if effects.action_requested {
            if math::vector_len_2d(vector_sub_2d(effects.bounding_box.center, [self.x, self.z]))
                < Fixed::from_raw(400)
            {
                self.flip();
                return Some(effects::OutputEvents::SwitchAction(effects::SwitchFlip {
                    switch_flip: true,
                }));
                /*
                return Option::Some(effects::OutputEvents {
                    move_x: Fixed::const_new(0),
                    move_y: Fixed::const_new(0),
                    move_z: Fixed::const_new(0),
                    finished: false,
                    switch_flip: true,
                });*/
            }
        }
        None
    }

    fn get_id(&self) -> i16 {
        return self.id;
    }

    fn set_id(&mut self, id: i16) {
        self.id = id
    }
}
