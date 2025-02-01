use agb::InternalAllocator;
use alloc::vec::Vec;
use serde::Deserialize;

use super::boundingshapes::BoundingShape;
use super::math;
use super::utils::cylinder_and_rotated_rect_collision;
use super::utils::rect_overlap;
use super::utils::rect_simple_overlap_check;
use super::BoundingBox;
use super::BoundingCylinder;
use super::Camera;
use super::Entity;
use crate::effects;
use crate::renderer;
use crate::renderer::render::back_face_culling;
use crate::renderer::polygon::Polygon;
use math::*;

use crate::fixed;
use fixed::*;

use crate::utils;

#[derive(Copy, Clone, Deserialize, Debug)]
pub struct Finish {
    #[serde(default = "default_i16")]
    id: i16,
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
    #[serde(rename = "rotation", default = "default_fixed")]
    y_rotation: Fixed,
    #[serde(default = "default_fixed")]
    z_rotation: Fixed,

    #[serde(default = "default_fixed_3_14")]
    points: [[Fixed; 3]; 14],
    #[serde(default = "default_fixed_3_14")]
    model_rotated_points: [[Fixed; 3]; 14],

    #[serde(default = "default_fixed_3_3")]
    x_rotation_matrix: [[Fixed; 3]; 3],
    #[serde(default = "default_fixed_3_3")]
    y_rotation_matrix: [[Fixed; 3]; 3],
    #[serde(default = "default_fixed_3_3")]
    z_rotation_matrix: [[Fixed; 3]; 3],

    #[serde(default = "default_u16")]
    color: u16,

    #[serde(default = "default_fixed")]
    radius: Fixed,
    #[serde(default = "default_fixed")]
    depth: Fixed,
}

impl Finish {
    #[allow(dead_code)]
    pub fn default() -> Self {
        Self {
            x: Fixed::const_new(0),
            y: Fixed::const_new(0),
            z: Fixed::const_new(0),
            size: Fixed::const_new(1),
            x_rotation: Fixed::const_new(0),
            y_rotation: Fixed::const_new(0),
            z_rotation: Fixed::const_new(0),
            points: [[Fixed::const_new(0); 3]; 14],
            model_rotated_points: [[Fixed::const_new(0); 3]; 14],
            x_rotation_matrix: [[Fixed::const_new(0); 3]; 3],
            y_rotation_matrix: [[Fixed::const_new(0); 3]; 3],
            z_rotation_matrix: [[Fixed::const_new(0); 3]; 3],
            color: 0,
            id: 0,
            radius: Fixed::const_new(0),
            depth: Fixed::const_new(0),
        }
    }

    fn finish_bounding_box(&self) -> BoundingBox {
        let points: [[Fixed; 2]; 4] = [
            [self.radius + self.x, self.depth / 2 + self.z],
            [self.radius + self.x, -self.depth / 2 + self.z],
            [-self.radius + self.x, -self.depth / 2 + self.z],
            [-self.radius + self.x, self.depth / 2 + self.z],
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
            y_top: self.radius + self.y,
            y_bottom: -self.radius + self.y,
            rotation: self.y_rotation,
        }
    }
}

impl Entity for Finish {
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
        self.depth = Fixed::from_raw(32);
        //front face
        self.points[0] = [Fixed::const_new(0), Fixed::const_new(0), self.depth / 2];

        self.radius = Fixed::const_new(2);

        for i in 1..8 {
            let angle = Fixed::from_raw(43) * i; // Angle in radians (i * 60 degrees)
            self.points[i] = [
                self.radius * angle.cos(),
                self.radius * angle.sin(),
                self.depth / 2,
            ];
        }
        //back face
        self.points[7] = [Fixed::const_new(0), Fixed::const_new(0), -self.depth / 2];

        for i in 8..14 {
            let angle = Fixed::from_raw(43) * (i - 7); // Angle in radians (i * 60 degrees)
            self.points[i] = [
                self.radius * angle.cos(),
                self.radius * angle.sin(),
                -self.depth / 2,
            ];
        }
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

    fn render(&mut self, camera: &Camera, polygons: &mut Vec<Polygon, InternalAllocator>, render_distance: Fixed) {

        if self.distance_from_camera(camera) > render_distance {
            return;
        }

        let mut screen_points: [[Fixed; 2]; 14] = [[Fixed::const_new(0), Fixed::const_new(0)]; 14];
        let mut translated_points: [[Fixed; 3]; 14] = [[
            Fixed::const_new(0),
            Fixed::const_new(0),
            Fixed::const_new(0),
        ]; 14];

        for i in 0..(self.model_rotated_points).len() {
            (translated_points[i], screen_points[i]) = renderer::render::translate_point(
                &self.model_rotated_points[i],
                &camera,
                self.x,
                self.y,
                self.z,
            );
        }

        let visible: bool = back_face_culling(&translated_points, 0, 1, 2);
        if visible {
            let color: u16 = renderer::utils::get_color(self.color, 0);
            let distance0 = renderer::utils::polygon_avg_z(&translated_points, 0, 1, 2);
            polygons.push(Polygon {
                distance_from_camera: distance0,
                shape: renderer::polygon::Shape::Triangle([
                    screen_points[0],
                    screen_points[1],
                    screen_points[2],
                ]),
                color,
            });
            let distance0 = renderer::utils::polygon_avg_z(&translated_points, 0, 2, 3);
            polygons.push(Polygon {
                distance_from_camera: distance0,
                shape: renderer::polygon::Shape::Triangle([
                    screen_points[0],
                    screen_points[2],
                    screen_points[3],
                ]),
                color,
            });
            let distance0 = renderer::utils::polygon_avg_z(&translated_points, 0, 3, 4);
            polygons.push(Polygon {
                distance_from_camera: distance0,
                shape: renderer::polygon::Shape::Triangle([
                    screen_points[0],
                    screen_points[3],
                    screen_points[4],
                ]),
                color,
            });
            let distance0 = renderer::utils::polygon_avg_z(&translated_points, 0, 4, 5);
            polygons.push(Polygon {
                distance_from_camera: distance0,
                shape: renderer::polygon::Shape::Triangle([
                    screen_points[0],
                    screen_points[4],
                    screen_points[5],
                ]),
                color,
            });
            let distance0 = renderer::utils::polygon_avg_z(&translated_points, 0, 5, 6);
            polygons.push(Polygon {
                distance_from_camera: distance0,
                shape: renderer::polygon::Shape::Triangle([
                    screen_points[0],
                    screen_points[5],
                    screen_points[6],
                ]),
                color,
            });
            let distance0 = renderer::utils::polygon_avg_z(&translated_points, 0, 6, 1);
            polygons.push(Polygon {
                distance_from_camera: distance0,
                shape: renderer::polygon::Shape::Triangle([
                    screen_points[0],
                    screen_points[6],
                    screen_points[1],
                ]),
                color,
            });
        }
        let visible: bool = back_face_culling(&translated_points, 7, 9, 8);
        if visible {
            let color: u16 = renderer::utils::get_color(self.color, 0);
            let distance0 = renderer::utils::polygon_avg_z(&translated_points, 7, 8, 9);
            polygons.push(Polygon {
                distance_from_camera: distance0,
                shape: renderer::polygon::Shape::Triangle([
                    screen_points[7],
                    screen_points[8],
                    screen_points[9],
                ]),
                color,
            });
            let distance0 = renderer::utils::polygon_avg_z(&translated_points, 7, 9, 10);
            polygons.push(Polygon {
                distance_from_camera: distance0,
                shape: renderer::polygon::Shape::Triangle([
                    screen_points[7],
                    screen_points[9],
                    screen_points[10],
                ]),
                color,
            });
            let distance0 = renderer::utils::polygon_avg_z(&translated_points, 7, 10, 11);
            polygons.push(Polygon {
                distance_from_camera: distance0,
                shape: renderer::polygon::Shape::Triangle([
                    screen_points[7],
                    screen_points[10],
                    screen_points[11],
                ]),
                color,
            });
            let distance0 = renderer::utils::polygon_avg_z(&translated_points, 7, 11, 12);
            polygons.push(Polygon {
                distance_from_camera: distance0,
                shape: renderer::polygon::Shape::Triangle([
                    screen_points[7],
                    screen_points[11],
                    screen_points[12],
                ]),
                color,
            });

            let distance0 = renderer::utils::polygon_avg_z(&translated_points, 7, 12, 13);
            polygons.push(Polygon {
                distance_from_camera: distance0,
                shape: renderer::polygon::Shape::Triangle([
                    screen_points[7],
                    screen_points[12],
                    screen_points[13],
                ]),
                color,
            });
            let distance0 = renderer::utils::polygon_avg_z(&translated_points, 7, 13, 8);
            polygons.push(Polygon {
                distance_from_camera: distance0,
                shape: renderer::polygon::Shape::Triangle([
                    screen_points[7],
                    screen_points[13],
                    screen_points[8],
                ]),
                color,
            });
        }

        for i in 1..6 {
            let visible: bool = back_face_culling(&translated_points, i, i + 8, i + 1);
            if visible {
                let color: u16 = renderer::utils::get_color(self.color, (i % 3 + 1) as i16);
                let distance0 = renderer::utils::polygon_avg_z(&translated_points, i, i + 8, i + 1);
                polygons.push(Polygon {
                    distance_from_camera: distance0,
                    shape: renderer::polygon::Shape::Triangle([
                        screen_points[i],
                        screen_points[i + 8],
                        screen_points[i + 1],
                    ]),
                    color,
                });
                let distance0 = renderer::utils::polygon_avg_z(&translated_points, i, i + 7, i + 8);
                polygons.push(Polygon {
                    distance_from_camera: distance0,
                    shape: renderer::polygon::Shape::Triangle([
                        screen_points[i],
                        screen_points[i + 8],
                        screen_points[i + 7],
                    ]),
                    color,
                });
            }
        }
        let visible: bool = back_face_culling(&translated_points, 6, 13, 8);
        if visible {
            let color: u16 = renderer::utils::get_color(self.color, 1);
            let distance0 = renderer::utils::polygon_avg_z(&translated_points, 6,13,8);
            polygons.push(Polygon {
                distance_from_camera: distance0,
                shape: renderer::polygon::Shape::Triangle([
                    screen_points[6],
                    screen_points[13],
                    screen_points[8],
                ]),
                color,
            });
            let distance0 = renderer::utils::polygon_avg_z(&translated_points, 6, 1, 8);
            polygons.push(Polygon {
                distance_from_camera: distance0,
                shape: renderer::polygon::Shape::Triangle([
                    screen_points[6],
                    screen_points[1],
                    screen_points[8],
                ]),
                color,
            });

        }
    }

    fn distance_from_camera(&self, camera: &Camera) -> Fixed {
        return (self.x - camera.x).abs() + (self.y - camera.y).abs() + (self.z - camera.z).abs();
    }

    fn bounding_shape(&self) -> Option<BoundingShape> {
        //the finish has no collision with the player
       None
    }

    fn bounding_cylinder(&self) -> BoundingCylinder {
        BoundingCylinder::empty()
    }

    fn get_y(&self) -> Fixed {
        return self.y;
    }
    fn get_height(&self) -> Fixed {
        return self.radius * 2;
    }

    fn set_color(&mut self, color: u16) {
        self.color = color;
    }

    fn tick(&mut self, effects: &effects::InputGameState) -> Option<effects::OutputEvents> {
        let hitbox = self.finish_bounding_box();
        if (effects.bounding_box.y_top > hitbox.y_bottom
            && effects.bounding_box.y_bottom < hitbox.y_top)
            && rect_simple_overlap_check(effects.bounding_box, &hitbox)
            && (rect_overlap(&hitbox, effects.bounding_box)
                || cylinder_and_rotated_rect_collision(effects.bounding_cylinder, &hitbox).1)
        {
            return Some(effects::OutputEvents::GameFinish(effects::Finished {
                //finished: true,
            }));
        } else {
            None
        }
    }
    fn get_id(&self) -> i16 {
        return self.id;
    }

    fn set_id(&mut self, id: i16) {
        self.id = id
    }
}
