use agb::InternalAllocator;
use alloc::vec::Vec;
use serde::Deserialize;

use super::boundingshapes::BoundingShape;
use super::math;
use super::BoundingCylinder;
use super::Camera;
use super::Entity;
use crate::effects;
use crate::renderer;
use crate::renderer::polygon::Polygon;
use crate::renderer::render::back_face_culling;
use math::*;

use crate::fixed;
use crate::utils;
use fixed::*;

#[derive(Copy, Clone, Deserialize, Debug)]
pub struct Bounce {
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
    height: Fixed,

    #[serde(default = "default_fixed")]
    x_rotation: Fixed,
    #[serde(rename = "rotation", default = "default_fixed")]
    y_rotation: Fixed,
    #[serde(default = "default_fixed")]
    z_rotation: Fixed,

    #[serde(default = "default_fixed_3_11")]
    points: [[Fixed; 3]; 11],
    #[serde(default = "default_fixed_3_11")]
    model_rotated_points: [[Fixed; 3]; 11],

    #[serde(default = "default_fixed_3_3")]
    x_rotation_matrix: [[Fixed; 3]; 3],
    #[serde(default = "default_fixed_3_3")]
    y_rotation_matrix: [[Fixed; 3]; 3],
    #[serde(default = "default_fixed_3_3")]
    z_rotation_matrix: [[Fixed; 3]; 3],

    #[serde(default = "default_u16")]
    color: u16,

    #[serde(default = "default_fixed")]
    power: Fixed,
}

impl Bounce {
    #[allow(dead_code)]
    pub fn default() -> Self {
        Self {
            id: 0,
            x: Fixed::const_new(0),
            y: Fixed::const_new(0),
            z: Fixed::const_new(0),
            size: Fixed::const_new(0),
            height: Fixed::const_new(0),
            x_rotation: Fixed::const_new(0),
            y_rotation: Fixed::const_new(0),
            z_rotation: Fixed::const_new(0),
            points: [[Fixed::const_new(0); 3]; 11],
            model_rotated_points: [[Fixed::const_new(0); 3]; 11],
            x_rotation_matrix: [[Fixed::const_new(0); 3]; 3],
            y_rotation_matrix: [[Fixed::const_new(0); 3]; 3],
            z_rotation_matrix: [[Fixed::const_new(0); 3]; 3],
            color: 0,
            power: Fixed::const_new(256),
        }
    }
}

impl Entity for Bounce {
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
        //self.points = rectangle_model_points(self.xsize, self.ysize, self.zsize);

        let radius = self.size / 2;
        let v_offset = self.height / 2;
        let angle_increment = Fixed::const_new(1) / Fixed::const_new(5);
        for i in 0..5 {
            let angle = angle_increment * i as i32;
            let x = -radius * angle.cos();
            let z = radius * angle.sin();
            //self.points[i] = [x, v_offset, z];
            self.points[i+1] = [x, v_offset, z];
            self.points[i+6] = [x, -v_offset, z];

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

    fn render(
        &mut self,
        camera: &Camera,
        polygons: &mut Vec<Polygon, InternalAllocator>,
        render_distance: Fixed,
    ) {
        if self.distance_from_camera(camera) > render_distance {
            return;
        }

        let mut screen_points: [[Fixed; 2]; 11] = [[Fixed::const_new(0), Fixed::const_new(0)]; 11];
        let mut translated_points: [[Fixed; 3]; 11] = [[
            Fixed::const_new(0),
            Fixed::const_new(0),
            Fixed::const_new(0),
        ]; 11];

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
                color: color,
                draw_always: false,
            });
            let distance0 = renderer::utils::polygon_avg_z(&translated_points, 0, 2, 3);
            polygons.push(Polygon {
                distance_from_camera: distance0,
                shape: renderer::polygon::Shape::Triangle([
                    screen_points[0],
                    screen_points[2],
                    screen_points[3],
                ]),
                color: color,
                draw_always: false,
            });
            let distance0 = renderer::utils::polygon_avg_z(&translated_points, 0, 3, 4);
            polygons.push(Polygon {
                distance_from_camera: distance0,
                shape: renderer::polygon::Shape::Triangle([
                    screen_points[0],
                    screen_points[3],
                    screen_points[4],
                ]),
                color: color,
                draw_always: false,
            });
            let distance0 = renderer::utils::polygon_avg_z(&translated_points, 0, 4, 5);
            polygons.push(Polygon {
                distance_from_camera: distance0,
                shape: renderer::polygon::Shape::Triangle([
                    screen_points[0],
                    screen_points[4],
                    screen_points[5],
                ]),
                color: color,
                draw_always: false,
            });
            let distance0 = renderer::utils::polygon_avg_z(&translated_points, 0, 5, 1);
            polygons.push(Polygon {
                distance_from_camera: distance0,
                shape: renderer::polygon::Shape::Triangle([
                    screen_points[0],
                    screen_points[5],
                    screen_points[1],
                ]),
                color: color,
                draw_always: false,
            });
        
        }

        for i in 1..5 {
            let visible: bool = back_face_culling(&translated_points, i, i + 5, i + 1);
            if visible {
                let color: u16 = renderer::utils::get_color(self.color, (i % 3 + 1) as i16);
                let distance0 = renderer::utils::polygon_avg_z(&translated_points, i, i + 5, i + 1);
                polygons.push(Polygon {
                    distance_from_camera: distance0,
                    shape: renderer::polygon::Shape::Triangle([
                        screen_points[i],
                        screen_points[i + 5],
                        screen_points[i + 1],
                    ]),
                    color: color,
                    draw_always: false,
                    });
                let distance0 = renderer::utils::polygon_avg_z(&translated_points, i+1, i + 5, i + 6);
                polygons.push(Polygon {
                    distance_from_camera: distance0,
                    shape: renderer::polygon::Shape::Triangle([
                        screen_points[i+1],
                        screen_points[i + 5],
                        screen_points[i + 6],
                    ]),
                    color: color,
                    draw_always: false,
                    });
            }
        }
        
        let visible: bool = back_face_culling(&translated_points, 5, 10, 6);
        if visible {
            let color: u16 = renderer::utils::get_color(self.color, (5 % 3 + 1) as i16);
            
            let distance0 = renderer::utils::polygon_avg_z(&translated_points, 5,10,6);
            polygons.push(Polygon {
                distance_from_camera: distance0,
                shape: renderer::polygon::Shape::Triangle([
                    screen_points[5],
                    screen_points[10],
                    screen_points[6],
                ]),
                color: color,
                draw_always: false,
            });
            
            let distance0 = renderer::utils::polygon_avg_z(&translated_points, 5,6,1);
            polygons.push(Polygon {
                distance_from_camera: distance0,
                shape: renderer::polygon::Shape::Triangle([
                    screen_points[5],
                    screen_points[6],
                    screen_points[1],
                ]),
                color: color,
                draw_always: false,
            });
        }


        /*
        renderer::render::render_rect(
            &self.model_rotated_points,
            self.x,
            self.y,
            self.z,
            self.y_rotation,
            camera,
            self.color,
            polygons,
        );*/
    }

    fn distance_from_camera(&self, camera: &Camera) -> Fixed {
        return (self.x - camera.x).abs() + (self.y - camera.y).abs() + (self.z - camera.z).abs();
    }

    fn bounding_shape(&self) -> Option<BoundingShape> {
        /*
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
        }))*/
        Some(BoundingShape::BoundingCylinder(BoundingCylinder {
            x: self.x,
            z: self.z,
            radius: self.size/2,
            y_top: self.model_rotated_points[0][1] + self.y,
            y_bottom: self.model_rotated_points[2][1] + self.y,
        }))

    }

    fn bounding_cylinder(&self) -> BoundingCylinder {
        BoundingCylinder {
            x: self.x,
            z: self.z,
            radius: self.size / 2,
            y_top: self.model_rotated_points[0][1] + self.y,
            y_bottom: self.model_rotated_points[2][1] + self.y,
        }
    }
    fn get_y(&self) -> Fixed {
        return self.y;
    }
    fn get_height(&self) -> Fixed {
        return self.height;
    }
    fn set_color(&mut self, color: u16) {
        self.color = color;
    }
    fn tick(&mut self, effects: &effects::InputGameState) -> Option<effects::OutputEvents> {
        if effects.support_below_id == self.id {
            return Some(effects::OutputEvents::BounceEvent(effects::Bounce {
                power: self.power,
            }));
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
