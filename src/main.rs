use std::{fs::File, io::Read};

mod ram;
mod chip8;
mod emulator;

use chip8::{Chip8};


fn main() {
    let mut file = File::open("data/INVADERS").unwrap();
    let mut data = Vec::<u8>::new();
    file.read_to_end(&mut data).expect("DATA READ ERROR");

    // load //
    let mut chip8 = Chip8::new();
    chip8.load_rom(&data);

}
