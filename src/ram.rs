use super::emulator::SPRITES;

pub struct Ram {
    memory: [u8; 4096], // 4096 bytes of Ram - Chip - 8
}
impl Ram {
    pub fn new() -> Ram {
        let mut ram = Ram {
            memory: [0; 4096],
        };
        // load sprites //
        let mut i: usize = 0;
        for sprite in SPRITES.iter() {
            for val in sprite {
                ram.memory[i] = *val;
                i += 1;
            }
        }

        for i in 0..0x1ff {
            print!("{:#x} ", ram.memory[i]);
        }
        ram
    }
    pub fn write_byte(&mut self, mem_address: u16, value: u8) {
        self.memory[mem_address as usize] = value;
    }
    pub fn read_byte(&mut self, mem_address: u16) -> u8 {
        self.memory[mem_address as usize]
    }
}