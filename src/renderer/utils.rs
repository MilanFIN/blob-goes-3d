use super::Fixed;

pub fn init_palette(bitmap4: &mut agb::display::bitmap4::Bitmap4) {
    // Set  palette entries

    bitmap4.set_palette_entry(1, 0b0000000000011111);

    //red
    let red: u16 = 31;
    for i in 0..8 {
        let red_shade: u16 = red - (14 - 2 * i); //<< 10; blue is shifted 5 bits, red is not
        bitmap4.set_palette_entry((i).into(), red_shade);
    }
    let green: u16 = 31;
    for i in 0..8 {
        let shade: u16 = green - (14 - 2 * i) << 5;
        bitmap4.set_palette_entry((8+i).into(), shade);
    }
    let blue: u16 = 31;
    for i in 0..8 {
        let shade: u16 = blue - (14 - 2 * i) << 10;
        bitmap4.set_palette_entry((16+i).into(), shade);
    }


    //bitmap4.set_palette_entry(2, 0x3E0);
    //bitmap4.set_palette_entry(3, 0x7C00);
}

pub fn get_color(index: u8, angle: Fixed) -> u8 {
    const SUN_ANGLE: Fixed = Fixed::from_raw(32);
    const BASE_COLOR: Fixed = Fixed::const_new(7);
    const QUARTER: Fixed = Fixed::from_raw(64);

    // Calculate the shortest angular difference
    let raw_diff = (angle - SUN_ANGLE).abs();
    let a2 = raw_diff.modulo(Fixed::const_new(1));

    let diff = a2.min(Fixed::const_new(1) - a2);

    // Clamp the difference to the range [0, 0.25] and calculate the index
    let clamped_diff = diff.min(QUARTER);
    let offset = BASE_COLOR * (Fixed::const_new(1) - clamped_diff / QUARTER);

    // Convert to integer index
    return (offset.trunc()) as u8 + index*8;
}
