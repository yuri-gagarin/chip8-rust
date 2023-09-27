use minifb::Key;

pub struct Keyboard {
    key_pressed: Option<u8>,
}

impl Keyboard {
    pub fn new() -> Keyboard {
        Keyboard { 
          key_pressed: None
        }
    }
    pub fn is_key_pressed(&self, key_code: u8) -> bool {
        if let Some(key) = self.key_pressed {
            key == key_code
        } else {
            false
        }
    }
    pub fn set_key_pressed(&mut self, key: Option<u8>) {
        self.key_pressed = key;
    }
    pub fn get_key_pressed(&self) -> Option<u8> {
        self.key_pressed
    }
    pub fn get_keycode(key: Option<Key>) -> Option<u8> {
        match key {
            Some(Key::Key1) => Some(0x1),
            Some(Key::Key2) => Some(0x2),
            Some(Key::Key3) => Some(0x3),
            Some(Key::C)    => Some(0xC),
    
            Some(Key::Key4) => Some(0x4),
            Some(Key::Key5) => Some(0x5),
            Some(Key::Key6) => Some(0x6),
            Some(Key::D)    => Some(0xD),
    
            Some(Key::Key7) => Some(0x7),
            Some(Key::Key8) => Some(0x8),
            Some(Key::Key9) => Some(0x9),
            Some(Key::E)    => Some(0xE),
    
            Some(Key::A)    => Some(0xA),
            Some(Key::Key0) => Some(0x0),
            Some(Key::B)    => Some(0xB),
            Some(Key::F)    => Some(0xF),
            _ => None,
        }
    }
}

