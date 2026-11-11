//! 0x8c1bb298 — *r4 = r5; rts delay r0 = r4.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let _ = mem.write_u32(cpu.r[4], cpu.r[5]);
    cpu.r[0] = cpu.r[4]; // rts delay
}
