#[derive(Debug, Clone, Copy)]
pub struct Key { x: u8 }

impl Key {
    pub fn new(x: u8) -> Self {
        assert!(x < 16);
        Key { x }
    }

    pub fn get(&self) -> u8 { self.x }
    pub fn idx(&self) -> usize { self.x as usize }
}

impl std::fmt::Display for Key {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "K{:X}", self.get())
    }
}

#[derive(Debug)]
pub struct Keypad {
    keys: [bool; 16],
}

impl Keypad {
    pub fn new() -> Self {
        Keypad { keys: [false; 16] }
    }

    pub fn set_pressed(&mut self, key: Key, pressed: bool) {
        self.keys[key.idx()] = pressed;
    }

    #[allow(unused_variables)]
    pub fn is_pressed(&self, key: Key) -> bool {
        self.keys[key.idx()]
    }

    pub fn get_key(&self) -> Option<Key> {
        for (i, &pressed) in self.keys.iter().enumerate() {
            if pressed {
                return Some(Key::new(i as u8));
            }
        }
        None
    }
}
