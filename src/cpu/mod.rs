const F_N: u8     = 0b1000_0000;
const F_V: u8     = 0b0100_0000;
const F_FIXED: u8 = 0b0010_0000;
const F_B: u8     = 0b0001_0000;
const F_D: u8     = 0b0000_1000;
const F_I: u8     = 0b0000_0100;
const F_Z: u8     = 0b0000_0010;
const F_C: u8     = 0b0000_0001;

#[derive(Default)]
struct Flags {
    // 8-bits encoding 7 flags, bit 5 is forced to 1
    n: bool,
    v: bool,
    b: bool, // not a real flag as far as the cpu is concerned
    d: bool,
    i: bool,
    z: bool,
    c: bool,
}

impl Flags {
    fn value(&self) -> u8 {
        let mut val: u8 = F_FIXED; // bit 5 is always 1
        if self.n { val |= F_N }
        if self.v { val |= F_V }
        if self.b { val |= F_B }
        if self.d { val |= F_D }
        if self.i { val |= F_I }
        if self.z { val |= F_Z }
        if self.c { val |= F_C }
        val
    }

    fn set(&mut self, val: u8) {
        self.n = (val & F_N) == F_N;
        self.v = (val & F_V) == F_V;
        self.b = (val & F_B) == F_B;
        self.d = (val & F_D) == F_D;
        self.i = (val & F_I) == F_I;
        self.z = (val & F_Z) == F_Z;
        self.c = (val & F_C) == F_C;
    }
}

pub struct CPU<'a> {
    pc: u16,
    s: u8,
    a: u8,
    x: u8,
    y: u8,
    p: Flags,
    bus: Bus<'a>, // cpu bus, represents hardwired connection between cpu and memory
}

// constructor
impl<'a> CPU<'a> {
    pub fn new(memory: &'a mut [u8]) {
        Self {
            pc: 0,
            s: 0,
            a: 0,
            x: 0,
            y: 0,
            p: Flags::default(),
            bus: Bus::new(memory),
        }
    }
}

// bus reads
impl CPU<'_> {
    // assumes PC is moved to address bus explicitly at the start of instruction execution
    fn fetch(&mut self) {
        Bus.read()
    }
    // read memory at address bus, increment address bus
    fn fetchLow(&mut self) {
        let data = Bus.read();
        Bus.address += 1;
        data
    }
    // fetch high byte from memory, assumes data bus contains low byte as last read. Sets Address bus.
    fn fetchHigh(&mut self) {
        let lo = Bus.data as u16;
        let hi = Bus.read() as u16;
        let address = (hi << 8) | lo;
        Bus.address = address;
        hi // hello!
    }
    // must be called after address is loaded from memory - however, handles incrementation
    fn fetchOperand(&mut self) {

    }
}


pub(super) mod bus;
pub(super) mod instruction;