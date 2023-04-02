use super::emulator::SPRITES;

pub struct Ram {
    memory: [u8; 4096],
}
impl Ram {
    pub fn new() -> Ram {
        Ram {
            memory: [0; 4096],
        }
    }
    pub fn write_byte(&mut self, mem_address: u16, value: u8) {
        self.memory[mem_address as usize] = value;
    }
    pub fn read_byte(&mut self, mem_address: u16, value: u8) {

    }
}