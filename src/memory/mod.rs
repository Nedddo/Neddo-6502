use std::vec;

// trait to be implemented for system-specific memory configurations
pub trait Addressable {
    fn read (&self, address: u16) -> u8;
    fn write(&mut self, address: u16, data: u8);
}

// most basic memory configuration - 0 memory mapping all 64kb of address space are fully accessible
pub struct TestMemory {
    memory: [u8; 0x10000],
}

impl TestMemory {
    pub fn new() -> Self {
        Self {
            memory: [0u8; 0x10000]
        }
    }
    pub fn load(&mut self, data: Vec<u8>) {
        self.memory = data.try_into().expect("too little memory");
    }
}

impl Addressable for TestMemory {
    fn read(&self, address: u16) -> u8 {
        self.memory[address as usize]
    }
    fn write(&mut self, address: u16, data: u8) {
        self.memory[address as usize] = data;
    }
}

