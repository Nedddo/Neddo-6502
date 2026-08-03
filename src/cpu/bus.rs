pub struct Bus<'a> {
    pub memory: &'a mut [u8],
    pub data: u8,
    pub address: u16,
}

impl Bus<'_> {
    // reads
    pub fn read(&mut self) -> u8 {
        self.data = self.memory[self.address as usize];
        self.data
    }
    pub fn readAt(&mut self, address: u16) -> u8 {
        self.address = address;
        self.read()
    }
    // writes
    pub fn write(&mut self) {
        self.memory[self.address as usize] = self.data;
    }
    pub fn writeTo(&mut self, address: u16) {
        self.address = address;
        self.write();
    }
    pub fn writeValueTo(&mut self, address: u16, val: u8) {
        self.address = address;
        self.data = val;
        self.write();
    }
}