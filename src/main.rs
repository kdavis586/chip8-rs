use std::{env, thread, time::{Duration, SystemTime}};
use raylib::prelude::*;

mod chip8;

const DISP_SCALE: i32 = 10;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("Unexpected number of args.")
    }
    let mut chip: chip8::Chip8 = chip8::Chip8::default();
    chip.init(&args[1]);

    let (mut rl, thread) = raylib::init()
        .size(64 * DISP_SCALE, 32 * DISP_SCALE)
        .title("Chip8 Display")
        .build();

    let mut timer_start = SystemTime::now();
    let mut cycle_start = SystemTime::now();
    while !rl.window_should_close() {
        match rl.get_key_pressed_number() {
            Some(k) => {
                match k {
                    48 => {chip.set_keypad(0x00);}, 
                    49 => {chip.set_keypad(0x01);}, 
                    50 => {chip.set_keypad(0x02);}, 
                    51 => {chip.set_keypad(0x03);}, 
                    52 => {chip.set_keypad(0x04);}, 
                    53 => {chip.set_keypad(0x05);}, 
                    54 => {chip.set_keypad(0x06);}, 
                    55 => {chip.set_keypad(0x07);}, 
                    56 => {chip.set_keypad(0x08);}, 
                    57 => {chip.set_keypad(0x09);}, 
                    65 => {chip.set_keypad(0x0A);}, 
                    66 => {chip.set_keypad(0x0B);}, 
                    67 => {chip.set_keypad(0x0C);}, 
                    68 => {chip.set_keypad(0x0D);},
                    69 => {chip.set_keypad(0x0E);}, 
                    70 => {chip.set_keypad(0x0F);}, 
                    _ => {},  
                }
            },
            None => {}, 
        }
        chip.cycle();

        let now = SystemTime::now();
        match now.duration_since(cycle_start) {
            Ok(duration) => {
                if duration.saturating_sub(Duration::from_millis(1000/100)) != Duration::ZERO {
                    chip.cycle();
                    cycle_start = now;
                }
            },
            Err(e) => {
                panic!("{}", e);
            },
        }
        match now.duration_since(timer_start) {
            Ok(duration) => {
                if duration.saturating_sub(Duration::from_millis(1000/60)) != Duration::ZERO {
                    chip.update_timers();
                    timer_start = now;
                }
                let disp_buffer: &[[u8; 64]; 32] = chip.get_display();
                let mut d = rl.begin_drawing(&thread);

                for y in 0..(disp_buffer.len() as i32) {
                    for x in 0..(disp_buffer[y as usize].len() as i32) {
                        let mut pixel_color: Color = Color::BEIGE; 
                        if disp_buffer[y as usize][x as usize] == 1 {
                            pixel_color = Color::NAVY; 
                        }
                        d.draw_rectangle(x * DISP_SCALE, y * DISP_SCALE, DISP_SCALE, DISP_SCALE, pixel_color);
                    } 
                }
            },
            Err(e) => {
                panic!("{}", e)
            },
        }
        thread::sleep(Duration::from_millis(2));
    }
}
