//! 0x8c0d4768 — same shape as 0x8c0d4754 but r0 = 2 stored at r4+56.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[0] = 61;
    cpu.r[3] = 0;
    cpu.r[1] = cpu.r[4];
    let _ = mem.write_u8(cpu.r[0].wrapping_add(cpu.r[4]), 0);
    cpu.r[1] = cpu.r[1].wrapping_add(80);
    cpu.fr[0] = 1.0;
    let _ = mem.write_u32(cpu.r[1], cpu.fr[0].to_bits());
    cpu.r[0] = 2;
    let _ = mem.write_u32(cpu.r[4] + 56, cpu.r[0]); // rts delay slot
}
