pub struct Bus<'a> {
    pub memory: &'a mut [u8],
    pub data: u8,
    pub address: u16,
}

// constructor
impl<'a> Bus<'a> {
    pub fn new(memory: &'a mut [u8]) -> Self {
        Bus {
            memory,
            data: 0,
            address: 0,
        }
    }
}

impl Bus<'_> {
    // reads
    pub fn read(&mut self) -> u8 {
        self.data = self.memory[self.address as usize];
        self.data
    }
    pub fn read_at(&mut self, address: u16) -> u8 {
        self.address = address;
        self.read()
    }
    // writes
    pub fn write(&mut self) {
        self.memory[self.address as usize] = self.data;
    }
    pub fn write_to(&mut self, address: u16) {
        self.address = address;
        self.write();
    }
    pub fn write_value_to(&mut self, address: u16, val: u8) {
        self.address = address;
        self.data = val;
        self.write();
    }
}