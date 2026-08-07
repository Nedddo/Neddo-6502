use super::*;

type SysMem<'a> = &'a mut dyn Addressable;

/* ----- MEMORY MICRO OPS ----- */

// fetch from memory at PC and increment PC
pub fn fetch_pc(cpu: &mut CPU, mem:SysMem) -> u8 { 
    let read = cpu.bus.read_at(cpu.pc, mem);
    cpu.pc += 1;
    read
}
// same as fetch but sets address bus. last read vale = AA, reads BB, address bus = BBAA -> BB
pub fn fetch_pc_high(cpu: &mut CPU, mem: SysMem) -> u8 { 
    let lo = cpu.bus.data;
    let hi = cpu.bus.read_at(cpu.pc, mem);
    let address = ((hi as u16) << 8) | (lo as u16);
    cpu.pc += 1;
    cpu.bus.address = address;
    hi // hello!
}

/* ----- INSTRUCTION MICRO OPS ----- */

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

