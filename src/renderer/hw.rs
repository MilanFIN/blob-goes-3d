const MODE_4_PAGE_1: *mut u16 = 0x600_0000 as *mut u16;
const MODE_4_PAGE_2: *mut u16 = 0x600_A000 as *mut u16;
const REG_DISPCNT: *mut u32 = 0x0400_0000 as *mut u32;
const DCNT_PAGE: u32 = 0x0010;

pub fn draw_wide_point(x: i32, y: i32, color: u32, page: u32) {
    let index = (y * 240 + x) / 2;
    let value = ((color as u16) << 8) | (color as u16);

    unsafe {
        if page == 1 {
            *MODE_4_PAGE_1.add(index as usize) = value;
        } else {
            *MODE_4_PAGE_2.add(index as usize) = value;
        }
    }
}

pub fn fill(page: u32, color: u8) {
    let active_page = if page == 1 {
        MODE_4_PAGE_1
    } else {
        MODE_4_PAGE_2
    };
    let value = ((color as u16) << 8) | (color as u16);

    unsafe {
        for i in 0..19200 {
            *active_page.add(i) = value;
        }
    }
}

pub fn flip(page: &mut u32) {
    *page = 1 - *page;
    unsafe {
        *REG_DISPCNT ^= DCNT_PAGE;
    };
}
