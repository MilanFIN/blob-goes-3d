use serde::Deserialize;

use super::math;
use super::render;
use super::BoundingBox;
use super::BoundingCylinder;
use super::Camera;
use super::Entity;
use math::*;
use render::*;

use crate::fixed;
use fixed::*;

use crate::utils;


/*
const fov: Fixed = Fixed::const_new(90); // FOV in degrees
const aspect_ratio: Fixed = Fixed::from_raw(384);
const near: Fixed = Fixed::from_raw(25);
const far: Fixed = Fixed::const_new(1000);


// Calculate constants for projection matrix
const fov_rad:Fixed = Fixed::from_raw(201) ;
//fov.to_radians() / Fixed::const_new(2);

const scale:Fixed = Fixed::from_raw(-1);
//Fixed::const_new(1) / fov_rad.tan();
const depth_range: Fixed = Fixed::const_new(975);
//far - near;

const projection_matrix: [[Fixed; 4]; 4] = [
    [Fixed::from_raw(-1), Fixed::const_new(0), Fixed::const_new(0), Fixed::const_new(0)],
    [Fixed::const_new(0), scale, Fixed::const_new(0), Fixed::const_new(0)],
    [Fixed::const_new(0), Fixed::const_new(0), -(far + near) / depth_range, -(Fixed::const_new(2) * far * near) / depth_range],
    [Fixed::const_new(0), Fixed::const_new(0), Fixed::const_new(-1), Fixed::const_new(0)],
];
*/

#[derive(Copy, Clone, Deserialize, Debug)]
pub struct Cube {
    #[serde(default = "default_fixed")]
    x: Fixed,
    #[serde(default = "default_fixed")]
    y: Fixed,
    #[serde(default = "default_fixed")]
    z: Fixed,

    #[serde(default = "default_fixed")]
    size: Fixed,

    #[serde(default = "default_fixed")]
    x_rotation: Fixed,
    #[serde(default = "default_fixed")]
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

    #[serde(default = "default_fixed_3_8")]
    world_points: [[Fixed; 3]; 8],
}

impl Cube {
    pub fn default() -> Self {
        Self {
            x: Fixed::const_new(0),
            y: Fixed::const_new(0),
            z: Fixed::const_new(0),
            size: Fixed::const_new(1),
            x_rotation: Fixed::const_new(0),
            y_rotation: Fixed::const_new(0),
            z_rotation: Fixed::const_new(0),
            points: [[Fixed::const_new(0); 3]; 8],
            model_rotated_points: [[Fixed::const_new(0); 3]; 8],
            x_rotation_matrix: [[Fixed::const_new(0); 3]; 3],
            y_rotation_matrix: [[Fixed::const_new(0); 3]; 3],
            z_rotation_matrix: [[Fixed::const_new(0); 3]; 3],
            world_points: [[Fixed::const_new(0); 3]; 8],
        }
    }
}

impl Entity for Cube {
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
        self.size = size;
    }

    fn recalculate_points(&mut self) {
        self.points = [
            [(self.size / 2), (self.size / 2), (self.size / 2)],
            [(-self.size / 2), (self.size / 2), (self.size / 2)],
            [(-self.size / 2), (-self.size / 2), (self.size / 2)],
            [(self.size / 2), (-self.size / 2), (self.size / 2)],
            [(self.size / 2), (self.size / 2), (-self.size / 2)],
            [(-self.size / 2), (self.size / 2), (-self.size / 2)],
            [(-self.size / 2), (-self.size / 2), (-self.size / 2)],
            [(self.size / 2), (-self.size / 2), (-self.size / 2)],
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

    fn render(&mut self, bitmap4: &mut agb::display::bitmap4::Bitmap4, camera: &Camera) {
        let projection_matrix: [[Fixed; 4]; 4] = [
            [
                Fixed::from_f32(0.66666667),
                Fixed::from_f32(0.0),
                Fixed::from_f32(0.0),
                Fixed::from_f32(0.0),
            ],
            [
                Fixed::from_f32(0.0),
                Fixed::from_f32(1.0),
                Fixed::from_f32(0.0),
                Fixed::from_f32(0.0),
            ],
            [
                Fixed::from_f32(0.0),
                Fixed::from_f32(0.0),
                Fixed::from_f32(-1.00020002),
                Fixed::from_f32(-0.20002),
            ],
            [
                Fixed::from_f32(0.0),
                Fixed::from_f32(0.0),
                Fixed::from_f32(-1.0),
                Fixed::from_f32(0.0),
            ],
        ];

        let width: i32 = 240;
        let height: i32 = 160;
        let middle: [Fixed; 2] = [Fixed::const_new(width / 2), Fixed::const_new(height / 2)]; // x, y

        let mut screen_points: [[Fixed; 2]; 8] = [[Fixed::const_new(0), Fixed::const_new(0)]; 8];
        let mut translated_points: [[Fixed; 3]; 8] = [[
            Fixed::const_new(0),
            Fixed::const_new(0),
            Fixed::const_new(0),
        ]; 8];

        for i in 0..self.model_rotated_points.len() {
            let mut translated_point: [Fixed; 4] = [
                self.model_rotated_points[i][0] + (self.x - camera.x),
                self.model_rotated_points[i][1] + (self.y - camera.y),
                self.model_rotated_points[i][2] + (self.z - camera.z),
                Fixed::const_new(1),
            ];

            self.world_points[i] = [
                self.model_rotated_points[i][0] + self.x,
                self.model_rotated_points[i][1] + self.y,
                self.model_rotated_points[i][2] + self.z,
            ];
            
            translated_point = matmul_4(camera.y_rotation_matrix, translated_point);
            translated_point = matmul_4(camera.x_rotation_matrix, translated_point);
            translated_point = matmul_4(camera.z_rotation_matrix, translated_point);

            // Apply projection matrix
            let projected_point = matmul_4(projection_matrix, translated_point);

            // Perform perspective divide (convert to 2D)
            if projected_point[3] != Fixed::const_new(0) {
                let x = projected_point[0] / projected_point[3];
                let y = projected_point[1] / projected_point[3];
                // Convert to screen space
                screen_points[i] = [
                    (x * Fixed::const_new(width) / Fixed::const_new(2)) + middle[0],
                    (y * Fixed::const_new(height) / Fixed::const_new(2)) + middle[1],
                ];
            } else {
                screen_points[i] = [middle[0], middle[1]];
            }

            translated_points[i] = [
                translated_point[0],
                translated_point[1],
                translated_point[2],
            ];

        }
        if back_face_culling(&translated_points, 0, 1, 2) {
            //draw_face_outline(&mut bitmap4, screenPoints, 0, 1, 2, 3);
            draw_triangle(
                bitmap4,
                screen_points[0],
                screen_points[1],
                screen_points[2],
                1,
            );
            draw_triangle(
                bitmap4,
                screen_points[0],
                screen_points[2],
                screen_points[3],
                1,
            );
        }
        if back_face_culling(&translated_points, 7, 6, 5) {
            //draw_face_outline(&mut bitmap4, screenPoints, 7, 6, 5, 4);
            draw_triangle(
                bitmap4,
                screen_points[7],
                screen_points[6],
                screen_points[5],
                1,
            );
            draw_triangle(
                bitmap4,
                screen_points[7],
                screen_points[5],
                screen_points[4],
                1,
            );
        }

        if back_face_culling(&translated_points, 0, 3, 7) {
            //draw_face_outline(&mut bitmap4, screenPoints, 0, 3, 7, 4);
            draw_triangle(
                bitmap4,
                screen_points[0],
                screen_points[3],
                screen_points[7],
                2,
            );
            draw_triangle(
                bitmap4,
                screen_points[0],
                screen_points[7],
                screen_points[4],
                2,
            );
        }
        if back_face_culling(&translated_points, 1, 5, 6) {
            //draw_face_outline(&mut bitmap4, screenPoints, 1, 5, 6, 2);
            draw_triangle(
                bitmap4,
                screen_points[1],
                screen_points[5],
                screen_points[6],
                2,
            );
            draw_triangle(
                bitmap4,
                screen_points[1],
                screen_points[6],
                screen_points[2],
                2,
            );
        }

        if back_face_culling(&translated_points, 7, 3, 2) {
            //draw_face_outline(&mut bitmap4, screenPoints, 7, 3, 2, 6);
            draw_triangle(
                bitmap4,
                screen_points[7],
                screen_points[3],
                screen_points[2],
                3,
            );
            draw_triangle(
                bitmap4,
                screen_points[7],
                screen_points[2],
                screen_points[6],
                3,
            );
        }
        if back_face_culling(&translated_points, 0, 4, 5) {
            //draw_face_outline(&mut bitmap4, screenPoints, 0, 4, 5, 1);
            draw_triangle(
                bitmap4,
                screen_points[0],
                screen_points[4],
                screen_points[5],
                3,
            );
            draw_triangle(
                bitmap4,
                screen_points[0],
                screen_points[5],
                screen_points[1],
                3,
            );
        }
    }

    fn distance_from_camera(&self, camera: &Camera) -> Fixed {
        return (self.x - camera.x).abs() + (self.y - camera.y).abs() + (self.z - camera.z).abs();
    }

    fn bounding_box(&self) -> BoundingBox {
        let points: [[Fixed; 2]; 4] =  [
            [self.world_points[0][0], self.world_points[0][2]],
            [self.world_points[1][0], self.world_points[1][2]],
            [self.world_points[5][0], self.world_points[5][2]],
            [self.world_points[4][0], self.world_points[4][2]],
        ];
        BoundingBox {
            data: points,
            center: utils::calculate_center(&points),
            width: (self.world_points[0][0] - self.world_points[1][0]).abs(),
            height: (self.world_points[1][2] - self.world_points[5][2]).abs(),
            y_top: self.world_points[0][1],
            y_bottom: self.world_points[2][1],
            rotation: self.y_rotation
        }
    }

    fn bounding_cylinder(&self) -> BoundingCylinder {
        BoundingCylinder {
            x: self.x,
            z: self.z,
            radius: self.size / 2,
            y_top: self.world_points[0][1],
            y_bottom: self.world_points[2][1],
        }
    }
    fn get_y(&self) -> Fixed {
        return self.y;
    }
}
