#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Address { x: u16 }

impl Address {
    pub fn new(x: u16) -> Self {
        assert!(x < 4096);
        Address { x }
    }

    pub fn add(&self, y: u16) -> Self {
        Address::new(self.x + y)
    }

    pub fn sub(&self, y: u16) -> Self {
        Address::new(self.x - y)
    }

    pub fn get(&self) -> usize { self.x as usize }
}

impl From<u16> for Address {
    fn from(x: u16) -> Self {
        Address::new(x)
    }
}
