use agb::fixnum::Num;

use super::Entity;
use super::Camera;
use super::math;
use math::*;
use super::render;
use render::*;


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
            x_offset: Num::new(0),
            y_offset: Num::new(0),
            z_offset: Num::new(0),
            x_rotation: Num::new(0),
            y_rotation: Num::new(0),
            z_rotation: Num::new(0),
            points: [[Num::new(0); 3]; 8],
            model_rotated_points: [[Num::new(0); 3]; 8],
            x_rotation_matrix: [[Num::new(0); 3]; 3],
            y_rotation_matrix: [[Num::new(0); 3]; 3],
            z_rotation_matrix: [[Num::new(0); 3]; 3],
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

    fn set_size(&mut self, size: i32) {
        let radius = size >> 1;
        self.points = [
            [Num::new(radius), Num::new(radius), Num::new(radius)],
            [Num::new(-radius), Num::new(radius), Num::new(radius)],
            [Num::new(-radius), Num::new(-radius), Num::new(radius)],
            [Num::new(radius), Num::new(-radius), Num::new(radius)],
            [Num::new(radius), Num::new(radius), Num::new(-radius)],
            [Num::new(-radius), Num::new(radius), Num::new(-radius)],
            [Num::new(-radius), Num::new(-radius), Num::new(-radius)],
            [Num::new(radius), Num::new(-radius), Num::new(-radius)],
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

    fn set_vertex(&mut self, point: [Num<i32, 8>; 3], index: i32) {
        //not implemented
    }

    fn render(&self, bitmap4: &mut agb::display::bitmap4::Bitmap4, camera: &Camera) {
        let width: i32 = 240;
        let height: i32 = 160;
        let scale: Num<i32, 8> = Num::new(30); //100;
        let middle: [Num<i32, 8>; 2] = [Num::new(width / 2), Num::new(height / 2)]; // x, y

        let mut screenPoints: [[Num<i32, 8>; 2]; 8] = [[Num::new(0), Num::new(0)]; 8];
        let mut translatedPoints: [[Num<i32, 8>; 3]; 8] =
            [[Num::new(0), Num::new(0), Num::new(0)]; 8];

        let mut i = 0;

        for point in &self.points {
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

            if (z != zero) {
                let perspective_scale: Num<i32, 8> = scale / z;
                x = (translated_point[0] * perspective_scale) + middle[0];
                y = (translated_point[1] * perspective_scale) + middle[1];
            } else {
                x = middle[0];
                y = middle[1];
            }

            screenPoints[i] = [x, y];
            translatedPoints[i] = translated_point;
            i += 1;
        }
        if (backFaceCulling(&translatedPoints, 0, 1, 2)) {
            //draw_face_outline(&mut bitmap4, screenPoints, 0, 1, 2, 3);
            draw_triangle(
                bitmap4,
                screenPoints[0],
                screenPoints[1],
                screenPoints[2],
                1,
            );
            draw_triangle(
                bitmap4,
                screenPoints[0],
                screenPoints[2],
                screenPoints[3],
                1,
            );
        }
        if (backFaceCulling(&translatedPoints, 7, 6, 5)) {
            //draw_face_outline(&mut bitmap4, screenPoints, 7, 6, 5, 4);
            draw_triangle(
                bitmap4,
                screenPoints[7],
                screenPoints[6],
                screenPoints[5],
                1,
            );
            draw_triangle(
                bitmap4,
                screenPoints[7],
                screenPoints[5],
                screenPoints[4],
                1,
            );
        }

        if (backFaceCulling(&translatedPoints, 0, 3, 7)) {
            //draw_face_outline(&mut bitmap4, screenPoints, 0, 3, 7, 4);
            draw_triangle(
                bitmap4,
                screenPoints[0],
                screenPoints[3],
                screenPoints[7],
                2,
            );
            draw_triangle(
                bitmap4,
                screenPoints[0],
                screenPoints[7],
                screenPoints[4],
                2,
            );
        }
        if (backFaceCulling(&translatedPoints, 1, 5, 6)) {
            //draw_face_outline(&mut bitmap4, screenPoints, 1, 5, 6, 2);
            draw_triangle(
                bitmap4,
                screenPoints[1],
                screenPoints[5],
                screenPoints[6],
                2,
            );
            draw_triangle(
                bitmap4,
                screenPoints[1],
                screenPoints[6],
                screenPoints[2],
                2,
            );
        }

        if (backFaceCulling(&translatedPoints, 7, 3, 2)) {
            //draw_face_outline(&mut bitmap4, screenPoints, 7, 3, 2, 6);
            draw_triangle(
                bitmap4,
                screenPoints[7],
                screenPoints[3],
                screenPoints[2],
                3,
            );
            draw_triangle(
                bitmap4,
                screenPoints[7],
                screenPoints[2],
                screenPoints[6],
                3,
            );
        }
        if (backFaceCulling(&translatedPoints, 0, 4, 5)) {
            //draw_face_outline(&mut bitmap4, screenPoints, 0, 4, 5, 1);
            draw_triangle(
                bitmap4,
                screenPoints[0],
                screenPoints[4],
                screenPoints[5],
                3,
            );
            draw_triangle(
                bitmap4,
                screenPoints[0],
                screenPoints[5],
                screenPoints[1],
                3,
            );
        }
    }
}
