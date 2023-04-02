use super::ram::{Ram};

pub struct Chip8 {
    ram: Ram,
}


impl Chip8 {
    pub fn new() -> Chip8 {
        Chip8 {
            ram: Ram::new(),
        }
    }
    pub fn load_rom(&mut self, data: &Vec<u8>) {
        // why do we need the offset //
        /*
        HiNT: take a look what is reserved at 0x000 - 0x200 
        http://devernay.free.fr/hacks/chip8/C8TECH10.HTM#memmap
        Section: Memory Map 
         */
        let offset: u16 = 0x200; 
        for i in 0..data.len() {
            self.ram.write_byte((offset + (i as u16)), data[i])
        }

    }
}