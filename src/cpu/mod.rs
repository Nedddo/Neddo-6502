const F_N: u8     = 0b1000_0000;
const F_V: u8     = 0b0100_0000;
const F_FIXED: u8 = 0b0010_0000;
const F_B: u8     = 0b0001_0000;
const F_D: u8     = 0b0000_1000;
const F_I: u8     = 0b0000_0100;
const F_Z: u8     = 0b0000_0010;
const F_C: u8     = 0b0000_0001;

use cpu_bus::Bus;

use crate::memory::Addressable;
use std::collections::VecDeque;

#[derive(Default, Debug)]
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
    // used in loads and transfers
    fn update_nz(&mut self, val: u8) {
        self.z = val == 0;
        self.n = (val & F_N) == F_N // negative = true if 7th bit is signed
    }
}

pub struct CPU {
    pc: u16,
    s: u8,
    a: u8,
    x: u8,
    y: u8,
    p: Flags,
    bus: Bus, // cpu bus, represents hardwired connection between cpu and memory
    m_op_queue: VecDeque<fn(&mut CPU, &mut dyn Addressable)>,
}

// constructor
impl CPU {
    pub fn new() -> Self {
        Self {
            pc: 0,
            s: 0,
            a: 0,
            x: 0,
            y: 0,
            p: Flags::default(),
            bus: Bus::new(),
            m_op_queue: VecDeque::new(),
        }
    }
}

// execution
impl CPU {
    pub fn cycle(&mut self, mem: &mut dyn Addressable) {
        use instruction::*;
        // current instruction has not finsihed execution
        if let Some(this_cycle) =  self.m_op_queue.pop_front() {
            this_cycle(self, mem);
        }
        else {
            // fetch and decode next instruction
            self.bus.address = self.pc;
            self.pc += 1;
            let inst = Instruction::decode(self.fetch(mem));

            match inst.op {
                Operation::TAX => {
                    self.m_op_queue.push_back(CPU::tax);
                },
                _ => panic!("Unimplemented or Invalid Instruction: {:?}", inst)
            }
        }
    }
    // TODO: Implement reset
}

// bus reads
// TODO: Redo these, just byte the bullet and make them more micro-op specific
impl CPU {
    // assumes PC is moved to address bus explicitly at the start of instruction execution
    fn fetch(&mut self, mem: &mut dyn Addressable) -> u8 {
        self.bus.read(mem)
    }
    // read memory at address bus, increment address bus
    fn fetch_low(&mut self, mem: &mut dyn Addressable) -> u8 {
        let data = self.bus.read(mem);
        self.bus.address += 1;
        data
    }
    // fetch high byte from memory, assumes data bus contains low byte as last read. Sets Address bus.
    fn fetch_high(&mut self, mem: &mut dyn Addressable) -> u8 {
        let lo = self.bus.data;
        let hi = self.bus.read(mem);
        let address = ((hi as u16) << 8) | (lo as u16);
        self.bus.address = address;
        hi // hello!
    }
}

// micro-ops (some of these are full 2-cycle instructions with the first cycle being the fetch-decode)
// TODO(?): Move these to a seperate module, still deciding
impl CPU {
    // doesn't use memory, still needs it because all m-ops must have the same signature
    fn tax(&mut self, _: &mut dyn Addressable) {
        self.x = self.a;
        self.p.update_nz(self.x);
    }
}

pub(super) mod cpu_bus;
pub(super) mod instruction;
pub(super) mod ops;
mod tests;