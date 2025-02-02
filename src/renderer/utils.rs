use super::Fixed;

pub const PROJECTION_MATRIX: [[Fixed; 4]; 4] = [
    [
        Fixed::from_raw(171), //(0.66666667),
        Fixed::from_raw(0),
        Fixed::from_raw(0),
        Fixed::from_raw(0),
    ],
    [
        Fixed::from_raw(0),
        Fixed::from_raw(256), //(1.0),
        Fixed::from_raw(0),
        Fixed::from_raw(0),
    ],
    [
        Fixed::from_raw(0),
        Fixed::from_raw(0),
        Fixed::from_raw(-256), //(-1.00020002),
        Fixed::from_raw(-51),  //(-0.20002),
    ],
    [
        Fixed::from_raw(0),
        Fixed::from_raw(0),
        Fixed::from_raw(-256), //(-1.0),
        Fixed::from_raw(0),
    ],
];

fn init_palette_slice(
    bitmap4: &mut agb::display::bitmap4::Bitmap4,
    r: i32,
    g: i32,
    b: i32,
    i: u32,
    scale: i32,
) {
    let red: Fixed = Fixed::new(r);
    let green: Fixed = Fixed::const_new(g);
    let blue: Fixed = Fixed::const_new(b);

    for j in 0..8 {
        let red_shade: u16 = (red - (red / 8 / scale) * j as i32).trunc() as u16;
        let green_shade: u16 = ((green - (green / 8 / scale) * j as i32).trunc() << 5) as u16;
        let blue_shade: u16 = ((blue - (blue / 8 / scale) * j as i32).trunc() << 10) as u16;
        bitmap4.set_palette_entry((i * 8 - j) as u32, red_shade | green_shade | blue_shade);
    }
}

pub fn init_palette(bitmap4: &mut agb::display::bitmap4::Bitmap4) {
    // Set  palette entries
    init_palette_slice(bitmap4, 31, 0, 0, 1, 2);
    init_palette_slice(bitmap4, 0, 31, 0, 2, 2);
    init_palette_slice(bitmap4, 0, 0, 31, 3, 2);
    init_palette_slice(bitmap4, 15, 10, 7, 4, 2);
    init_palette_slice(bitmap4, 31, 31, 0, 5, 2);

}

pub fn get_color(index: u16, shade: i16) -> u16 {
    if shade == 0 {
        return index * 8 + 7;
    } else if shade == 1 {
        return index * 8 + 5;
    } else if shade == 2 {
        return index * 8 + 4;
    }
    return index*8+3;

}

pub fn polygon_avg_z(points: &[[Fixed; 3]], a: usize, b: usize, c:usize) -> Fixed {
    // let x = points[a][0] + points[b][0] + points[c][0] / Fixed::const_new(3);
    // let y = points[a][1] + points[b][1] + points[c][1] / Fixed::const_new(3);
    // let z = points[a][2] + points[b][2] + points[c][2] / Fixed::const_new(3);
    // return math::vector_square_len([x,y,z]);
    return (points[a][2] + points[b][2] + points[c][2]) / Fixed::const_new(3);
}

pub fn polygon_avg_z_2(points: &[[Fixed; 3]], a: usize, b: usize) -> Fixed {
    // let x = points[a][0] + points[b][0]  / Fixed::const_new(2);
    // let y = points[a][1] + points[b][1] / Fixed::const_new(2);
    // let z = points[a][2] + points[b][2] / Fixed::const_new(2);
    // return math::vector_square_len([x,y,z]);
    return (points[a][2] + points[b][2]) / Fixed::const_new(2);
}


#[inline(always)]
pub fn safe_fraction_fixed(numerator: Fixed, denominator: Fixed) -> Fixed {
    if denominator == 0 {
        return Fixed::const_new(0);
    }
    return numerator / denominator;
}
