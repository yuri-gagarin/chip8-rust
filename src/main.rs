use core::panic;
use std::{fs::File, io::Read};
use std::time::{Duration, Instant};
use minifb::{Window, WindowOptions, Key};

use display::Display;

mod ram;
mod bus;
mod chip8;
mod cpu;
mod display;
mod emulator;
mod keyboard;

use chip8::{Chip8};


fn main() {
    let mut file = File::open("data/PONG").unwrap();
    let mut data = Vec::<u8>::new();
    file.read_to_end(&mut data).expect("DATA READ ERROR");

    const WIDTH: usize = 640;
    const HEIGHT: usize = 320;

    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];

    let mut last_instruction_time = Instant::now();
    let mut last_win_display_time = Instant::now();
    // window //

    let mut window = Window::new("CHIP8", WIDTH, HEIGHT, WindowOptions::default()).unwrap_or_else(|error| {panic!("Failed to launch window: {:?}", error)});
    // load //
    let mut chip8 = Chip8::new();
    chip8.load_rom(&data);

    while window.is_open() && !window.is_key_down(Key::Escape) {
        if Instant::now() - last_instruction_time > Duration::from_millis(2) {
            chip8.run_instruction();
            last_instruction_time = Instant::now();
        }
        if Instant::now() - last_win_display_time > Duration::from_millis(10) {
            let display_buffer = chip8.get_display_buffer();

            for y in 0..HEIGHT {
                let y_coordinate = y / 10;
                let screen_offset = y * WIDTH;
                for x in 0..WIDTH {
                    let x_coordinate = x / 10;
                    let idx = Display::get_index_from_coordinates(x_coordinate, y_coordinate);
                    let pixel = display_buffer[idx];

                    let displayed_pixel = match pixel {
                        0 => 0x0,
                        1 => 0xffffff,
                        _ => unreachable!("Can't resolve pixel color")
                    };
                    buffer[screen_offset + x] = displayed_pixel;
                }
            }

            window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap_or_else(|error| {panic!("Failed to update window: {:?}", error)});
            last_win_display_time = Instant::now();

        }
    }

}
