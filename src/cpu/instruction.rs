#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Operation {
    ADC, AND, ASL, BCC, BCS, BEQ, BIT, BMI, BNE, BPL, BRK, BVC, BVS,
    CLC, CLD, CLI, CLV, CMP, CPX, CPY, DEC, DEX, DEY, EOR, INC, INX,
    INY, JMP, JSR, LDA, LDX, LDY, LSR, NOP, ORA, PHA, PHP, PLA, PLP,
    ROL, ROR, RTI, RTS, SBC, SEC, SED, SEI, STA, STX, STY, TAX, TAY,
    TSX, TXA, TXS, TYA,
    /// Placeholder for undocumented/illegal NMOS 6502 opcodes.
    XXX,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AddressingMode {
    Implied,
    Accumulator,
    Immediate,
    ZeroPage,
    ZeroPageX,
    ZeroPageY,
    Relative,
    Absolute,
    AbsoluteX,
    AbsoluteY,
    Indirect,
    IndirectX, // (indirect,X)
    IndirectY, // (indirect),Y
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Instruction {
    pub op: Operation,
    pub mode: AddressingMode,
    pub len: u8, // byte length from 1 - 3, depending on instruction
}

impl Instruction {
    pub fn decode(opcode: u8) -> Instruction {
        use AddressingMode::*;
        use Operation::*;

        let (op, mode, len) = match opcode {
            0x00 => (BRK, Implied, 1),
            0x01 => (ORA, IndirectX, 2),
            0x05 => (ORA, ZeroPage, 2),
            0x06 => (ASL, ZeroPage, 2),
            0x08 => (PHP, Implied, 1),
            0x09 => (ORA, Immediate, 2),
            0x0A => (ASL, Accumulator, 1),
            0x0D => (ORA, Absolute, 3),
            0x0E => (ASL, Absolute, 3),

            0x10 => (BPL, Relative, 2),
            0x11 => (ORA, IndirectY, 2),
            0x15 => (ORA, ZeroPageX, 2),
            0x16 => (ASL, ZeroPageX, 2),
            0x18 => (CLC, Implied, 1),
            0x19 => (ORA, AbsoluteY, 3),
            0x1D => (ORA, AbsoluteX, 3),
            0x1E => (ASL, AbsoluteX, 3),

            0x20 => (JSR, Absolute, 3),
            0x21 => (AND, IndirectX, 2),
            0x24 => (BIT, ZeroPage, 2),
            0x25 => (AND, ZeroPage, 2),
            0x26 => (ROL, ZeroPage, 2),
            0x28 => (PLP, Implied, 1),
            0x29 => (AND, Immediate, 2),
            0x2A => (ROL, Accumulator, 1),
            0x2C => (BIT, Absolute, 3),
            0x2D => (AND, Absolute, 3),
            0x2E => (ROL, Absolute, 3),

            0x30 => (BMI, Relative, 2),
            0x31 => (AND, IndirectY, 2),
            0x35 => (AND, ZeroPageX, 2),
            0x36 => (ROL, ZeroPageX, 2),
            0x38 => (SEC, Implied, 1),
            0x39 => (AND, AbsoluteY, 3),
            0x3D => (AND, AbsoluteX, 3),
            0x3E => (ROL, AbsoluteX, 3),

            0x40 => (RTI, Implied, 1),
            0x41 => (EOR, IndirectX, 2),
            0x45 => (EOR, ZeroPage, 2),
            0x46 => (LSR, ZeroPage, 2),
            0x48 => (PHA, Implied, 1),
            0x49 => (EOR, Immediate, 2),
            0x4A => (LSR, Accumulator, 1),
            0x4C => (JMP, Absolute, 3),
            0x4D => (EOR, Absolute, 3),
            0x4E => (LSR, Absolute, 3),

            0x50 => (BVC, Relative, 2),
            0x51 => (EOR, IndirectY, 2),
            0x55 => (EOR, ZeroPageX, 2),
            0x56 => (LSR, ZeroPageX, 2),
            0x58 => (CLI, Implied, 1),
            0x59 => (EOR, AbsoluteY, 3),
            0x5D => (EOR, AbsoluteX, 3),
            0x5E => (LSR, AbsoluteX, 3),

            0x60 => (RTS, Implied, 1),
            0x61 => (ADC, IndirectX, 2),
            0x65 => (ADC, ZeroPage, 2),
            0x66 => (ROR, ZeroPage, 2),
            0x68 => (PLA, Implied, 1),
            0x69 => (ADC, Immediate, 2),
            0x6A => (ROR, Accumulator, 1),
            0x6C => (JMP, Indirect, 3),
            0x6D => (ADC, Absolute, 3),
            0x6E => (ROR, Absolute, 3),

            0x70 => (BVS, Relative, 2),
            0x71 => (ADC, IndirectY, 2),
            0x75 => (ADC, ZeroPageX, 2),
            0x76 => (ROR, ZeroPageX, 2),
            0x78 => (SEI, Implied, 1),
            0x79 => (ADC, AbsoluteY, 3),
            0x7D => (ADC, AbsoluteX, 3),
            0x7E => (ROR, AbsoluteX, 3),

            0x81 => (STA, IndirectX, 2),
            0x84 => (STY, ZeroPage, 2),
            0x85 => (STA, ZeroPage, 2),
            0x86 => (STX, ZeroPage, 2),
            0x88 => (DEY, Implied, 1),
            0x8A => (TXA, Implied, 1),
            0x8C => (STY, Absolute, 3),
            0x8D => (STA, Absolute, 3),
            0x8E => (STX, Absolute, 3),

            0x90 => (BCC, Relative, 2),
            0x91 => (STA, IndirectY, 2),
            0x94 => (STY, ZeroPageX, 2),
            0x95 => (STA, ZeroPageX, 2),
            0x96 => (STX, ZeroPageY, 2),
            0x98 => (TYA, Implied, 1),
            0x99 => (STA, AbsoluteY, 3),
            0x9A => (TXS, Implied, 1),
            0x9D => (STA, AbsoluteX, 3),

            0xA0 => (LDY, Immediate, 2),
            0xA1 => (LDA, IndirectX, 2),
            0xA2 => (LDX, Immediate, 2),
            0xA4 => (LDY, ZeroPage, 2),
            0xA5 => (LDA, ZeroPage, 2),
            0xA6 => (LDX, ZeroPage, 2),
            0xA8 => (TAY, Implied, 1),
            0xA9 => (LDA, Immediate, 2),
            0xAA => (TAX, Implied, 1),
            0xAC => (LDY, Absolute, 3),
            0xAD => (LDA, Absolute, 3),
            0xAE => (LDX, Absolute, 3),

            0xB0 => (BCS, Relative, 2),
            0xB1 => (LDA, IndirectY, 2),
            0xB4 => (LDY, ZeroPageX, 2),
            0xB5 => (LDA, ZeroPageX, 2),
            0xB6 => (LDX, ZeroPageY, 2),
            0xB8 => (CLV, Implied, 1),
            0xB9 => (LDA, AbsoluteY, 3),
            0xBA => (TSX, Implied, 1),
            0xBC => (LDY, AbsoluteX, 3),
            0xBD => (LDA, AbsoluteX, 3),
            0xBE => (LDX, AbsoluteY, 3),

            0xC0 => (CPY, Immediate, 2),
            0xC1 => (CMP, IndirectX, 2),
            0xC4 => (CPY, ZeroPage, 2),
            0xC5 => (CMP, ZeroPage, 2),
            0xC6 => (DEC, ZeroPage, 2),
            0xC8 => (INY, Implied, 1),
            0xC9 => (CMP, Immediate, 2),
            0xCA => (DEX, Implied, 1),
            0xCC => (CPY, Absolute, 3),
            0xCD => (CMP, Absolute, 3),
            0xCE => (DEC, Absolute, 3),

            0xD0 => (BNE, Relative, 2),
            0xD1 => (CMP, IndirectY, 2),
            0xD5 => (CMP, ZeroPageX, 2),
            0xD6 => (DEC, ZeroPageX, 2),
            0xD8 => (CLD, Implied, 1),
            0xD9 => (CMP, AbsoluteY, 3),
            0xDD => (CMP, AbsoluteX, 3),
            0xDE => (DEC, AbsoluteX, 3),

            0xE0 => (CPX, Immediate, 2),
            0xE1 => (SBC, IndirectX, 2),
            0xE4 => (CPX, ZeroPage, 2),
            0xE5 => (SBC, ZeroPage, 2),
            0xE6 => (INC, ZeroPage, 2),
            0xE8 => (INX, Implied, 1),
            0xE9 => (SBC, Immediate, 2),
            0xEA => (NOP, Implied, 1),
            0xEC => (CPX, Absolute, 3),
            0xED => (SBC, Absolute, 3),
            0xEE => (INC, Absolute, 3),

            0xF0 => (BEQ, Relative, 2),
            0xF1 => (SBC, IndirectY, 2),
            0xF5 => (SBC, ZeroPageX, 2),
            0xF6 => (INC, ZeroPageX, 2),
            0xF8 => (SED, Implied, 1),
            0xF9 => (SBC, AbsoluteY, 3),
            0xFD => (SBC, AbsoluteX, 3),
            0xFE => (INC, AbsoluteX, 3),

            // Everything else is an undocumented/illegal NMOS 6502 opcode.
            _ => (XXX, Implied, 1),
        };

        Instruction { op, mode, len }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn decodes_known_opcodes() {
        assert_eq!(
            Instruction::decode(0xA9),
            Instruction { op: Operation::LDA, mode: AddressingMode::Immediate, len: 2 }
        );
        assert_eq!(
            Instruction::decode(0x00),
            Instruction { op: Operation::BRK, mode: AddressingMode::Implied, len: 1 }
        );
        assert_eq!(
            Instruction::decode(0x6C),
            Instruction { op: Operation::JMP, mode: AddressingMode::Indirect, len: 3 }
        );
    }

    #[test]
    fn illegal_opcodes_fall_through() {
        assert_eq!(
            Instruction::decode(0x02),
            Instruction { op: Operation::XXX, mode: AddressingMode::Implied, len: 1 }
        );
    }
}