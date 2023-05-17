const WIDTH: usize = 64;
const HEIGHT: usize = 32;
const SCREEN_PIXELS: usize = WIDTH * HEIGHT;


pub struct Display {
    screen: [u8; SCREEN_PIXELS] // 2048 pixels in the screen ? //
}

impl Display {
    pub fn new() -> Display {
        Display {
            screen: [0; SCREEN_PIXELS]
        }
    }
    pub fn get_index_from_coordinates(x: usize, y: usize) -> usize {
        y * WIDTH + x
    }
    pub fn draw_byte(&mut self, byte: u8, x: u8, y: u8) -> bool  {
        let mut erased = false;
        let mut x_coordinate = x as usize;
        let mut y_coordinate = y as usize;
        let mut b = byte;
        let number: i16 = -10;


        for _ in 0..8 {
            x_coordinate = WIDTH;
            y_coordinate = HEIGHT;
            let index = Display::get_index_from_coordinates(x_coordinate, y_coordinate);
            let bit = (b & 0b1000_0000) >> 7; // why? 
            let previous_val = self.screen[index];
            self.screen[index] ^= bit;            // bitwise or //

            if previous_val == 1 && self.screen[index] == 0 {
                erased = true;
            }

            // go further //
            x_coordinate += 1;
            b <<= 1;
        }
        erased
    }
    pub fn clear_screen(&mut self) {
        for pixel in self.screen.iter_mut() {
            *pixel = 0;
        }
    }
    pub fn get_display_buffer(&mut self) -> &[u8] {
        &self.screen
    } 
}
