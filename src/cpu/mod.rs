// submodudles
pub(super) mod cpu_bus;
pub(super) mod instruction;
pub(super) mod cpu_ops;
// imports
use cpu_bus::Bus;
use crate::{cpu::instruction::AddressingMode, memory::Addressable};
use std::collections::VecDeque;
use cpu_ops::*;

pub struct CPU {
    pc: u16,
    s: u8,
    a: u8,
    x: u8,
    y: u8,
    p: Flags,
    bus: Bus, // cpu bus, represents hardwired connection between cpu and memory
    m_op_queue: VecDeque<for<'a> fn(&mut CPU, &'a mut (dyn Addressable + 'a))>,
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
            fetch_immediate(self, mem);
            let opcode = self.bus.data;
            let inst = Instruction::decode(opcode);

            match inst.op {
                Operation::TAX => {
                    self.m_op_queue.push_back(tax);
                },
                Operation::ADC => {
                    self.queue_address_fetch(inst.mode);
                    self.m_op_queue.push_back(adc);
                }
                Operation::LDA => {
                    self.queue_address_fetch(inst.mode);
                    self.m_op_queue.push_back(lda);
                }
                Operation::STA => {
                    self.queue_address_fetch(inst.mode);
                    self.m_op_queue.push_back(sta);
                }
                _ => panic!("Unimplemented or Invalid Instruction: {:?}", inst)
            }
        }
    }
    // TODO: Implement reset
}

// private helpers
impl CPU {
    // functions will get their operand predictably based on their addressing mode
    fn queue_address_fetch(&mut self, mode: AddressingMode) {
        use instruction::AddressingMode::*;
        match mode {
            Immediate | Relative => { self.bus.address = self.pc; self.pc += 1} // address is already at the pc! just move it to the bus
            Absolute => {
                self.m_op_queue.push_back(fetch_immediate);
                self.m_op_queue.push_back(fetch_absolute_high);
            }
            AbsoluteX {dynamic_cycles} => {
                self.m_op_queue.push_back(fetch_immediate);
                let high_fetch = if dynamic_cycles {fetch_absolute_x} else {fetch_absolute_x_fixed};
                self.m_op_queue.push_back(high_fetch);
            }
            AbsoluteY {dynamic_cycles} => {
                self.m_op_queue.push_back(fetch_immediate);
                let high_fetch = if dynamic_cycles {fetch_absolute_y} else {fetch_absolute_y_fixed};
                self.m_op_queue.push_back(high_fetch);
            }
            ZeroPage => { self.m_op_queue.push_back(fetch_zpg); }
            ZeroPageX => {
                self.m_op_queue.push_back(fetch_zpg); 
                self.m_op_queue.push_back(inc_zpg_x);
            }
            ZeroPageY => {
                self.m_op_queue.push_back(fetch_zpg);
                self.m_op_queue.push_back(inc_zpg_y);
            }
            IndirectX => {
                // fetch address at zero page + x
                self.m_op_queue.push_back(fetch_zpg);
                self.m_op_queue.push_back(inc_zpg_x);
                self.m_op_queue.push_back(fetch_indirect_low);
                self.m_op_queue.push_back(fetch_indirect_high);
            }
            IndirectY {dynamic_cycles} => {
                self.m_op_queue.push_back(fetch_zpg);
                self.m_op_queue.push_back(fetch_indirect_low);
                let high_fetch = if dynamic_cycles {fetch_indirect_high_y} else {fetch_indirect_high_y_fixed};
                self.m_op_queue.push_back(high_fetch);
            }
            Indirect => {
                self.m_op_queue.push_back(fetch_immediate); 
                self.m_op_queue.push_back(fetch_absolute_high);
                self.m_op_queue.push_back(fetch_indirect_low);
                // high fetch is done DURING jmp instruction, which is the only instruction which uses this mode
            }
            _ => {/* do nothing for other addressing modes */}
        }
    }
}

mod tests; // cpu tests, declared here because it lookes nicer to me okaY!

// allows me to represent p register as a set of booleans, which reflects its function in like 90% of cases.
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
    // bitmasks!!!
    const F_N: u8     = 0b1000_0000;
    const F_V: u8     = 0b0100_0000;
    const F_FIXED: u8 = 0b0010_0000;
    const F_B: u8     = 0b0001_0000;
    const F_D: u8     = 0b0000_1000;
    const F_I: u8     = 0b0000_0100;
    const F_Z: u8     = 0b0000_0010;
    const F_C: u8     = 0b0000_0001;

    fn value(&self) -> u8 {
        let mut val: u8 = Self::F_FIXED; // bit 5 is always 1
        if self.n { val |= Self::F_N }
        if self.v { val |= Self::F_V }
        if self.b { val |= Self::F_B }
        if self.d { val |= Self::F_D }
        if self.i { val |= Self::F_I }
        if self.z { val |= Self::F_Z }
        if self.c { val |= Self::F_C }
        val
    }

    fn set(&mut self, val: u8) {
        self.n = (val & Self::F_N) == Self::F_N;
        self.v = (val & Self::F_V) == Self::F_V;
        self.b = (val & Self::F_B) == Self::F_B;
        self.d = (val & Self::F_D) == Self::F_D;
        self.i = (val & Self::F_I) == Self::F_I;
        self.z = (val & Self::F_Z) == Self::F_Z;
        self.c = (val & Self::F_C) == Self::F_C;
    }
    // used in loads and transfers
    fn update_nz(&mut self, val: u8) {
        self.z = val == 0;
        self.n = (val & Self::F_N) == Self::F_N // negative = true if 7th bit is signed
    }
}