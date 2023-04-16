use std::fmt;

use super::ram::Ram;
pub const PROGRAM_START: u16 = 0x200;
const WRITE_ERROR_MSG: &str = "Write error in <fmt::Debug> for <CPU>";


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
        let current_instruction: u16 = (hi << 8) | lo;
        
        println!("Instruction - {:#X} - read: LO - {:#X}. HI - {:#X}", current_instruction, lo, hi);
        /* 
        if hi == 0 && lo == 0 {
            panic!();
        }
        */
        let nnn: u16 = current_instruction & 0x0FFF; // NNN address //
        let nn: u16 = current_instruction & 0x0FF;   // NN 8 bit constant //
        let n: u16 = current_instruction & 0x00F;    // N 4 bit constant // 
        let x: u16 = current_instruction & 0x0F00 >> 8; //
        let y: u16 = current_instruction & 0x00F0 >> 4;
        
        match (current_instruction & 0xF000) >> 12 {
            0x1 => {
                self.pc = nnn;
            },
            _   => panic!("Unrecognized instruction at {:#X}:{:#X}", self.pc, current_instruction)
        }

    }
}

impl fmt::Debug for CPU {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

        write!(f, "pc: {:#X}\n", self.pc).expect(WRITE_ERROR_MSG);
        write!(f, "vx: ").expect(WRITE_ERROR_MSG);
        for item in self.vx.iter() {
            write!(f, "{:#X}", *item).expect("Write error");
        }
        write!(f, "\n").expect(WRITE_ERROR_MSG);
        write!(f, "i: {:#X}\n", self.i)
    }
}

