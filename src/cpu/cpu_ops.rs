use super::*;

type SysMem<'a> = &'a mut dyn Addressable;

/* ----- MEMORY MICRO OPS ----- */

// fetch from memory at PC and increment PC
pub fn fetch_pc(cpu: &mut CPU, mem: SysMem) { 
    cpu.bus.read_at(cpu.pc, mem);
    cpu.pc += 1;
}
// same as fetch but sets address bus. last read vale = AA, reads BB, address bus = BBAA -> BB
pub fn fetch_pc_high(cpu: &mut CPU, mem: SysMem) { 
    let lo = cpu.bus.data;
    let hi = cpu.bus.read_at(cpu.pc, mem);
    cpu.bus.address = ((hi as u16) << 8) | (lo as u16);
    cpu.pc += 1;
}
pub fn fetch_zpg(cpu: &mut CPU, mem: SysMem) { 
    cpu.bus.address = cpu.bus.read_at(cpu.pc, mem) as u16;
    cpu.pc += 1;
}
pub fn fetch_indirect_low(cpu: &mut CPU, mem: SysMem) { 
    cpu.bus.read(mem);
    cpu.bus.address += 1;
}
pub fn fetch_indirect_high(cpu: &mut CPU, mem: SysMem) { 
    let lo = cpu.bus.data;
    let hi = cpu.bus.read(mem);
    cpu.bus.address = ((hi as u16) << 8) | (lo as u16);
}
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
    cpu.p.update_nz(operand);
}


