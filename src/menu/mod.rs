use agb::InternalAllocator;
use alloc::format;
use alloc::string::ToString;
use alloc::vec::Vec;

use crate::audio;
use crate::levels;
use crate::renderer;
use crate::textengine;
use crate::utils;

const HEADINGHEIGHT: u16 = 20;

pub fn levelmenu(
    selected_level: usize,
    input: &mut agb::input::ButtonController,
    page: &mut u16,
    vblank: &agb::interrupt::VBlank,
    sound: &agb::sound::dmg::Sound,
    completed_levels: &Vec<bool, InternalAllocator>
) -> (usize, bool) {
    let levelcount: usize = levels::levelstore::LEVELS.len();

    let mut selected_level: i32 = selected_level as i32;

    let character_count = levelcount.to_string().len();

    let x_center: i32 = 120;
    let total_width: i32 = (character_count * 12 - 4) as i32;
    let word_separation: i32 = 8;
    let padded_width = total_width + ((character_count as i32) * word_separation);
    let middle_level_left = x_center - (total_width / 2);

    let y: u16 = 80;

    loop {
        renderer::hw::fill(*page, 0);
        let color = 48;

        textengine::draw::write_line(52, HEADINGHEIGHT, "select level", color - 2, *page);
        //textengine::draw::write_line(86, 100, "a-play", color - 2, *page);

        input.update();

        if input.is_just_pressed(agb::input::Button::A) {
            break;
        }
        if input.is_just_pressed(agb::input::Button::RIGHT) {
            selected_level += 1;
            audio::play_sound(0, &vblank, &sound);
        }
        if input.is_just_pressed(agb::input::Button::LEFT) {
            selected_level -= 1;
            audio::play_sound(0, &vblank, &sound);
        }
        if input.is_just_pressed(agb::input::Button::B) {
            return (0, true);
        }

        utils::clamp(&mut selected_level, 0, (levelcount - 1) as i32);
        let first_visible_level = selected_level - 10;

        for i in first_visible_level..selected_level + 11 {
            if i >= 0 && (i as usize) < levelcount {
                let level = selected_level - i;

                let x: i32 = middle_level_left + (i - selected_level) * padded_width;
                if x <= 10 || x + padded_width >= 240 {
                    continue;
                }
                let x = x as u16;
                let mut shade: u16 = color - ((level) * 3).abs() as u16;
                if shade < 34 {
                    shade = 34;
                }

                let distance_from_middle = (level).abs() as u16;
                let v_offset; // = 0;
                if distance_from_middle == 0 {
                    v_offset = 0;
                } else if distance_from_middle == 1 {
                    v_offset = 1;
                } else if distance_from_middle == 2 {
                    v_offset = 4;
                } else {
                    v_offset = 8;
                }
                let level_str = (i + 1).to_string();
                let level_str = format!("{:0>width$}", level_str, width = character_count);
                textengine::draw::write_line(x, y - v_offset, &level_str, shade, *page);

                let completed = completed_levels[i as usize];
                if completed {
                    textengine::draw::write_tile(
                        (x + total_width as u16 - 2) as u16,
                        y - v_offset,
                        37,
                        10,
                        *page,
                    );
                }
            }
        }
        renderer::hw::flip(page);
    }

    return (selected_level as usize, false);
}

pub fn presstart(input: &mut agb::input::ButtonController, page: &mut u16) {
    renderer::hw::fill(*page, 0);
    let color = 48;

    textengine::draw::write_line(58, 140, "press start", color - 2, *page);

    renderer::hw::flip(page);

    loop {
        input.update();

        if input.is_just_pressed(agb::input::Button::A)
            || input.is_just_pressed(agb::input::Button::START)
        {
            break;
        }
    }
}

pub fn mainmenu(
    input: &mut agb::input::ButtonController,
    page: &mut u16,
    vblank: &agb::interrupt::VBlank,
    sound: &agb::sound::dmg::Sound,
) -> u16 {
    let color = 48;
    let mut option = 0;

    loop {
        renderer::hw::fill(*page, 0);

        textengine::draw::write_line(70, HEADINGHEIGHT, "main menu", color - 2, *page);

        textengine::draw::write_line(50, 80, "select level", color - 2, *page);
        textengine::draw::write_line(50, 100, "how to play", color - 2, *page);

        textengine::draw::write_line(40, 80 + 20 * option, "*", color - 2, *page);

        renderer::hw::flip(page);

        input.update();

        if input.is_just_pressed(agb::input::Button::A) {
            return option;
        }
        if input.is_just_pressed(agb::input::Button::DOWN)
            || input.is_just_pressed(agb::input::Button::UP)
        {
            option = 1 - option;
            audio::play_sound(0, &vblank, &sound);
        }
    }
}

pub fn info(input: &mut agb::input::ButtonController, page: &mut u16) {
    let color = 48;

    renderer::hw::fill(*page, 0);

    textengine::draw::write_line(94, HEADINGHEIGHT, "keys", color - 2, *page);

    textengine::draw::write_line(10, 60, "dpad-move", color - 2, *page);
    textengine::draw::write_line(10, 80, "a-jump", color - 2, *page);
    textengine::draw::write_line(10, 100, "b-toggle switch", color - 2, *page);
    textengine::draw::write_line(10, 120, "select-camera mode", color - 2, *page);

    renderer::hw::flip(page);

    loop {
        input.update();

        if input.is_just_pressed(agb::input::Button::B) {
            return;
        }
    }
}
