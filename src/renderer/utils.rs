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
    //bitmap4.set_palette_entry(1, 0b0000000000011111);
    init_palette_slice(bitmap4, 31, 0, 0, 1, 2);
    init_palette_slice(bitmap4, 0, 31, 0, 2, 2);
    init_palette_slice(bitmap4, 0, 0, 31, 3, 2);
    init_palette_slice(bitmap4, 15, 10, 7, 4, 2);
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
    /*
    const SUN_ANGLE: Fixed = Fixed::from_raw(32);
    const BASE_COLOR: Fixed = Fixed::const_new(7);
    const QUARTER: Fixed = Fixed::from_raw(128);

    // Calculate the shortest angular difference
    let raw_diff = (angle - SUN_ANGLE).abs();
    let a2 = raw_diff.modulo(Fixed::const_new(1));

    let diff = a2.min(Fixed::const_new(1) - a2);

    // Clamp the difference to the range [0, 0.25] and calculate the index
    let clamped_diff = diff.min(QUARTER);
    let offset = BASE_COLOR * (Fixed::const_new(1) - clamped_diff / QUARTER);

    // Convert to integer index
    return ((offset.trunc()) as u16) + index * 8;
    */
}
