use super::*;

type SysMem<'a> = &'a mut dyn Addressable;

/* ----- INSTRUCTION MICRO OPS ----- */

pub fn nop(_: &mut CPU, _: SysMem) {
    // you will do nothing and be happy
}
// transfers
pub fn tax(cpu: &mut CPU, _: SysMem) {
    cpu.x = cpu.a;
    cpu.p.update_nz(cpu.x);
}
pub fn txa(cpu: &mut CPU, _: SysMem) {
    cpu.a = cpu.x;
    cpu.p.update_nz(cpu.a);
}
pub fn tay(cpu: &mut CPU, _: SysMem) {
    cpu.y = cpu.a;
    cpu.p.update_nz(cpu.y);
}
pub fn tya(cpu: &mut CPU, _: SysMem) {
    cpu.a = cpu.y;
    cpu.p.update_nz(cpu.a);
}
// stack transfers do not update any flags
pub fn txs(cpu: &mut CPU, _: SysMem) {
    cpu.s = cpu.x;
}
pub fn tsx(cpu: &mut CPU, _: SysMem) {
    cpu.x = cpu.s;
}
// register increments
pub fn inx(cpu: &mut CPU, _: SysMem) {
    cpu.x += 1;
    cpu.p.update_nz(cpu.x);
}
pub fn iny(cpu: &mut CPU, _: SysMem) {
    cpu.y += 1;
    cpu.p.update_nz(cpu.y);
}
// register decrements
pub fn dex(cpu: &mut CPU, _: SysMem) {
    cpu.x -= 1;
    cpu.p.update_nz(cpu.x);
}
pub fn dey(cpu: &mut CPU, _: SysMem) {
    cpu.y -= 1;
    cpu.p.update_nz(cpu.y);
}
// flag clearing
pub fn clc(cpu: &mut CPU, _: SysMem) {
    cpu.p.c = false;
}
pub fn cld(cpu: &mut CPU, _: SysMem) {
    cpu.p.d = false;
}
pub fn cli(cpu: &mut CPU, _: SysMem) {
    cpu.p.i = false;
}
pub fn clv(cpu: &mut CPU, _: SysMem) {
    cpu.p.v = false;
}
// flag setting
pub fn sec(cpu: &mut CPU, _: SysMem) {
    cpu.p.c = true;
}
pub fn sed(cpu: &mut CPU, _: SysMem) {
    cpu.p.d = true;
}
pub fn sei(cpu: &mut CPU, _: SysMem) {
    cpu.p.i = true;
}
// arithmetic / logic
pub fn adc(cpu: &mut CPU, mem: SysMem) {
    // read operand
    let operand_a = cpu.a;
    let operand_b = cpu.bus.read(mem);
    let (result, carry) = operand_a.carrying_add(operand_b, cpu.p.c);
    cpu.a = result;
    // update flags
    cpu.p.update_nz(cpu.a);
    cpu.p.c = carry;
    cpu.p.v = (((result ^ operand_a) & (result ^ operand_b)) as i8) < 0; // will be negative if and only if sign change ocurs between both operands
}
pub fn and(cpu: &mut CPU, mem: SysMem) {
    let operand = cpu.bus.read(mem);
    cpu.a &= operand;
    cpu.p.update_nz(cpu.a);
}
pub fn ora(cpu: &mut CPU, mem: SysMem) {
    let operand = cpu.bus.read(mem);
    cpu.a |= operand;
    cpu.p.update_nz(cpu.a);
}
pub fn eor(cpu: &mut CPU, mem: SysMem) {
    let operand = cpu.bus.read(mem);
    cpu.a ^= operand;
    cpu.p.update_nz(cpu.a);
}
// memory!!
pub fn lda(cpu: &mut CPU, mem: SysMem) {
    let data = cpu.bus.read(mem);
    cpu.a = data;
    cpu.p.update_nz(cpu.a);
}
pub fn ldx(cpu: &mut CPU, mem: SysMem) {
    let data = cpu.bus.read(mem);
    cpu.x = data;
    cpu.p.update_nz(cpu.x);
}
pub fn ldy(cpu: &mut CPU, mem: SysMem) {
    let data = cpu.bus.read(mem);
    cpu.y = data;
    cpu.p.update_nz(cpu.y);
}
pub fn sta (cpu: &mut CPU, mem: SysMem) {
    cpu.bus.write_value(cpu.a, mem);
}
pub fn stx (cpu: &mut CPU, mem: SysMem) {
    cpu.bus.write_value(cpu.x, mem);
}
pub fn sty (cpu: &mut CPU, mem: SysMem) {
    cpu.bus.write_value(cpu.y, mem);
}
// read modify execute
pub fn inc(cpu: &mut CPU, _: SysMem) {
    cpu.bus.data += 1;
    cpu.p.update_nz(cpu.bus.data);
}
pub fn asl(cpu: &mut CPU, _: SysMem) {
    // check if bit 7 is shifted out
    if (cpu.bus.data & 0x80 )!= 0 { cpu.p.c = true }
    cpu.bus.data <<= 1;
    cpu.p.update_nz(cpu.bus.data);
}
pub fn asl_a(cpu: &mut CPU, _: SysMem) {
    if (cpu.a & 0x80 )!= 0 { cpu.p.c = true }
    cpu.a <<= 1;
    cpu.p.update_nz(cpu.a);
}

/* ----- MEMORY MICRO OPS ----- */

// fetch from memory at PC and increment PC - used for immediate as well as first byte of absolute
pub fn fetch_immediate(cpu: &mut CPU, mem: SysMem) { 
    cpu.bus.read_at(cpu.pc, mem);
    cpu.pc += 1;
}
// same as fetch but sets address bus. last read vale = AA, reads BB, address bus = BBAA -> BB
pub fn fetch_absolute_high(cpu: &mut CPU, mem: SysMem) { 
    let lo = cpu.bus.data;
    let hi = cpu.bus.read_at(cpu.pc, mem);
    cpu.bus.address = ((hi as u16) << 8) | (lo as u16);
    cpu.pc += 1;
}
// memory increments need read and write versions. readonly can speculatively read the wrong address and finish early if it happens to be correct. write instructions cant take the same risk.
pub fn fetch_absolute_x(cpu: &mut CPU, mem: SysMem) { 
    let lo = cpu.bus.data;
    let hi = cpu.bus.read_at(cpu.pc, mem);
    // calculate address
    let absolute_address = ((hi as u16) << 8) | (lo as u16);
    let incremented_address = absolute_address + cpu.x as u16;
    // detect an overflow into high byte, if present add an additional cycle
    if (absolute_address ^ incremented_address) > 0xFF {
        // actual 6502 takes a cycle to correct the 16bit addition while performing a dummy read
        cpu.m_op_queue.push_front(dummy_access);
    }
    cpu.bus.address = incremented_address;
    cpu.pc += 1;
}
pub fn fetch_absolute_y(cpu: &mut CPU, mem: SysMem) { 
    let lo = cpu.bus.data;
    let hi = cpu.bus.read_at(cpu.pc, mem);
    // calculate address
    let absolute_address = ((hi as u16) << 8) | (lo as u16);
    let incremented_address = absolute_address + cpu.y as u16;
    // detect an overflow into high byte, if present add an additional cycle
    if (absolute_address ^ incremented_address) > 0xFF {
        // actual 6502 takes a cycle to correct the 16bit addition while performing a dummy read
        cpu.m_op_queue.push_front(dummy_access);
    }
    cpu.bus.address = incremented_address;
    cpu.pc += 1;
}
// fixed refers to cycle counts, used in writes
pub fn fetch_absolute_x_fixed(cpu: &mut CPU, mem: SysMem) { 
    let lo = cpu.bus.data;
    let hi = cpu.bus.read_at(cpu.pc, mem);
    // calculate address
    cpu.bus.address = ((hi as u16) << 8) | (lo as u16) + cpu.x as u16;
    cpu.m_op_queue.push_front(dummy_access);
    cpu.pc += 1;
}
pub fn fetch_absolute_y_fixed(cpu: &mut CPU, mem: SysMem) { 
    let lo = cpu.bus.data;
    let hi = cpu.bus.read_at(cpu.pc, mem);
    // calculate address
    cpu.bus.address = ((hi as u16) << 8) | (lo as u16) + cpu.y as u16;
    cpu.m_op_queue.push_front(dummy_access);
    cpu.pc += 1;
}
// zero page adressing
pub fn fetch_zpg(cpu: &mut CPU, mem: SysMem) { 
    cpu.bus.address = cpu.bus.read_at(cpu.pc, mem) as u16;
    cpu.pc += 1;
}
pub fn inc_zpg_x(cpu: &mut CPU, _: SysMem) {
    let target = cpu.bus.address as u8 + cpu.x;
    cpu.bus.address = target as u16;
}
pub fn inc_zpg_y(cpu: &mut CPU, _: SysMem) {
    let target = cpu.bus.address as u8 + cpu.y;
    cpu.bus.address = target as u16;
}
// indirect addressing
pub fn fetch_indirect_low(cpu: &mut CPU, mem: SysMem) { 
    cpu.bus.read(mem);
    // indriect jump bug + zero page wrapping
    let lo = cpu.bus.address as u8 + 1;
    cpu.bus.address = (cpu.bus.address & 0xFF00) | lo as u16;
}
pub fn fetch_indirect_high(cpu: &mut CPU, mem: SysMem) { 
    let lo = cpu.bus.data;
    let hi = cpu.bus.read(mem);
    cpu.bus.address = ((hi as u16) << 8) | (lo as u16);
}
pub fn fetch_indirect_high_y(cpu: &mut CPU, mem: SysMem) { 
    let lo = cpu.bus.data;
    let hi = cpu.bus.read(mem);
    let absolute_address = ((hi as u16) << 8) | (lo as u16);
    let incremented_address = absolute_address + cpu.y as u16;
    // detect an overflow into high byte, if present add an additional cycle
    if (absolute_address ^ incremented_address) > 0xFF {
        // actual 6502 takes a cycle to correct the 16bit addition while performing a dummy read
        cpu.m_op_queue.push_front(dummy_access);
    }
    cpu.bus.address = incremented_address;
}
pub fn fetch_indirect_high_y_fixed(cpu: &mut CPU, mem: SysMem) { 
    let lo = cpu.bus.data;
    let hi = cpu.bus.read(mem);
    cpu.bus.address = ((hi as u16) << 8) | (lo as u16) + cpu.y as u16;
    cpu.m_op_queue.push_front(dummy_access);
}
pub fn dummy_access(_: &mut CPU, _: SysMem) {
    // its the same as no-op, but i wanted to distinguish this from the instruction
    // worth noting, the actual 6502 as the function name suggests, does a dummy memory access. No point emulating this, its just extra complexity.
}
// this is for instructions that take an extra cycle to write result back to memory
pub fn write_back(cpu: &mut CPU, mem: SysMem) {
    cpu.bus.write(mem);
}
// likewise for read-mod-write instructions, they cant pipeline the modification during the next ops fetch, so it needs to take an extra cycle to explicitly read operant
pub fn read_operand(cpu: &mut CPU, mem: SysMem) {
    cpu.bus.read(mem);
}
