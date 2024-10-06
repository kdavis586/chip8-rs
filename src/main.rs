use std::env;

mod chip8;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("Unexpected number of args.")
    }
    let mut chip: chip8::Chip8 = chip8::Chip8::default();
    chip.init(&args[1]);

    let (mut rl, thread) = raylib::init()
        .size(640, 480)
        .title("Hello, World")
        .build();

    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);

        d.clear_background(Color::WHITE);
        d.draw_text("Hello, world!", 12, 12, 20, Color::BLACK);
    }

    loop {
        // TODO runs as fast as possible, limit to a target Hz
        chip.cycle();
        chip.get_display();
    }
}