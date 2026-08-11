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
// - stack transfers do not update any flags
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
    cpu.p.v = (((result ^ operand_a) & (result ^ operand_b)) & 0x80) != 0; // will be negative if and only if sign change ocurs between both operands
}
pub fn sbc(cpu: &mut CPU, mem: SysMem) {
    // read operand
    let operand_a = cpu.a;
    let operand_b = cpu.bus.read(mem);
    let borrow = !cpu.p.c as u8;
    cpu.a = operand_a - operand_b - borrow;
    // update flags
    cpu.p.update_nz(cpu.a);
    // carry flag is NOT borrow
    cpu.p.c = !((operand_a as u16) < (operand_b as u16 + borrow as u16));
    // signed overflow can only occur if a and b have different signage
    cpu.p.v = ((operand_a ^ operand_b) & (cpu.a ^ operand_a) & 0x80) != 0;
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
// comparison operations
pub fn cmp(cpu: &mut CPU, mem: SysMem) {
    let operand_a = cpu.a;
    let operand_b = cpu.bus.read(mem);

    let result = operand_a - operand_b;
    // update flags
    cpu.p.update_nz(result);
    // carry flag is NOT borrow
    cpu.p.c = !(operand_a < operand_b);
}
pub fn cpx(cpu: &mut CPU, mem: SysMem) {
    let operand_a = cpu.x;
    let operand_b = cpu.bus.read(mem);

    let result = operand_a - operand_b;
    // update flags
    cpu.p.update_nz(result);
    // carry flag is NOT borrow
    cpu.p.c = !(operand_a < operand_b);
}

pub fn cpy(cpu: &mut CPU, mem: SysMem) {
    let operand_a = cpu.y;
    let operand_b = cpu.bus.read(mem);

    let result = operand_a - operand_b;
    // update flags
    cpu.p.update_nz(result);
    // carry flag is NOT borrow
    cpu.p.c = !(operand_a < operand_b);
}
pub fn bit(cpu: &mut CPU, mem: SysMem) {
    let operand = cpu.bus.read(mem);
    cpu.p.z = (operand & cpu.a) == 0;
    cpu.p.v = (Flags::F_V & operand) != 0;
    cpu.p.n = (Flags::F_N & operand) != 0;
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
pub fn php (cpu: &mut CPU, mem: SysMem) {
    cpu.p.b = true;
    push(cpu, mem, cpu.p.value());
    cpu.p.b = false;
}
pub fn pha (cpu: &mut CPU, mem: SysMem) {
    push(cpu, mem, cpu.a);
}
pub fn plp (cpu: &mut CPU, mem: SysMem) {
    let p_val = pull(cpu, mem);
    cpu.p.set(p_val);
    cpu.p.b = false; // this isnt a real flag
}
pub fn pla (cpu: &mut CPU, mem: SysMem) {
    cpu.a = pull(cpu, mem);
    cpu.p.update_nz(cpu.a);
}
// read modify execute
pub fn inc(cpu: &mut CPU, _: SysMem) {
    cpu.bus.data += 1;
    cpu.p.update_nz(cpu.bus.data);
}
pub fn dec(cpu: &mut CPU, _: SysMem) {
    cpu.bus.data -= 1;
    cpu.p.update_nz(cpu.bus.data);
}

pub fn asl(cpu: &mut CPU, _: SysMem) {
    // check if bit 7 is shifted out
    cpu.p.c = (cpu.bus.data & 0x80 ) != 0;
    cpu.bus.data <<= 1;
    cpu.p.update_nz(cpu.bus.data);
}
pub fn asl_a(cpu: &mut CPU, _: SysMem) {
    cpu.p.c = (cpu.a & 0x80 ) != 0;
    cpu.a <<= 1;
    cpu.p.update_nz(cpu.a);
}

pub fn lsr(cpu: &mut CPU, _: SysMem) {
    // check if bit 7 is shifted out
    cpu.p.c = (cpu.bus.data & 0x01 ) != 0;
    cpu.bus.data >>= 1;
    cpu.p.update_nz(cpu.bus.data);
}
pub fn lsr_a(cpu: &mut CPU, _: SysMem) {
    cpu.p.c = (cpu.a & 0x01 ) != 0;
    cpu.a >>= 1;
    cpu.p.update_nz(cpu.a);
}

pub fn rol(cpu: &mut CPU, _: SysMem) {
    // check if bit 7 is shifted out
    cpu.p.c = (cpu.bus.data & 0x80 ) != 0;
    cpu.bus.data <<= 1;
    cpu.bus.data |= cpu.p.c as u8;
    cpu.p.update_nz(cpu.bus.data);
}
pub fn rol_a(cpu: &mut CPU, _: SysMem) {
    cpu.p.c = (cpu.a & 0x80 ) != 0;
    cpu.a <<= 1;
    cpu.bus.data |= cpu.p.c as u8;
    cpu.p.update_nz(cpu.a);
}
pub fn ror(cpu: &mut CPU, _: SysMem) {
    // check if bit 7 is shifted out
    cpu.p.c = (cpu.bus.data & 0x01) != 0;
    cpu.bus.data >>= 1;
    cpu.bus.data |= (cpu.p.c as u8) << 7;
    cpu.p.update_nz(cpu.bus.data);
}
pub fn ror_a(cpu: &mut CPU, _: SysMem) {
    cpu.p.c = (cpu.a & 0x01 ) != 0;
    cpu.a >>= 1;
    cpu.bus.data |= (cpu.p.c as u8) << 7;
    cpu.p.update_nz(cpu.a);
}
// branch instructions
fn evaluated_branch(cpu: &mut CPU, mem: SysMem) {
    let offset = cpu.bus.read(mem) as i8; // read immediate val as signed int
    let old_pc = cpu.pc;
    cpu.pc += offset as u16;
    // bitwise op detects if a carry into the high byte occured (equal values xord always evaluate to 0)
    if (cpu.pc ^ old_pc) > 0xFF {
        // extra cycle for adjustment
        cpu.queue(dummy_access);
    }
}
pub fn beq(cpu: &mut CPU, _: SysMem) {
    if cpu.p.z {
        cpu.queue(evaluated_branch);
    }
}
pub fn bne(cpu: &mut CPU, _: SysMem) {
    if !cpu.p.z {
        cpu.queue(evaluated_branch);
    }
}
pub fn bcs(cpu: &mut CPU, _: SysMem) {
    if cpu.p.c {
        cpu.queue(evaluated_branch);
    }
}
pub fn bcc(cpu: &mut CPU, _: SysMem) {
    if !cpu.p.c {
        cpu.queue(evaluated_branch);
    }
}
pub fn bmi(cpu: &mut CPU, _: SysMem) {
    if cpu.p.n {
        cpu.queue(evaluated_branch);
    }
}
pub fn bpl(cpu: &mut CPU, _: SysMem) {
    if !cpu.p.n {
        cpu.queue(evaluated_branch);
    }
}
pub fn bvs(cpu: &mut CPU, _: SysMem) {
    if cpu.p.v {
        cpu.queue(evaluated_branch);
    }
}
pub fn bvc(cpu: &mut CPU, _: SysMem) {
    if !cpu.p.v {
        cpu.queue(evaluated_branch);
    }
}
// not hugely sure where jmp fits here
pub fn jmp(cpu: &mut CPU, _: SysMem) {
    cpu.pc = cpu.bus.address;
}
// interrupts, subroutines
pub fn jsr(cpu: &mut CPU, _: SysMem) {
    cpu.pc += 1;
    cpu.queue(push_pc_high);
    cpu.queue(push_pc_low);
    cpu.pc = cpu.bus.address;
}
pub fn brk(cpu: &mut CPU, mem: SysMem) { 
    // read padding byte
    fetch_immediate(cpu, mem);
    // queue the rest of the brk instruction
    cpu.queue(push_pc_high);
    cpu.queue(push_pc_low);
    cpu.queue(php);
    cpu.queue(irq_low);
    cpu.queue(irq_high);
}
pub fn rti(cpu: &mut CPU, mem: SysMem) {
    cpu.queue(plp);
    cpu.queue(pull_pc_low);
    cpu.queue(pull_pc_high);
    cpu.queue(dummy_access); // apply flags / fetch next
}
fn inc_pc(cpu: &mut CPU, _: SysMem) {
    cpu.pc += 1;
}
pub fn rts(cpu: &mut CPU, mem: SysMem) {
    cpu.queue(pull_pc_low);
    cpu.queue(pull_pc_high);
    cpu.queue(inc_pc); // increment
    cpu.queue(dummy_access); // apply flags / fetch next
}


/* ----- INTERRUPT MICRO OPS ----- */


// TODO: seeing as this pattern is so common, I should make a generalised low/high read function
// that i can call from each specialised one with prefilled operands
// cant be assed refactoring rn though
pub fn irq_low(cpu: &mut CPU, mem: SysMem) { 
    cpu.bus.read_at(0xFFFE, mem);
}
pub fn irq_high(cpu: &mut CPU, mem: SysMem) { 
    let lo = cpu.bus.data as u16;
    let hi = cpu.bus.read_at(0xFFFE, mem) as u16;
    cpu.bus.address = (hi << 8) | lo;
}
pub fn nmi_low(cpu: &mut CPU, mem: SysMem) { 
    cpu.bus.read_at(0xFFFA, mem);
}
pub fn nmi_high(cpu: &mut CPU, mem: SysMem) { 
    let lo = cpu.bus.data as u16;
    let hi = cpu.bus.read_at(0xFFFB, mem) as u16;
    cpu.bus.address = (hi << 8) | lo;
}
pub fn reset_low(cpu: &mut CPU, mem: SysMem) { 
    cpu.bus.read_at(0xFFFC, mem);
}
pub fn reset_high(cpu: &mut CPU, mem: SysMem) { 
    let lo = cpu.bus.data as u16;
    let hi = cpu.bus.read_at(0xFFFD, mem) as u16;
    cpu.bus.address = (hi << 8) | lo;
}


/* ----- MEMORY MICRO OPS ----- */



// used in jsr, brk and interrupt requests
pub fn push_pc_low (cpu: &mut CPU, mem: SysMem) { 
    let pc_low = (cpu.pc & 0xFF) as u8;
    push(cpu, mem, pc_low);
}
pub fn push_pc_high (cpu: &mut CPU, mem: SysMem) { 
    let pc_high = ((cpu.pc & 0xFF00) >> 8) as u8;
    push(cpu, mem, pc_high);
}
pub fn pull_pc_low (cpu: &mut CPU, mem: SysMem) {
    pull(cpu, mem);
}
pub fn pull_pc_high (cpu: &mut CPU, mem: SysMem) {
    let lo = cpu.bus.address; 
    let hi = pull(cpu, mem);
    cpu.bus.address = ((hi as u16) << 8 ) | (lo);
}
// helper for push instructions
fn push(cpu: &mut CPU, mem: SysMem, data: u8) {
    let address = 0x0100 | cpu.s as u16;
    cpu.bus.write_value_to(address, data, mem);
    cpu.s -= 1;
}
fn pull(cpu: &mut CPU, mem: SysMem) -> u8 {
    cpu.s += 1;
    let address = 0x0100 | cpu.s as u16;
    cpu.bus.read_at(address, mem)
}
// fetch from memory at PC and increment PC - used for immediate as well as first byte of absolute
pub fn fetch_immediate(cpu: &mut CPU, mem: SysMem) { 
    cpu.bus.read_at(cpu.pc, mem);
    cpu.pc += 1;
}
// same as fetch but sets address bus. last read vale = AA, reads BB, address bus = BBAA -> BB
pub fn fetch_absolute_high(cpu: &mut CPU, mem: SysMem) { 
    let lo = cpu.bus.data;
    let hi = cpu.bus.read_at(cpu.pc, mem);
    cpu.bus.address = ((hi as u16) << 8) | (lo as u16); // TODO: refactor later (see interrupt reads)
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
