//! 0x8c093048 — r1 = 0 only if both bytes at r4+0x95 and r4+0x96 are zero, else 1.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[0] = 0x95;
    let b1 = mem.read_u8(cpu.r[0].wrapping_add(cpu.r[4])).unwrap_or(0);
    cpu.r[0] = b1 as u32; // extu.b x2
    cpu.set_t(cpu.r[0] == 0); // tst r0,r0
    cpu.r[1] = 1; // bf.s delay slot
    if cpu.r[0] == 0 {
        cpu.r[0] = 0x96;
        let b2 = mem.read_u8(cpu.r[0].wrapping_add(cpu.r[4])).unwrap_or(0);
        cpu.r[0] = b2 as u32;
        cpu.set_t(cpu.r[0] == 0);
        if cpu.r[0] == 0 {
            cpu.r[1] = 0;
        }
    }
}
