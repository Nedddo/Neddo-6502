use crate::memory::Addressable;
pub struct Bus {
    pub data: u8,
    pub address: u16,
}

// constructor
impl Bus {
    pub fn new() -> Self {
        Bus {
            data: 0,
            address: 0,
        }
    }
}

impl Bus {
    // reads
    pub fn read(&mut self, memory: &dyn Addressable) -> u8 {
        self.data = memory.read(self.address);
        self.data
    }
    pub fn read_at(&mut self, address: u16, memory: &dyn Addressable) -> u8 {
        self.address = address;
        self.read(memory)
    }
    // writes
    pub fn write(&mut self, memory: &mut dyn Addressable) {
        memory.write(self.address, self.data);
    }
    pub fn write_to(&mut self, address: u16, memory: &mut dyn Addressable) {
        self.address = address;
        self.write(memory);
    }
    pub fn write_value_to(&mut self, address: u16, val: u8, memory: &mut dyn Addressable) {
        self.address = address;
        self.data = val;
        self.write(memory);
    }
    pub fn write_value(&mut self, val: u8, memory: &mut dyn Addressable) {
        self.data = val;
        self.write(memory);
    }
}