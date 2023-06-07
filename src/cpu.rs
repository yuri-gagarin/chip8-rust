use std::fmt;
use super::bus::Bus;
pub const PROGRAM_START: u16 = 0x200;
const WRITE_ERROR_MSG: &str = "Write error in <fmt::Debug> for <CPU>";


pub struct CPU {
    vx: [u8; 16], // Vx register - 16 general purpose 8-bit registers
    i: u16,       // 16-bit register, generally used to store memory addresses, only the lowest 12 bits are usually used
    pc: u16,      // store the currently executing address
    return_stack: Vec<u16>,
}

impl CPU {
    pub fn new() -> CPU {
        CPU { 
            vx: [0; 16], 
            i: 0, 
            pc: PROGRAM_START,
            return_stack: Vec::<u16>::new(),
        }
    }
    // write and read to VX //
    pub fn write_reg_vx(&mut self, index: u8, value: u8) {
        self.vx[index as usize] = value;
    }
    pub fn read_reg_vx(&mut self, index: u8) -> u8 {
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
    pub fn run_instruction(&mut self, bus: &mut Bus) {
        let low = bus.ram_read_byte(self.pc) as u16;
        let high = bus.ram_read_byte(self.pc + 1) as u16;
        let current_instruction: u16 = (high << 8) | low;
        
        println!("Instruction - {:#X} - read: LO - {:#X}. HI - {:#X}", current_instruction, low, high);
        /* 
        if hi == 0 && lo == 0 {
            panic!();
        }
        */
        let nnn: u16 = current_instruction & 0x0FFF;            // NNN address //
        let nn: u8 = (current_instruction & 0x0FF) as u8;       // NN 8 bit constant //
        let n: u8 = (current_instruction & 0x00F) as u8;        // N 4 bit constant // 
        let x: u8 = (current_instruction & 0x0F00 >> 8) as u8;  //
        let y: u8 = (current_instruction & 0x00F0 >> 4) as u8;  //
        
        match (current_instruction & 0xF000) >> 12 {
            0x0 => {
                match nn {
                    0xE0 => {
                        bus.display_clear_screen();
                        self.pc += 2;
                    }
                    0xEE => {
                        // get and return from subroutinge //
                        let address: u16 = self.return_stack.pop().unwrap();
                        self.pc = address;
                    }
                    _ => panic!(
                        "Unrecognized 0x00** instruction {:#X}:{:#X}",
                        self.pc,
                        current_instruction
                    ),
                }
            }
            0x1 => {
                // Jump to location nnn //
                //The interpreter sets the program counter to nnn //
                self.pc = nnn;
            },
            0x2 => {
                // call a subroutine at NNN //
                // The interpreter increments the stack pointer, then puts the current PC on the top of the stack. The PC is then set to nnn. //
                self.return_stack.push(self.pc + 2);
                self.pc = nnn;
            }
            0x3 => {
                // Skip next instruction if Vx = nn //
                // The interpreter compares register Vx to nn, and if they are equal, increments the program counter by 2 //
                let vx = self.read_reg_vx(x);
                if vx == nn {
                    self.pc += 4;
                } else {
                    self.pc += 2;
                }
            }
            0x4 => {
                // Skip next instruction if Vx != nn //
                // The interpreter compares register Vx to nn, and if they are not equal, increments the program counter by 2 //
                let vx = self.read_reg_vx(x);
                if vx != nn {
                    self.pc += 4;
                } else {
                    self.pc += 2;
                }
            }
            0x5 => {
                // Skip next instruction if Vx = Vy //
                // The interpreter compares register Vx to register Vy, and if they are equal, increments the program counter by 2. //
                let vx = self.read_reg_vx(x);
                let vy = self.read_reg_vx(y);
                if vx == vy {
                    self.pc += 4;
                } else {
                    self.pc += 2;
                }
            }
            0x6 => {
                // Set Vx = nn //
                // The interpreter puts the value nn into register Vx //
                self.write_reg_vx(x, nn);
                self.pc += 2;
            }
            0x7 => {
                // Set Vx = Vx + nn //
                // Adds the value nn to the value of register Vx, then stores the result in Vx //
                let vx = self.read_reg_vx(x);
                self.write_reg_vx(x, vx.wrapping_add(nn));
                self.pc += 2;

            }
            0x8 => {
               
                let vy = self.read_reg_vx(y);
                let vx = self.read_reg_vx(x);
                match n {
                    0x0 => {
                        // Set Vx = Vy //
                        // Stores the value of register Vy in register Vx //
                        self.write_reg_vx(x, vy);
                    }
                    0x2 => {
                        // Set Vx = Vx AND Vy. //
                        // Performs a bitwise AND on the values of Vx and Vy, then stores the result in Vx. A bitwise AND compares the corresponding bits from two values, and if both bits are 1, then the same bit in the result is also 1. Otherwise, it is 0.
                        let res = vx & vy;
                        self.write_reg_vx(x, res);
                    }
                    0x3 => {
                        // Set Vx = Vx XOR Vy. //
                        // Performs a bitwise exclusive OR on the values of Vx and Vy, then stores the result in Vx. An exclusive OR compares the corrseponding bits from two values, and if the bits are not both the same, then the corresponding bit in the result is set to 1. Otherwise, it is 0. //
                        let res = vx ^ vy;
                        self.write_reg_vx(x, res);

                    }
                    0x4 => {
                        //  Set Vx = Vx + Vy, set VF = carry. //
                        //  The values of Vx and Vy are added together. If the result is greater than 8 bits (i.e., > 255,) VF is set to 1, otherwise 0. Only the lowest 8 bits of the result are kept, and stored in Vx. //
                        let res = vx + vy;
                        self.write_reg_vx(x, res);
                        if res > 0xFF {
                            self.write_reg_vx(0xF, 1);
                        }
                    }
                    0x5 => {
                        //  Set Vx = Vx - Vy, set VF = NOT borrow. //
                        //  If Vx > Vy, then VF is set to 1, otherwise 0. Then Vy is subtracted from Vx, and the results stored in Vx. //
                        let result: i8 = (vx - vy) as i8;
                        self.write_reg_vx(x, result);
                        if result < 0 {
                            self.write_reg_vx(0xF, 1);
                        } else {
                            self.write_reg_vx(0xF, 0);
                        }
                    }
                }
            }
            0xD => {
                self.debug_draw_sprite(bus, x, y, 100);
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

