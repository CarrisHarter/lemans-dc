//! 0x8c10b958 — *r4 = **r4; r0 = *r15; rts delay *r0 = r1.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[1] = mem.read_u32(cpu.r[4]).unwrap_or(0);
    cpu.r[0] = mem.read_u32(cpu.r[1]).unwrap_or(0);
    let _ = mem.write_u32(cpu.r[4], cpu.r[0]);
    cpu.r[0] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    let _ = mem.write_u32(cpu.r[0], cpu.r[1]); // rts delay
}
