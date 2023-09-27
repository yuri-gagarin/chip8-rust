use std::fmt;
use super::bus::Bus;
use rand::{thread_rng, Rng};

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
        if should_set_vf {
            self.write_reg_vx(0xF, 1);
        } else {
            self.write_reg_vx(0xF, 0);
        }
    }
    // instructions //
    pub fn run_instruction(&mut self, bus: &mut Bus) {
        let high = bus.ram_read_byte(self.pc) as u16;
        let low = bus.ram_read_byte(self.pc + 1) as u16;
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
        let x: u8 = (current_instruction & 0x0F00 >> 8) as u8;  // x 
        let y: u8 = (current_instruction & 0x00F0 >> 4) as u8;  // y
        
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
                        self.write_reg_vx(x, vx & vy);
                    }
                    0x3 => {
                        // Set Vx = Vx XOR Vy. //
                        // Performs a bitwise exclusive OR on the values of Vx and Vy, then stores the result in Vx. An exclusive OR compares the corrseponding bits from two values, and if the bits are not both the same, then the corresponding bit in the result is set to 1. Otherwise, it is 0. //
                        self.write_reg_vx(x, vx ^ vy);

                    }
                    0x4 => {
                        //  Set Vx = Vx + Vy, set VF = carry. //
                        //  The values of Vx and Vy are added together. If the result is greater than 8 bits (i.e., > 255,) VF is set to 1, otherwise 0. Only the lowest 8 bits of the result are kept, and stored in Vx. //
                        let res = vx as u16 + vy as u16;
                        self.write_reg_vx(x, res as u8);
                        if res > 0xFF {
                            self.write_reg_vx(0xF, 1);
                        }
                    }
                    0x5 => {
                        //  Set Vx = Vx - Vy, set VF = NOT borrow. //
                        //  If Vx > Vy, then VF is set to 1, otherwise 0. Then Vy is subtracted from Vx, and the results stored in Vx. //
                        let result: i8 = vx as i8 - vy as i8;                    
                        self.write_reg_vx(x, result as u8);
                        if result < 0 {
                            self.write_reg_vx(0xF, 1);
                        } else {
                            self.write_reg_vx(0xF, 0);
                        }
                    }
                    0x6 => {
                        //  Set Vx = Vx SHR 1. //
                        //  If the least-significant bit of Vx is 1, then VF is set to 1, otherwise 0. Then Vx is divided by 2. //
                        self.write_reg_vx(0xF, vx & 0x1);
                        self.write_reg_vx(x, vx >> 1);
                    }
                    0x7 => {
                        //  Set Vx = Vy - Vx, set VF = NOT borrow. //
                        //  If Vy > Vx, then VF is set to 1, otherwise 0. Then Vx is subtracted from Vy, and the results stored in Vx. //
                        let result: i8 = vy as i8 - vx as i8;
                        self.write_reg_vx(x, result as u8);
                        if result < 0 {
                            self.write_reg_vx(0xF, 1);
                        } else {
                            self.write_reg_vx(0xF, 0);
                        }
                    }
                    0xE => {
                        //  Set Vx = Vx SHL 1. //
                        //  If the most-significant bit of Vx is 1, then VF is set to 1, otherwise to 0. Then Vx is multiplied by 2. //
                        self.write_reg_vx(0xF, (vx & (0x80)) >> 7);
                        self.write_reg_vx(x, vx << 1);
                    }
                    _  => {
                        panic!("Unrecognized 0x8XY* instruction {:#X} {:#X}", self.pc, current_instruction);
                    }
                }
                self.pc += 2;
            }
            0x9 => {
                //  Skip next instruction if Vx != Vy. //
                //  The values of Vx and Vy are compared, and if they are not equal, the program counter is increased by 2. //
                let vx = self.read_reg_vx(x);
                let vy = self.read_reg_vx(y);

                if vx != vy {
                    self.pc += 4;
                } else {
                    self.pc += 2;
                }
            }
            0xA => {
                //  Set I = nnn. //
                //  The value of register I is set to nnn. //
                self.i = nnn;
                self.pc += 2;
            }
            0xB => {
                //  Jump to location nnn + V0. //
                //  The program counter is set to nnn plus the value of V0. //
                let location: u16 = self.read_reg_vx(0) as u16 + nnn;
                self.pc = location;
            }
            0xC => {
                //  Set Vx = random byte AND kk. //
                //  The interpreter generates a random number from 0 to 255, which is then ANDed with the value kk. The results are stored in Vx. See instruction 8xy2 for more information on AND. //
                let mut range = thread_rng();
                let rand_num: u8 = range.gen_range(0..=255);
                self.write_reg_vx(x, rand_num & nn);
                self.pc += 2;
            }
            0xD => {
                //  Display n-byte sprite starting at memory location I at (Vx, Vy), set VF = collision. //
                //  The interpreter reads n bytes from memory, starting at the address stored in I. //
                //  These bytes are then displayed as sprites on screen at coordinates (Vx, Vy). Sprites are XORed onto the existing screen. If this causes any pixels to be erased, VF is set to 1, otherwise it is set to 0.
                //  If the sprite is positioned so part of it is outside the coordinates of the display, it wraps around to the opposite side of the screen. See instruction 8xy3 for more information on XOR, and section 2.4, Display, for more information on the Chip-8 screen and sprites. //
                let vx = self.read_reg_vx(x);
                let vy = self.read_reg_vx(y);
                self.debug_draw_sprite(bus, vx, vy, n);
                self.pc += 2;
            }
            0xE => {
                match nn {
                    0x9E => {
                        // Skip next instruction if key with the value of Vx is pressed.  //
                        let key = self.read_reg_vx(x);
                        if bus.is_key_pressed(key) {
                            self.pc += 4;
                        } else {
                            self.pc += 2;
                        }
                    }
                    0xA1 => {
                        //  Skip next instruction if key with the value of Vx is not pressed. //
                        let key = self.read_reg_vx(x);
                        if !bus.is_key_pressed(key) {
                            self.pc += 4;
                        } else {
                            self.pc += 2;
                        }
                    }
                    _ => {
                        panic!("Unrecognized 0xEX** instruction {:#X}:{:#X}", self.pc, current_instruction);
                    }
                }
            }
            0xF => {
                match nn {
                    0x07 => {
                        //  Set Vx = delay timer value. //
                        //  The value of DT is placed into Vx. //
                        self.write_reg_vx(x, bus.get_delay_timer());
                        self.pc += 2;
                    }
                    0x0A => {
                        //  Wait for a key press, store the value of the key in Vx. //
                        //  All execution stops until a key is pressed, then the value of that key is stored in Vx. //
                        if let Some(val) = bus.get_key_pressed() {
                            self.write_reg_vx(x, val);
                            self.pc += 2;
                        }
                    }
                    0x15 => {
                        //  Set delay timer = Vx. //
                        let vs = self.read_reg_vx(x);
                        bus.set_delay_timer(vs);
                        self.pc += 2;
                    }
                    0x18 => {
                        //  Set sound timer = Vx. //
                        //  ST is set equal to the value of Vx. //
                        //  work on sound timer //

                        //  Sound: The Chip8 can only play the one sound which is a beep noise. 
                        //  The sound is played whenever the sound timer is greater than 0. So every time I decrement the sound timer I then check its value to see if it is greater than 0, if so I play a beep sound. 
                        //  This is a really very primitive way of emulating sound. Unfortunately this wont teach you much about real sound emulation which is widely regarded as being the most difficult part of emulation. 
                        self.pc += 2;
                    }
                    0x1E => {
                        //  Set I = I + Vx. //
                        //  The values of I and Vx are added, and the results are stored in I. //
                        let vx = self.read_reg_vx(x);
                        self.i = self.i + vx as u16;
                        self.pc += 2;
                    }
                    0x29 => {
                        //  Set I = location of sprite for digit Vx. //
                        //  The value of I is set to the location for the hexadecimal sprite corresponding to the value of Vx. //
                        //  Multiply by 5, each sprite has 5 lines //
                        self.i = self.read_reg_vx(x) as u16 * 5;
                        self.pc += 2;
                    }
                    0x33 => {
                        //  Store BCD representation of Vx in memory locations I, I+1, and I+2. //
                        //  The interpreter takes the decimal value of Vx, and places the hundreds digit in memory at location in I, the tens digit at location I+1, and the ones digit at location I+2. //
                        let vx = self.read_reg_vx(x);
                        bus.ram_write_byte(self.i, vx / 100);            // hundreds //
                        bus.ram_write_byte(self.i + 1, (vx % 100) / 10); // tens //
                        bus.ram_write_byte(self.i + 2, vx % 10);         // ones //
                        self.pc += 2;
                    }
                    0x55 => {
                        //  Store registers V0 through Vx in memory starting at location I. //
                        //  The interpreter copies the values of registers V0 through Vx into memory, starting at the address in I. //
                        for index in 0..x + 1 {
                            let val = self.read_reg_vx(index);
                            bus.ram_write_byte(self.i + index as u16, val);
                        }
                        self.i += x as u16 + 1;
                        self.pc += 2;
                    }
                    0x65 => {
                        //  Read registers V0 through Vx from memory starting at location I. //
                        //  The interpreter reads values from memory starting at location I into registers V0 through Vx. //
                        for index in 0..x + 1 {
                            let value = bus.ram_read_byte(self.i + index as u16);
                            self.write_reg_vx(index, value);

                        }
                        self.i += x as u16 + 1;
                        self.pc += 2;
                    }
                    _ => panic!("Unrecognized 0xF instruction {:#x}:{:#x}", self.pc, current_instruction)
                }
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

