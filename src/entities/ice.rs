//disable player movement input controls when on top of this block type
//reset at the end of every frame, and only do inputs after effects have taken place

use agb::InternalAllocator;
use alloc::vec::Vec;
use serde::Deserialize;

use super::boundingshapes::BoundingShape;
use super::math;
use super::BoundingBox;
use super::BoundingCylinder;
use super::Camera;
use super::Entity;
use crate::effects;
use crate::rectangle_model_points;
use crate::renderer;
use crate::renderer::polygon::Polygon;
use math::*;

use crate::fixed;
use crate::utils;
use fixed::*;

#[derive(Copy, Clone, Deserialize, Debug)]
pub struct Ice {
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

    #[serde(default = "default_fixed")]
    acceleration: Fixed,
}

impl Ice {
    #[allow(dead_code)]
    pub fn default() -> Self {
        Self {
            id: 0,
            x: Fixed::const_new(0),
            y: Fixed::const_new(0),
            z: Fixed::const_new(0),
            xsize: Fixed::const_new(0),
            ysize: Fixed::const_new(0),
            zsize: Fixed::const_new(0),
            x_rotation: Fixed::const_new(0),
            y_rotation: Fixed::const_new(0),
            z_rotation: Fixed::const_new(0),
            points: [[Fixed::const_new(0); 3]; 8],
            model_rotated_points: [[Fixed::const_new(0); 3]; 8],
            x_rotation_matrix: [[Fixed::const_new(0); 3]; 3],
            y_rotation_matrix: [[Fixed::const_new(0); 3]; 3],
            z_rotation_matrix: [[Fixed::const_new(0); 3]; 3],
            color: 0,
            acceleration: Fixed::const_new(0),
        }
    }
}

impl Entity for Ice {
    fn set_x_offset(&mut self, x_offset: Fixed) {
        self.x = x_offset;
    }

    fn set_y_offset(&mut self, y_offset: Fixed) {
        self.y = y_offset;
    }

    fn set_z_offset(&mut self, z_offset: Fixed) {
        self.z = z_offset;
    }

    fn set_size(&mut self, size: Fixed) {
        self.xsize = size;
        self.ysize = size;
        self.zsize = size;
    }

    fn recalculate_points(&mut self) {
        self.points = rectangle_model_points(self.xsize, self.ysize, self.zsize);
    }

    fn set_x_rotation(&mut self, x_rotation: Fixed) {
        self.x_rotation = x_rotation;
        self.x_rotation_matrix = utils::x_rotation_matrix(x_rotation);
    }

    fn set_y_rotation(&mut self, y_rotation: Fixed) {
        self.y_rotation = y_rotation;
        self.y_rotation_matrix = utils::y_rotation_matrix(y_rotation);
    }

    fn set_z_rotation(&mut self, z_rotation: Fixed) {
        self.z_rotation = z_rotation;
        self.z_rotation_matrix = utils::z_rotation_matrix(z_rotation);
    }

    fn reload_rotation_matrices(&mut self) {
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

    fn render(
        &mut self,
        camera: &Camera,
        polygons: &mut Vec<Polygon, InternalAllocator>,
        render_distance: Fixed,
    ) {
        if self.distance_from_camera(camera) > render_distance {
            return;
        }

        renderer::render::render_rect(
            &self.model_rotated_points,
            self.x,
            self.y,
            self.z,
            self.y_rotation,
            camera,
            self.color,
            polygons,
            false,
        );
    }

    fn distance_from_camera(&self, camera: &Camera) -> Fixed {
        return (self.x - camera.x).abs() + (self.y - camera.y).abs() + (self.z - camera.z).abs();
    }

    fn bounding_shape(&self) -> Option<BoundingShape> {
        let points: [[Fixed; 2]; 4] = [
            [
                self.model_rotated_points[0][0] + self.x,
                self.model_rotated_points[0][2] + self.z,
            ],
            [
                self.model_rotated_points[1][0] + self.x,
                self.model_rotated_points[1][2] + self.z,
            ],
            [
                self.model_rotated_points[5][0] + self.x,
                self.model_rotated_points[5][2] + self.z,
            ],
            [
                self.model_rotated_points[4][0] + self.x,
                self.model_rotated_points[4][2] + self.z,
            ],
        ];

        Some(BoundingShape::BoundingBox(BoundingBox {
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
            rotation: -self.y_rotation,
        }))
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
    fn get_height(&self) -> Fixed {
        return self.ysize;
    }
    fn set_color(&mut self, color: u16) {
        self.color = color;
    }
    fn tick(&mut self, effects: &effects::InputGameState) -> Option<effects::OutputEvents> {
        if effects.support_below_id == self.id {
            return Some(effects::OutputEvents::Sliding(effects::Sliding {acceleration: self.acceleration}));
        } else {
            return None;
        }
    }

    fn get_id(&self) -> i16 {
        return self.id;
    }

    fn set_id(&mut self, id: i16) {
        self.id = id
    }
}
