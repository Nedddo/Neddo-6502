use crate::memory::TestMemory;
use crate::memory::Addressable;
use super::*;

#[cfg(test)] 
// test suite for cpu memory accesses 
mod cpu_memory_tests {
    use super::*;
    #[test]
    fn cpu_fetch_test() {
        let mut mem = TestMemory::new();
        // write 0xBEEF to memory (little endian)
        mem.write(0, 0xEF);
        mem.write(1, 0xBE);
        let mut cpu = CPU::new();
        // set address bus to 0
        cpu.bus.address = 0;
        assert_eq!(fetch_pc(&mut cpu, &mut mem), 0xEF);
    }
    #[test]
    fn cpu_fetch_address_test() {
        let mut mem = TestMemory::new();
        // write 0xBEEF to memory (little endian)
        mem.write(0, 0xEF);
        mem.write(1, 0xBE);
        let mut cpu = CPU::new();
        // set address bus to 0
        cpu.bus.address = 0;
        assert_eq!(fetch_pc(&mut cpu, &mut mem), 0xEF);
        assert_eq!(fetch_pc_high(&mut cpu, &mut mem), 0xBE);
        assert_eq!(cpu.bus.address, 0xBEEF);

    }
}
#[cfg(test)] 
mod cpu_functionality_tests {
    use super::*;
    #[test]
    fn cpu_tax_test() {
        let mut mem = TestMemory::new();
        // write TAX
        mem.write(0, 0xAA);
        let mut cpu = CPU::new();
        // give register a some value
        cpu.a = 0x67;
        // -- test 1
        println!("Testing register transfer...");

        cpu.cycle(&mut mem);
        cpu.cycle(&mut mem);

        assert_eq!(cpu.x, cpu.a);
        assert_eq!(cpu.p.n, false);
        assert_eq!(cpu.p.z, false);

        println!("Test Passed!");
        // -- test 2
        println!("Testing zero flag updates...");
        
        cpu.pc = 0;
        cpu.a = 0;

        cpu.cycle(&mut mem);
        cpu.cycle(&mut mem);

        assert_eq!(cpu.p.z, true);

        println!("Test Passed!");
        // -- test 3
        println!("Testing negative flag updates...");

        cpu.pc = 0;
        cpu.a = 0x80;

        cpu.cycle(&mut mem);
        cpu.cycle(&mut mem);

        assert_eq!(cpu.p.n, true);

        println!("Test Passed!");

    }
    #[test]
    fn cpu_adc_test() {
        let mut mem = TestMemory::new();
        // write adc immediate
        mem.write(0, 0x69); // 69 and 67!!!
        mem.write(1, 0x67);
        let mut cpu = CPU::new();
        // give register a some value
        cpu.a = 0x10;
        // -- test 1
        println!("Testing addition...");

        cpu.cycle(&mut mem);
        cpu.cycle(&mut mem);

        assert_eq!(cpu.a, 0x77);

        println!("Test Passed!");

        println!("Testing add where carry is true...");

        cpu.pc = 0;
        cpu.a = 0x0;
        cpu.p.c = true;

        cpu.cycle(&mut mem);
        cpu.cycle(&mut mem);

        assert_eq!(cpu.a, 0x68);

        println!("Test Passed!");

        println!("Testing add where carry occurs...");

        cpu.pc = 0;
        cpu.a = 0xF9;

        cpu.cycle(&mut mem);
        cpu.cycle(&mut mem);

        assert_eq!(cpu.a, 0x60);
        assert_eq!(cpu.p.c, true);
        assert_eq!(cpu.p.v, false);

        println!("Test Passed!");

        println!("Testing add where carry and overflow occurs...");

        cpu.pc = 0;
        cpu.a = 0x80;
        mem.write(1, 0x80);

        cpu.cycle(&mut mem);
        cpu.cycle(&mut mem);

        assert_eq!(cpu.p.c, true);
        assert_eq!(cpu.p.v, true);

        println!("Test Passed!");
        println!("Testing add where only overflow occurs...");

        cpu.pc = 0;
        cpu.a = 0x7F;
        cpu.p.c = true; 
        mem.write(1, 0x00);
        // 127 + 00 + 1 = -128

        cpu.cycle(&mut mem);
        cpu.cycle(&mut mem);

        assert_eq!(cpu.p.c, false);
        assert_eq!(cpu.p.v, true);

        println!("Test Passed!");
    }
}

