use agb::fixnum::Num;

use crate::math;
use math::*;

use crate::render;
use render::*;

use crate::camera;
use camera::*;

#[derive(Copy, Clone)]
pub enum EntityEnum {
    Cube(Cube),
    Empty(Empty),
}

impl EntityEnum {
    pub fn set_x_offset(&mut self, offset: Num<i32, 8>) {
        match self {
            EntityEnum::Cube(c) => c.set_x_offset(offset),
            EntityEnum::Empty(e) => e.set_y_offset(offset),
        }
    }
    pub fn set_y_offset(&mut self, offset: Num<i32, 8>) {
        match self {
            EntityEnum::Cube(c) => c.set_x_offset(offset),
            EntityEnum::Empty(e) => e.set_y_offset(offset),
        }
    }
    pub fn set_z_offset(&mut self, offset: Num<i32, 8>) {
        match self {
            EntityEnum::Cube(c) => c.set_z_offset(offset),
            EntityEnum::Empty(e) => e.set_z_offset(offset),
        }
    }
    pub fn set_x_rotation(&mut self, rot: Num<i32, 8>) {
        match self {
            EntityEnum::Cube(c) => c.set_x_rotation(rot),
            EntityEnum::Empty(e) => e.set_x_rotation(rot),
        }
    }
    pub fn set_y_rotation(&mut self, rot: Num<i32, 8>) {
        match self {
            EntityEnum::Cube(c) => c.set_y_rotation(rot),
            EntityEnum::Empty(e) => e.set_y_rotation(rot),
        }
    }
    pub fn set_z_rotation(&mut self, rot: Num<i32, 8>) {
        match self {
            EntityEnum::Cube(c) => c.set_z_rotation(rot),
            EntityEnum::Empty(e) => e.set_z_rotation(rot),
        }
    }
    pub fn set_size(&mut self, size: i32) {
        match self {
            EntityEnum::Cube(c) => c.set_size(size),
            EntityEnum::Empty(e) => e.set_size(size),
        }
    }
    pub fn render(&self, bitmap4: &mut agb::display::bitmap4::Bitmap4, camera: &Camera) {
        match self {
            EntityEnum::Cube(c) => c.render(bitmap4, camera),
            EntityEnum::Empty(e) => e.render(bitmap4, camera),
        }
    }
}

pub trait Entity {
    fn render(&self, bitmap4: &mut agb::display::bitmap4::Bitmap4, camera: &Camera);

    fn set_x_offset(&mut self, x_offset: Num<i32, 8>);
    fn set_y_offset(&mut self, y_offset: Num<i32, 8>);
    fn set_z_offset(&mut self, z_offset: Num<i32, 8>);

    fn set_x_rotation(&mut self, x_rotation: Num<i32, 8>);
    fn set_y_rotation(&mut self, y_rotation: Num<i32, 8>);
    fn set_z_rotation(&mut self, z_rotation: Num<i32, 8>);

    fn set_size(&mut self, size: i32);
    fn set_vertex(&mut self, point: [Num<i32, 8>; 3], index: i32);
}

#[derive(Copy, Clone)]
pub struct Cube {
    x_offset: Num<i32, 8>,
    y_offset: Num<i32, 8>,
    z_offset: Num<i32, 8>,

    x_rotation: Num<i32, 8>,
    y_rotation: Num<i32, 8>,
    z_rotation: Num<i32, 8>,

    points: [[Num<i32, 8>; 3]; 8],
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
    }

    fn set_y_rotation(&mut self, y_rotation: Num<i32, 8>) {
        self.y_rotation = y_rotation;
    }

    fn set_z_rotation(&mut self, z_rotation: Num<i32, 8>) {
        self.z_rotation = z_rotation;
    }

    fn set_vertex(&mut self, point: [Num<i32, 8>; 3], index: i32) {
        //not implemented
    }

    fn render(&self, bitmap4: &mut agb::display::bitmap4::Bitmap4, camera: &Camera) {
        let width: i32 = 240;
        let height: i32 = 160;
        let scale: Num<i32, 8> = Num::new(30); //100;
        let middle: [Num<i32, 8>; 2] = [Num::new(width / 2), Num::new(height / 2)]; // x, y

        let rotX: [[Num<i32, 8>; 3]; 3] = [
            [Num::new(1), Num::new(0), Num::new(0)],
            [Num::new(0), self.x_rotation.cos(), -self.x_rotation.sin()],
            [Num::new(0), self.x_rotation.sin(), self.x_rotation.cos()],
        ];

        let rotY: [[Num<i32, 8>; 3]; 3] = [
            [self.y_rotation.cos(), Num::new(0), self.y_rotation.sin()],
            [Num::new(0), Num::new(1), Num::new(0)],
            [-self.y_rotation.sin(), Num::new(0), self.y_rotation.cos()],
        ];

        let rotZ: [[Num<i32, 8>; 3]; 3] = [
            [self.z_rotation.cos(), -self.z_rotation.sin(), Num::new(0)],
            [self.z_rotation.sin(), self.z_rotation.cos(), Num::new(0)],
            [Num::new(0), Num::new(0), Num::new(1)],
        ];

        let mut screenPoints: [[Num<i32, 8>; 2]; 8] = [[Num::new(0), Num::new(0)]; 8];
        let mut translatedPoints: [[Num<i32, 8>; 3]; 8] =
            [[Num::new(0), Num::new(0), Num::new(0)]; 8];

        let mut i = 0;

        for point in &self.points {
            let mut rotated_point: [Num<i32, 8>; 3] = matmul(rotX, *point);
            rotated_point = matmul(rotY, rotated_point);
            rotated_point = matmul(rotZ, rotated_point);

            //todo: need to use both object x, y z and world x, y & z
            //that way we could rotate the entire scene
            let mut translated_point: [Num<i32, 8>; 3] = rotated_point;
            translated_point[0] += self.x_offset - camera.x;
            translated_point[1] += self.y_offset - camera.y;
            translated_point[2] += self.z_offset - camera.z;

            // might want to perform world rotation and translation here later on
            //in that case, there would be a common rotx, y and z for all objects to rotate the scene around
            let mut rotated_point: [Num<i32, 8>; 3] = matmul(rotX, translated_point);
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

#[derive(Copy, Clone)]
pub struct Empty {}
impl Entity for Empty {
    fn render(&self, bitmap4: &mut agb::display::bitmap4::Bitmap4, camera: &Camera) {}
    fn set_x_offset(&mut self, x_offset: Num<i32, 8>) {}
    fn set_y_offset(&mut self, y_offset: Num<i32, 8>) {}
    fn set_z_offset(&mut self, z_offset: Num<i32, 8>) {}
    fn set_size(&mut self, size: i32) {}
    fn set_x_rotation(&mut self, x_rotation: Num<i32, 8>) {}
    fn set_y_rotation(&mut self, y_rotation: Num<i32, 8>) {}
    fn set_z_rotation(&mut self, z_rotation: Num<i32, 8>) {}
    fn set_vertex(&mut self, point: [Num<i32, 8>; 3], index: i32) {}
}
impl Empty {
    pub fn default() -> Self {
        Self {}
    }
}

/*
    let vertices = [10, 20, 30, 40, 50]; // Example i32 array
    let count = vertices.len() as i32;

    // Pass a pointer to the array
    set_vertices(vertices.as_ptr(), count);

    unsafe {
    for i in 0..count {
        // Dereference the pointer to access the array elements
        println!("Vertex {}: {}", i, *vertices.offset(i as isize));
    }
}
*/
