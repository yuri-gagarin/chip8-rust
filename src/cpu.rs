use super::ram::Ram;
pub const PROGRAM_START: u16 = 0x200;

pub struct CPU {
    vx: [u8; 16], // Vx register - 16 general purpose 8-bit registers
    i: u16,       // 16-bit register, generally used to store memory addresses, only the lowest 12 bits are usually used
    pc: u16       // store the currently executing address
}

impl CPU {
    pub fn new() -> CPU {
        CPU { 
            vx: [0; 16], 
            i: 0, 
            pc: PROGRAM_START 
        }
    }
    pub fn run_instruction(&mut self, ram: &mut Ram) {
        let lo = ram.read_byte(self.pc) as u16;
        let hi = ram.read_byte(self.pc + 1) as u16;
        let current_instruction: u16 = (lo << 8) as u16 | hi as u16;
        println!("Instruction - {:#X} - read: LO - {:#X}. HI - {:#X}", current_instruction, lo, hi);
        if hi == 0 && lo == 0 {
            panic!();
        }
        self.pc += 2;
    }
}

