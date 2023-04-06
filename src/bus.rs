use super::display::Display;
use super::keyboard::Keyboard;
use super::ram::Ram;

pub struct Bus {
    display: Display,
    keyboard: Keyboard,
    ram: Ram,
    delay_timer: u8,
    delay_timer_set_time: std::time::Instant,
}

impl Bus {
    pub fn new() -> Bus {
        Bus {
            display: Display::new(),
            keyboard: Keyboard::new(),
            ram: Ram::new(),
            delay_timer: 0,
            delay_timer_set_time: std::time::Instant::now(),
        }
    }
    pub fn ram_read_byte(&mut self, address: u16) -> u8 {
        self.ram.read_byte(address)
    }
    pub fn ram_write_byte(&mut self, address: u16, value: u8) {
        self.ram.write_byte(address, value);
    }
    pub fn display_draw_byte(&mut self, byte: u8, x: u8, y: u8) {
        self.display.draw_byte(byte, x, y);
    }
    pub fn display_clear_screen(&mut self) {
        self.display.clear_screen();
    }
    pub fn set_key_pressed(&mut self, key: Option<u8>) {
        self.keyboard.set_key_pressed(key);
    }
    pub fn get_key_pressed(&mut self) -> Option<u8> {
        self.keyboard.get_key_pressed()
    }
    pub fn is_key_pressed(&self, key_code: u8) -> bool {
        self.keyboard.is_key_pressed(key_code)
    }
    pub fn set_daly_timer(&mut self, value: u8) {
        self.delay_timer_set_time = std::time::Instant::now();
        self.delay_timer = value;
    }
    pub fn get_delay_timer(&self) -> u8 {
        let diff = std::time::Instant::now();
    }
}