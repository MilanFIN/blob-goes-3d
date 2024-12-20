use agb::fixnum::Num;

use crate::new_num;

use super::math;
use super::render;
use super::Camera;
use super::Entity;
use super::utils;
use math::*;
use render::*;
use utils::*;

#[derive(Copy, Clone)]
pub struct Cube {
    x_offset: Num<i32, 8>,
    y_offset: Num<i32, 8>,
    z_offset: Num<i32, 8>,

    x_rotation: Num<i32, 8>,
    y_rotation: Num<i32, 8>,
    z_rotation: Num<i32, 8>,

    points: [[Num<i32, 8>; 3]; 8],
    model_rotated_points: [[Num<i32, 8>; 3]; 8],

    x_rotation_matrix: [[Num<i32, 8>; 3]; 3],
    y_rotation_matrix: [[Num<i32, 8>; 3]; 3],
    z_rotation_matrix: [[Num<i32, 8>; 3]; 3],
}

impl Cube {
    pub fn default() -> Self {
        Self {
            x_offset: new_num(0),
            y_offset: new_num(0),
            z_offset: new_num(0),
            x_rotation: new_num(0),
            y_rotation: new_num(0),
            z_rotation: new_num(0),
            points: [[new_num(0); 3]; 8],
            model_rotated_points: [[new_num(0); 3]; 8],
            x_rotation_matrix: [[new_num(0); 3]; 3],
            y_rotation_matrix: [[new_num(0); 3]; 3],
            z_rotation_matrix: [[new_num(0); 3]; 3],
        }
    }

}

impl Entity for Cube {
    fn set_x_offset(&mut self, x_offset: Num<i32, 8>) {
        self.x_offset = x_offset;
    }

    fn set_y_offset(&mut self, y_offset: Num<i32, 8>) {
        self.y_offset = y_offset;
    }

    fn set_z_offset(&mut self, z_offset: Num<i32, 8>) {
        self.z_offset = z_offset;
    }

    fn set_size(&mut self, size: Num<i32, 8>) {
        let radius: Num<i32, 8> = size / 2; 
        self.points = [
            [(radius), (radius), (radius)],
            [(-radius), (radius), (radius)],
            [(-radius), (-radius), (radius)],
            [(radius), (-radius), (radius)],
            [(radius), (radius), (-radius)],
            [(-radius), (radius), (-radius)],
            [(-radius), (-radius), (-radius)],
            [(radius), (-radius), (-radius)],
        ];
    }

    fn set_x_rotation(&mut self, x_rotation: Num<i32, 8>) {
        self.x_rotation = x_rotation;
        self.x_rotation_matrix = [
            [Num::new(1), Num::new(0), Num::new(0)],
            [Num::new(0), self.x_rotation.cos(), -self.x_rotation.sin()],
            [Num::new(0), self.x_rotation.sin(), self.x_rotation.cos()],
        ];
    }

    fn set_y_rotation(&mut self, y_rotation: Num<i32, 8>) {
        self.y_rotation = y_rotation;
        self.y_rotation_matrix = [
            [self.y_rotation.cos(), Num::new(0), self.y_rotation.sin()],
            [Num::new(0), Num::new(1), Num::new(0)],
            [-self.y_rotation.sin(), Num::new(0), self.y_rotation.cos()],
        ];
    }

    fn set_z_rotation(&mut self, z_rotation: Num<i32, 8>) {
        self.z_rotation = z_rotation;
        self.z_rotation_matrix = [
            [self.z_rotation.cos(), -self.z_rotation.sin(), Num::new(0)],
            [self.z_rotation.sin(), self.z_rotation.cos(), Num::new(0)],
            [Num::new(0), Num::new(0), Num::new(1)],
        ];
    }

    fn refresh_model_matrix(&mut self) {
        for i in 0..self.points.len() {
            let point: &[Num<i32, 8>; 3] = &self.points[i];

            let mut rotated_point: [Num<i32, 8>; 3] = matmul(self.x_rotation_matrix, *point);
            rotated_point = matmul(self.y_rotation_matrix, rotated_point);
            rotated_point = matmul(self.z_rotation_matrix, rotated_point);

            self.model_rotated_points[i] = rotated_point;
        }
    }

    fn set_vertex(&mut self, _point: [Num<i32, 8>; 3], _index: i32) {
        //not implemented
    }

    fn render(&self, bitmap4: &mut agb::display::bitmap4::Bitmap4, camera: &Camera) {
        let width: i32 = 240;
        let height: i32 = 160;
        let scale: Num<i32, 8> = Num::new(30); //100;
        let middle: [Num<i32, 8>; 2] = [Num::new(width / 2), Num::new(height / 2)]; // x, y

        let mut screen_points: [[Num<i32, 8>; 2]; 8] = [[Num::new(0), Num::new(0)]; 8];
        let mut translated_points: [[Num<i32, 8>; 3]; 8] =
            [[Num::new(0), Num::new(0), Num::new(0)]; 8];


        for i in 0..self.model_rotated_points.len(){
            /*let mut rotated_point: [Num<i32, 8>; 3] = matmul(self.x_rotation_matrix, *point);
            rotated_point = matmul(self.y_rotation_matrix, rotated_point);
            rotated_point = matmul(self.z_rotation_matrix, rotated_point);*/

            let mut translated_point: [Num<i32, 8>; 3] = self.model_rotated_points[i];
            translated_point[0] += self.x_offset - camera.x;
            translated_point[1] += self.y_offset - camera.y;
            translated_point[2] += self.z_offset - camera.z;

            translated_point = matmul(camera.x_rotation_matrix, translated_point);
            translated_point = matmul(camera.y_rotation_matrix, translated_point);
            translated_point = matmul(camera.z_rotation_matrix, translated_point);

            // might want to perform world rotation here later on
            //in that case, there would be a common rotx, y and z for all objects to rotate the scene around

            //perspective
            let z: Num<i32, 8> = translated_point[2];
            let zero: Num<i32, 8> = Num::new(0);
            let x: Num<i32, 8>;
            let y: Num<i32, 8>;

            if z != zero {
                let perspective_scale: Num<i32, 8> = scale / z;
                x = (translated_point[0] * perspective_scale) + middle[0];
                y = (translated_point[1] * perspective_scale) + middle[1];
            } else {
                x = middle[0];
                y = middle[1];
            }

            screen_points[i] = [x, y];
            translated_points[i] = translated_point;
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
    

    fn distance_from_camera(&self, camera: &Camera) -> Num<i32, 8> {
        return abs(self.x_offset - camera.x)
            + abs(self.y_offset - camera.y)
            + abs(self.z_offset - camera.z);
    }
}
