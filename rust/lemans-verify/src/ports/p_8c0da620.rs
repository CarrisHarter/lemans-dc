//! 0x8c0da620 — fr0=0 → r4, r4+4; rts delay r0=r4.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.fr[0] = 0.0;
    cpu.r[0] = cpu.r[4];
    let _ = mem.write_u32(cpu.r[4], 0);
    cpu.r[0] = cpu.r[0].wrapping_add(4);
    let _ = mem.write_u32(cpu.r[0], 0);
    cpu.r[0] = cpu.r[4]; // rts delay
}
