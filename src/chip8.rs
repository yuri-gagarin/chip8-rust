use crate::bus::Bus;
use super::cpu::{PROGRAM_START, CPU};

pub struct Chip8 {
    cpu: CPU,
    bus: Bus,
}


impl Chip8 {
    pub fn new() -> Chip8 {
        Chip8 {
            bus: Bus::new(),
            cpu: CPU::new(),
        }
    }
    pub fn load_rom(&mut self, data: &Vec<u8>) {
        // why do we need the offset //
        /*
        HiNT: take a look what is reserved at 0x000 - 0x200 
        http://devernay.free.fr/hacks/chip8/C8TECH10.HTM#memmap
        Section: Memory Map 
         */
        // let offset: u16 = 0x200; 
        for i in 0..data.len() {
            self.bus.ram_write_byte(PROGRAM_START + (i as u16), data[i])
        }
    }
    pub fn run_instruction(&mut self) {
        self.cpu.run_instruction(&mut self.bus);
    }
}