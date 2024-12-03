use agb::fixnum::Num;

trait Entity {
    fn render(&self);

    fn set_x_offset(&mut self, x_offset: Num<i32, 8>);
    fn set_y_offset(&mut self, y_offset: Num<i32, 8>);
    fn set_z_offset(&mut self, z_offset: Num<i32, 8>);

    fn set_x_rotation(&mut self, x_rotation: Num<i32, 8>);
    fn set_y_rotation(&mut self, y_rotation: Num<i32, 8>);
    fn set_z_rotation(&mut self, z_rotation: Num<i32, 8>);

    fn set_size(&mut self, size: i32);
    fn set_vertex(&mut self, point: [Num<i32, 8>; 3], index: i32);
}

struct Cube {
    x_offset: Num<i32, 8>,
    y_offset: Num<i32, 8>,
    z_offset: Num<i32, 8>,

    x_rotation: Num<i32, 8>,
    y_rotation: Num<i32, 8>,
    z_rotation: Num<i32, 8>,

    points: [[Num<i32, 8>; 3]; 8],
}

impl Entity for Cube {
    fn render(&self) {
        //todo: implement
    }

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
