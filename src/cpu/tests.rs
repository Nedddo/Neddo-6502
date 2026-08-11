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
        fetch_immediate(&mut cpu, &mut mem);
        assert_eq!(cpu.bus.data, 0xEF);
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
        fetch_immediate(&mut cpu, &mut mem);
        assert_eq!(cpu.bus.data, 0xEF);
        fetch_absolute_high(&mut cpu, &mut mem);
        assert_eq!(cpu.bus.data, 0xBE);
        assert_eq!(cpu.bus.address, 0xBEEF);

    }
    // uses adc, validate this is functional in cpu_functionality_tests before running
    #[test]
    fn cpu_absolute_test() {
        let mut mem = TestMemory::new();
        // write adc immediate
        mem.write(0, 0x6D);

        mem.write(1, 0xEF);
        mem.write(2, 0xBE);
        mem.write(0xBEEF, 0x67);
        let mut cpu = CPU::new();
        // give register a some value
        cpu.a = 0;

        // should take 4 cycles
        cpu.cycle(&mut mem);
        cpu.cycle(&mut mem);
        cpu.cycle(&mut mem);
        cpu.cycle(&mut mem);

        assert_eq!(cpu.a, 0x67);

    }
}
#[cfg(test)] 
mod address_modes_dynamic_test {
    use super::*;

    fn setup() -> (CPU, TestMemory) {
        let cpu = CPU::new();
        let mem = TestMemory::new();
        (cpu, mem)
    }
    // uses lda to test cycle variance, when implemented will use sta to test cycle invariance with writes
    #[test]
    fn immediate_test() {
        let (mut cpu, mut mem) = setup();
        mem.write(0x8000, 0xA9);
        cpu.pc = 0x8000;
        // fetch
        cpu.cycle(&mut mem);
        let mut cycles = 1;
        while !cpu.m_op_queue.is_empty() {
            cpu.cycle(&mut mem);
            cycles += 1;
        }
        assert_eq!(cycles, 2);
    }
    #[test]
    fn zpg_test() {
        let (mut cpu, mut mem) = setup();
        mem.write(0x8000, 0xA5);
        mem.write(0x8001, 0x67);
        cpu.pc = 0x8000;
        // fetch
        cpu.cycle(&mut mem);
        let mut cycles = 1;
        while !cpu.m_op_queue.is_empty() {
            cpu.cycle(&mut mem);
            cycles += 1;
        }
        assert_eq!(cycles, 3);
        assert_eq!(cpu.bus.address, 0x0067);
    }
    // lda does not support zpg, y but its the same so doesnt need testing really
    #[test]
    fn zpg_x_test() {
        let (mut cpu, mut mem) = setup();
        mem.write(0x8000, 0xB5);
        mem.write(0x8001, 0x67);
        cpu.pc = 0x8000;
        // fetch
        cpu.cycle(&mut mem);
        let mut cycles = 1;
        while !cpu.m_op_queue.is_empty() {
            cpu.cycle(&mut mem);
            cycles += 1;
        }
        assert_eq!(cycles, 4);
        assert_eq!(cpu.bus.address, 0x0067);
    }
    #[test]
    fn zpg_x_overflow_test() {
        let (mut cpu, mut mem) = setup();
        mem.write(0x8000, 0xB5);
        mem.write(0x8001, 0xFF);
        cpu.pc = 0x8000;
        cpu.x = 0x68;
        // fetch
        cpu.cycle(&mut mem);
        let mut cycles = 1;
        while !cpu.m_op_queue.is_empty() {
            cpu.cycle(&mut mem);
            cycles += 1;
        }
        assert_eq!(cycles, 4);
        assert_eq!(cpu.bus.address, 0x0067);
    }
    #[test]
    fn indirect_x_test() {
        let (mut cpu, mut mem) = setup();
        mem.write(0x8000, 0xA1);
        mem.write(0x8001, 0x10);
        // target address should be BEEF
        mem.write(0x15, 0xEF);
        mem.write(0x16, 0xBE);
        cpu.pc = 0x8000;
        cpu.x = 0x05;
        // fetch
        cpu.cycle(&mut mem);
        let mut cycles = 1;
        while !cpu.m_op_queue.is_empty() {
            cpu.cycle(&mut mem);
            cycles += 1;
        }
        assert_eq!(cycles, 6);
        assert_eq!(cpu.bus.address, 0xBEEF);
    }
    #[test]
    fn indirect_y_test() {
        let (mut cpu, mut mem) = setup();
        mem.write(0x8000, 0xB1);
        mem.write(0x8001, 0x10);
        // target address should be BEEE
        mem.write(0x10, 0xEE);
        mem.write(0x11, 0xBE);
        cpu.pc = 0x8000;
        // BEEE + y = BEEF
        cpu.y = 0x01;
        // fetch
        cpu.cycle(&mut mem);
        let mut cycles = 1;
        while !cpu.m_op_queue.is_empty() {
            cpu.cycle(&mut mem);
            cycles += 1;
        }
        assert_eq!(cycles, 5);
        assert_eq!(cpu.bus.address, 0xBEEF);
    }
    #[test]
    fn indirect_y_boundary_cross_test() {
        let (mut cpu, mut mem) = setup();
        mem.write(0x8000, 0xB1);
        mem.write(0x8001, 0x10);
        // target address should be BEEE
        mem.write(0x10, 0xFF);
        mem.write(0x11, 0xBE);
        cpu.pc = 0x8000;
        // BEFF + y = BF00
        cpu.y = 0x01;
        // fetch
        cpu.cycle(&mut mem);
        let mut cycles = 1;
        while !cpu.m_op_queue.is_empty() {
            cpu.cycle(&mut mem);
            cycles += 1;
        }
        // page boundary cross should be 6 cycles
        assert_eq!(cycles, 6);
        assert_eq!(cpu.bus.address, 0xBF00);
    }
    #[test]
    fn abs_test() {
        let (mut cpu, mut mem) = setup();
        mem.write(0x8000, 0xAD);
        mem.write(0x8001, 0xEF);
        mem.write(0x8002, 0xBE);
        // target address should be BEEF
        cpu.pc = 0x8000;

        // fetch
        cpu.cycle(&mut mem);
        let mut cycles = 1;
        // count cycles
        while !cpu.m_op_queue.is_empty() {
            cpu.cycle(&mut mem);
            cycles += 1;
        }
        // page boundary cross should be 6 cycles
        assert_eq!(cycles, 4);
        assert_eq!(cpu.bus.address, 0xBEEF);
    }
    #[test]
    fn abs_x_test() {
        let (mut cpu, mut mem) = setup();
        mem.write(0x8000, 0xBD);
        mem.write(0x8001, 0xEF);
        mem.write(0x8002, 0xBE);
        // target address should be BEEF
        cpu.pc = 0x8000;
        cpu.x = 1;

        // fetch
        cpu.cycle(&mut mem);
        let mut cycles = 1;
        // count cycles
        while !cpu.m_op_queue.is_empty() {
            cpu.cycle(&mut mem);
            cycles += 1;
        }

        assert_eq!(cycles, 4);
        assert_eq!(cpu.bus.address, 0xBEF0);
    }
    #[test]
    fn abs_y_test() {
        let (mut cpu, mut mem) = setup();
        mem.write(0x8000, 0xB9);
        mem.write(0x8001, 0xEF);
        mem.write(0x8002, 0xBE);
        // target address should be BEEF
        cpu.pc = 0x8000;
        cpu.y = 1;

        // fetch
        cpu.cycle(&mut mem);
        let mut cycles = 1;
        // count cycles
        while !cpu.m_op_queue.is_empty() {
            cpu.cycle(&mut mem);
            cycles += 1;
        }

        assert_eq!(cycles, 4);
        assert_eq!(cpu.bus.address, 0xBEF0);
    }
    #[test]
    fn abs_x_overflow_test() {
        let (mut cpu, mut mem) = setup();
        mem.write(0x8000, 0xBD);
        mem.write(0x8001, 0xFF);
        mem.write(0x8002, 0xBE);
        // target address should be BEEF
        cpu.pc = 0x8000;
        cpu.x = 1;

        // fetch
        cpu.cycle(&mut mem);
        let mut cycles = 1;
        // count cycles
        while !cpu.m_op_queue.is_empty() {
            cpu.cycle(&mut mem);
            cycles += 1;
        }
        // page boundary cross should be 5 cycles
        assert_eq!(cycles, 5);
        assert_eq!(cpu.bus.address, 0xBF00);
    }
    #[test]
    fn abs_y_overflow_test() {
        let (mut cpu, mut mem) = setup();
        mem.write(0x8000, 0xB9);
        mem.write(0x8001, 0xFF);
        mem.write(0x8002, 0xBE);
        // target address should be BEEF
        cpu.pc = 0x8000;
        cpu.y = 1;

        // fetch
        cpu.cycle(&mut mem);
        let mut cycles = 1;
        // count cycles
        while !cpu.m_op_queue.is_empty() {
            cpu.cycle(&mut mem);
            cycles += 1;
        }
        // page boundary cross should be 5 cycles
        assert_eq!(cycles, 5);
        assert_eq!(cpu.bus.address, 0xBF00);
    }
}

mod address_modes_fixed_test {
    use super::*;

    fn setup() -> (CPU, TestMemory) {
        let cpu = CPU::new();
        let mem = TestMemory::new();
        (cpu, mem)
    }

    #[test]
    fn zpg_test() {
        let (mut cpu, mut mem) = setup();
        mem.write(0x8000, 0x85);
        mem.write(0x8001, 0x67);
        cpu.pc = 0x8000;
        cpu.cycle(&mut mem);
        let mut cycles = 1;
        while !cpu.m_op_queue.is_empty() {
            cpu.cycle(&mut mem);
            cycles += 1;
        }
        assert_eq!(cycles, 3);
        assert_eq!(cpu.bus.address, 0x0067);
    }

    #[test]
    fn abs_test() {
        let (mut cpu, mut mem) = setup();
        mem.write(0x8000, 0x8D);
        mem.write(0x8001, 0xEF);
        mem.write(0x8002, 0xBE);
        cpu.pc = 0x8000;
        cpu.cycle(&mut mem);
        let mut cycles = 1;
        while !cpu.m_op_queue.is_empty() {
            cpu.cycle(&mut mem);
            cycles += 1;
        }
        assert_eq!(cycles, 4);
        assert_eq!(cpu.bus.address, 0xBEEF);
    }

    #[test]
    fn indirect_x_test() {
        let (mut cpu, mut mem) = setup();
        mem.write(0x8000, 0x81);
        mem.write(0x8001, 0x10);
        mem.write(0x15, 0xEF);
        mem.write(0x16, 0xBE);
        cpu.pc = 0x8000;
        cpu.x = 0x05;
        cpu.cycle(&mut mem);
        let mut cycles = 1;
        while !cpu.m_op_queue.is_empty() {
            cpu.cycle(&mut mem);
            cycles += 1;
        }
        assert_eq!(cycles, 6);
        assert_eq!(cpu.bus.address, 0xBEEF);
    }

    // no page cross, but STA must still take the "slow" count -- this is
    // the fixed-cost vs early-exit distinction under test
    #[test]
    fn abs_x_no_cross_test() {
        let (mut cpu, mut mem) = setup();
        mem.write(0x8000, 0x9D);
        mem.write(0x8001, 0xEF);
        mem.write(0x8002, 0xBE);
        cpu.pc = 0x8000;
        cpu.x = 1;
        cpu.cycle(&mut mem);
        let mut cycles = 1;
        while !cpu.m_op_queue.is_empty() {
            cpu.cycle(&mut mem);
            cycles += 1;
        }
        // LDA would take 4 here; STA must still take 5
        assert_eq!(cycles, 5);
        assert_eq!(cpu.bus.address, 0xBEF0);
    }

    #[test]
    fn abs_y_no_cross_test() {
        let (mut cpu, mut mem) = setup();
        mem.write(0x8000, 0x99);
        mem.write(0x8001, 0xEF);
        mem.write(0x8002, 0xBE);
        cpu.pc = 0x8000;
        cpu.y = 1;
        cpu.cycle(&mut mem);
        let mut cycles = 1;
        while !cpu.m_op_queue.is_empty() {
            cpu.cycle(&mut mem);
            cycles += 1;
        }
        // LDA would take 4 here; STA must still take 5
        assert_eq!(cycles, 5);
        assert_eq!(cpu.bus.address, 0xBEF0);
    }

    #[test]
    fn indirect_y_no_cross_test() {
        let (mut cpu, mut mem) = setup();
        mem.write(0x8000, 0x91);
        mem.write(0x8001, 0x10);
        mem.write(0x10, 0xEE);
        mem.write(0x11, 0xBE);
        cpu.pc = 0x8000;
        cpu.y = 0x01;
        cpu.cycle(&mut mem);
        let mut cycles = 1;
        while !cpu.m_op_queue.is_empty() {
            cpu.cycle(&mut mem);
            cycles += 1;
        }
        // LDA would take 5 here; STA must still take 6
        assert_eq!(cycles, 6);
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

