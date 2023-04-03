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
    pub fn draw_byte(&mut self, byte: u8, x: u8, y: u8) {
        
    }
}
