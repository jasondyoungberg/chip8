pub struct Key { x: u8 }

impl Key {
    pub fn new(x: u8) -> Self {
        assert!(x < 16);
        Key { x }
    }

    pub fn get(&self) -> u8 { self.x }
}

impl std::fmt::Display for Key {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "K{:X}", self.get())
    }
}

pub struct Keypad();

impl Keypad {
    pub fn new() -> Self { Keypad() }

    #[allow(unused_variables)]
    pub fn is_pressed(&self, key: Key) -> bool {
        todo!()
    }

    pub fn get_key(&self) -> Option<Key> {
        todo!()
    }
}
