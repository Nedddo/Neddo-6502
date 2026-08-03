const F_N = 0b1000`0000;
const F_V = 0b0100`0000;
const F_FIXED = 0b0010`0000;
const F_B = 0b0001`0000;
const F_D = 0b0000`1000;
const F_I = 0b0000`0100;
const F_Z = 0b0000`0010;
const F_C = 0b0000`0001;

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
    fn value(self) -> u8 {
        val: u8 = F_FIXED; // bit 5 is always 1
        if n { val |= F_N }
        if v { val |= F_V }
        if b { val |= F_B }
        if d { val |= F_D }
        if i { val |= F_I }
        if z { val |= F_Z }
        if c { val |= F_C }
        val
    }

    fn set(self, val: u8) {
        if val & F_N == F_N { self.n = true }
        if val & F_V == F_V { self.v = true }
        if val & F_B == F_B { self.b = true }
        if val & F_D == F_D { self.d = true }
        if val & F_I == F_I { self.i = true }
        if val & F_Z == F_Z { self.z = true }
        if val & F_C == F_C { self.c = true }
    }
}

#[derive(Default)]
pub struct CPU {
    pc: u16,
    s: u8,
    a: u8,
    x: u8,
    y: u8,
    p: Flags
}

mod bus;