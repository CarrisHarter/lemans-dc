//! 0x8c0b5e78 — *r4 = 0x8c1eaaac; rts delay r0 = r4.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[0] = 0x8c1eaaac;
    let _ = mem.write_u32(cpu.r[4], cpu.r[0]);
    cpu.r[0] = cpu.r[4]; // rts delay slot
}
