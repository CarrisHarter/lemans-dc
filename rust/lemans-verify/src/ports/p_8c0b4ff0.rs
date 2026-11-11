//! 0x8c0b4ff0 — fr0=0 → r4+8, r4+4, rts delay r4.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[1] = cpu.r[4].wrapping_add(8);
    cpu.fr[0] = 0.0;
    cpu.r[0] = cpu.r[4];
    let _ = mem.write_u32(cpu.r[1], 0);
    cpu.r[0] = cpu.r[0].wrapping_add(4);
    let _ = mem.write_u32(cpu.r[0], 0);
    let _ = mem.write_u32(cpu.r[4], 0); // rts delay
}
