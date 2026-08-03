#[derive(Default)]

struct Flags {
    // 8-bits encoding 7 flags, bit 5 is forced to 1
    n: bool,
    v: bool,
    b: bool,
    d: bool,
    i: bool,
    z: bool,
    c: bool,
}

impl Flags {
    value(self) -> u8 {
        
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