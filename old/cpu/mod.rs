// submodudles
pub(super) mod cpu_bus;
pub(super) mod instruction;
pub(super) mod cpu_ops;
// imports
use cpu_bus::Bus;
use crate::{cpu::instruction::{AddressingMode::{self, Accumulator}, Operation::JSR}, memory::Addressable};
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
    tmp_address: u16, // used for jsr, TODO: use this instead of address bus, address bus works everywhere else but If im doing this might as well use it 
    m_op_queue: VecDeque<for<'a> fn(&mut CPU, &'a mut (dyn Addressable + 'a))>,
}

// constructor
impl CPU {
    pub fn new() -> Self {
        Self {
            pc: 0,
            s: 0xFF,
            a: 0,
            x: 0,
            y: 0,
            p: Flags::default(),
            bus: Bus::new(),
            tmp_address: 0, 
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

            // fetch target address for this instruction (or do nothing for impl)
            self.queue_address_fetch(inst.mode);

            match inst.op {
                Operation::ADC => self.queue(adc),
                Operation::AND => self.queue(and),
                Operation::ASL => {
                    if inst.mode == Accumulator { 
                        // accumulator mode is a bit of an edge case
                        self.queue(asl_a);
                    } 
                    else {
                    self.queue(read_operand);
                    self.queue(asl);
                    self.queue(write_back);
                    }
                }
                Operation::BCC => self.queue(bcc),
                Operation::BCS => self.queue(bcs),
                Operation::BEQ => self.queue(beq),
                Operation::BIT => self.queue(bit),
                Operation::BMI => self.queue(bmi),
                Operation::BNE => self.queue(bne),
                Operation::BPL => self.queue(bpl),
                Operation::BRK => self.queue(brk),
                Operation::BVC => self.queue(bvc),
                Operation::BVS => self.queue(bvs),
                Operation::CLC => self.queue(clc),
                Operation::CLD => self.queue(cld),
                Operation::CLI => self.queue(cli),
                Operation::CLV => self.queue(clv),
                Operation::CMP => self.queue(cmp),
                Operation::CPX => self.queue(cpx),
                Operation::CPY => self.queue(cpy),
                Operation::DEC => {
                    self.queue(read_operand);
                    self.queue(dec);
                    self.queue(write_back);
                }
                Operation::DEX => self.queue(dex),
                Operation::DEY => self.queue(dey),
                Operation::EOR => self.queue(eor),
                Operation::INC => {
                    // read-modify-execute
                    self.queue(read_operand);
                    self.queue(inc);
                    self.queue(write_back);
                }
                Operation::INX => self.queue(inx),
                Operation::INY => self.queue(iny),
                Operation::JMP => {
                    self.queue(jmp);
                    self.cycle(mem); // <- this is foul
                    // hacky solution, jmp reads bytes straight to pc
                    // rather than some special addressing helpers for jump
                    // I just progress 1 cycle to keep it accurate.
                    // its really fine for the most part but i cant think of a less upsetting solution
                }
                Operation::JSR => self.queue(jsr),
                Operation::LDA => self.queue(lda),
                Operation::LDX => self.queue(ldx),
                Operation::LDY => self.queue(ldy),
                Operation::LSR => {
                    if inst.mode == Accumulator { 
                        // accumulator mode is a bit of an edge case
                        self.queue(lsr_a);
                    } 
                    else {
                    self.queue(read_operand);
                    self.queue(lsr);
                    self.queue(write_back);
                    }
                }
                Operation::NOP => self.queue(nop),
                Operation::ORA => self.queue(ora),
                Operation::PHA => {
                    // sorta did these wrongs, I assumed they were more simple
                    // turns out they do a lot of little bits and pieces across cycles
                    // these dummy accesses arent a terrible approximation but it could be improved
                    self.queue(dummy_access);
                    self.queue(pha);
                }
                Operation::PHP => {
                    self.queue(dummy_access);
                    self.queue(php);
                }
                Operation::PLA => {
                    self.queue(dummy_access);
                    self.queue(dummy_access);
                    self.queue(pla);
                }
                Operation::PLP => {
                    self.queue(dummy_access);
                    self.queue(dummy_access);
                    self.queue(plp);
                }
                Operation::ROL => {
                    if inst.mode == Accumulator { 
                        // accumulator mode is a bit of an edge case
                        self.queue(rol_a);
                    } 
                    else {
                    self.queue(read_operand);
                    self.queue(rol);
                    self.queue(write_back);
                    }
                }
                Operation::ROR => {
                    if inst.mode == Accumulator { 
                        // accumulator mode is a bit of an edge case
                        self.queue(ror_a);
                    } 
                    else {
                    self.queue(read_operand);
                    self.queue(ror);
                    self.queue(write_back);
                    }
                }
                Operation::RTI => self.queue(rti),
                Operation::RTS => self.queue(rts),
                Operation::SBC => self.queue(sbc),
                Operation::SEC => self.queue(sec),
                Operation::SED => self.queue(sed),
                Operation::SEI => self.queue(sei),
                Operation::STA => self.queue(sta),
                Operation::STX => self.queue(stx),
                Operation::STY => self.queue(sty),
                Operation::TAX => self.queue(tax), 
                Operation::TAY => self.queue(tay),
                Operation::TSX => self.queue(tsx), 
                Operation::TXA => self.queue(txa), 
                Operation::TXS => self.queue(txs), 
                Operation::TYA => self.queue(tya), 
                _ => panic!("Unimplemented or Invalid Instruction at PC={:x} : {:?} : {:x} \n sp: {:x}", self.pc,  inst, opcode, self.s)
            }
        }
    }
    pub fn reset(&mut self, mem: &mut dyn Addressable) {
        self.m_op_queue.clear();
        self.queue(dummy_access);
        self.queue(dummy_access);
        self.queue(push_pc_high);
        self.queue(push_pc_low);
        self.queue(php_hw);
        self.queue(pull_pc_low);
        self.queue(pull_pc_high);
    }
}

// private helpers
impl CPU {
    // legit just because the syntax was annoying to me
    fn queue(&mut self, m_op: fn(&mut CPU, &mut dyn Addressable) ) {
        self.m_op_queue.push_back(m_op);
    }
    // functions will get their operand predictably based on their addressing mode
    fn queue_address_fetch(&mut self, mode: AddressingMode) {
        use instruction::AddressingMode::*;
        match mode {
            Immediate | Relative => { self.bus.address = self.pc; self.pc += 1} // address is already at the pc! just move it to the bus
            Absolute => {
                self.queue(fetch_immediate);
                self.queue(fetch_absolute_high);
            }
            AbsoluteX {dynamic_cycles} => {
                self.queue(fetch_immediate);
                let high_fetch = if dynamic_cycles {fetch_absolute_x} else {fetch_absolute_x_fixed};
                self.queue(high_fetch);
            }
            AbsoluteY {dynamic_cycles} => {
                self.queue(fetch_immediate);
                let high_fetch = if dynamic_cycles {fetch_absolute_y} else {fetch_absolute_y_fixed};
                self.queue(high_fetch);
            }
            ZeroPage => { self.queue(fetch_zpg); }
            ZeroPageX => {
                self.queue(fetch_zpg); 
                self.queue(inc_zpg_x);
            }
            ZeroPageY => {
                self.queue(fetch_zpg);
                self.queue(inc_zpg_y);
            }
            IndirectX => {
                // fetch address at zero page + x
                self.queue(fetch_zpg);
                self.queue(inc_zpg_x);
                self.queue(fetch_indirect_low);
                self.queue(fetch_indirect_high);
            }
            IndirectY {dynamic_cycles} => {
                self.queue(fetch_zpg);
                self.queue(fetch_indirect_low);
                let high_fetch = if dynamic_cycles {fetch_indirect_high_y} else {fetch_indirect_high_y_fixed};
                self.queue(high_fetch);
            }
            Indirect => {
                self.queue(fetch_immediate); 
                self.queue(fetch_absolute_high);
                self.queue(fetch_indirect_low);
                self.queue(fetch_indirect_high)
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
        if self.d { val |= Self::F_D }
        if self.i { val |= Self::F_I }
        if self.z { val |= Self::F_Z }
        if self.c { val |= Self::F_C }
        val
    }

    fn set(&mut self, val: u8) {
        self.n = (val & Self::F_N) == Self::F_N;
        self.v = (val & Self::F_V) == Self::F_V;
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