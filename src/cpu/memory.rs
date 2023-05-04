use crate::cpu::Address;

pub struct Memory {
    data: [u8; 4096]
}

impl Memory {
    pub fn new() -> Self {
        Memory { data: [0; 4096] }
    }

    pub fn write(&mut self, addr: Address, data: &[u8]) {
        self.data[addr.get()..=addr.add(data.len() as u16 - 1).get()].copy_from_slice(data);
    }

    #[allow(dead_code)]
    pub fn write8(&mut self, addr: Address, value: u8) {
        self.data[addr.get()] = value;
    }

    #[allow(dead_code)]
    pub fn write16(&mut self, addr: Address, value: u16) {
        self.write8(addr, (value >> 8) as u8);
        self.write8(addr.add(1), value as u8);
    }

    pub fn read(&self, addr: Address, len: u16) -> &[u8] {
        &self.data[addr.get()..=addr.add(len-1).get()]
    }

    pub fn read8(&self, addr: Address) -> u8 {
        self.data[addr.get()]
    }

    pub fn read16(&self, addr: Address) -> u16 {
        ((self.read8(addr) as u16) << 8) |
        (self.read8(addr.add(1)) as u16)
    }
}
