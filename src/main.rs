use std::{env, thread, time::Duration};
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

    // TODO REMOVE
    while !rl.window_should_close() {
        chip.cycle();
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
        thread::sleep(Duration::from_millis(1));
    }
}