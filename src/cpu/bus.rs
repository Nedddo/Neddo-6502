pub struct Bus {
    &mut memory[u8],
    pub data: u8,
    pub address: u16,
}

impl Bus {
    // reads
    pub fn read(self) -> u8 {
        self.memory[self.address]
    }
    pub fn readAt(self, address: u16) -> u8 {
        self.address = address;
        self.read()
    }
    // writes
    pub fn write(self) {
        self.memory[self.address] = self.data;
    }
    pub fn writeTo(self, address: u16) {
        self.address = address;
        self.write();
    }
    pub fn writeValueTo(self, address: u16, val: u8) {
        self.address = address;
        self.data = val;
        self.write();
    }
}