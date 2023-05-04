#[derive(Debug, Clone, Copy)]
pub struct Register { x: u8 }

impl Register {
    pub fn new(x: u8) -> Self {
        assert!(x < 16);
        Register { x }
    }

    pub fn get(&self) -> u8 { self.x }
    pub fn idx(&self) -> usize { self.x as usize }
}
