//! 0x8c0918e4 — scan up to 36 bytes at r4+72; nonzero byte → r0 = 0, all zero → r0 = 1.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[0] = 0;
    cpu.r[4] = cpu.r[4].wrapping_add(72);
    cpu.r[1] = 36;
    loop {
        let b = mem.read_u8(cpu.r[0].wrapping_add(cpu.r[4])).unwrap_or(0);
        cpu.r[3] = b as u32;
        cpu.set_t(cpu.r[3] == 0); // tst r3,r3
        if cpu.r[3] != 0 {
            cpu.r[0] = 0; // rts delay slot
            return;
        }
        cpu.r[0] = cpu.r[0].wrapping_add(1);
        cpu.set_t(36 > (cpu.r[0] as i32)); // cmp/gt r0,r1
        if 36 > (cpu.r[0] as i32) {
            continue;
        }
        break;
    }
    cpu.r[0] = 1; // rts delay slot
}
