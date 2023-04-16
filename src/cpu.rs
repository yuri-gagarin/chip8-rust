use std::fmt;
use super::ram::Ram;
use super::bus::Bus;
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
    // write and read to VX //
    pub fn write_reg_vx(&mut self, index: u16, value: u8) {
        self.vx[index as usize] = value;
    }
    pub fn read_reg_vx(&mut self, index: u16) -> u8 {
        self.vx[index as usize]
    }

    pub fn debug_draw_sprite(&mut self, bus: &mut Bus, x: u8, y: u8, height: u8) {
        let mut should_set_vf = false;
        for sprite_y in 0..height {
            let b = bus.ram_read_byte(self.i + sprite_y as u16);
            if bus.display_draw_byte(b, x, y + sprite_y) {
                should_set_vf = true;
            }
        }
    }
    // instructions //
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
        let nnn: u16 = current_instruction & 0x0FFF;        // NNN address //
        let nn: u8 = (current_instruction & 0x0FF) as u8;   // NN 8 bit constant //
        let n: u8 = (current_instruction & 0x00F) as u8;    // N 4 bit constant // 
        let x: u16 = current_instruction & 0x0F00 >> 8;     //
        let y: u16 = current_instruction & 0x00F0 >> 4;
        
        match (current_instruction & 0xF000) >> 12 {
            0x1 => {
                // goto NNN //
                self.pc = nnn;
            },
            0x6 => {
                self.write_reg_vx(x, nn);
                self.pc += 2;
            }
            0xD => {
                self.debug_draw_sprite(n);
            }
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

