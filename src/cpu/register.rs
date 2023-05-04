pub struct Register { x: u8 }

impl Register {
    pub fn new(x: u8) -> Self {
        assert!(x < 16);
        Register { x }
    }

    pub fn get(&self) -> u8 { self.x }
    pub fn idx(&self) -> usize { self.x as usize }
}

impl std::fmt::Display for Register {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "V{:X}", self.get())
    }
}
