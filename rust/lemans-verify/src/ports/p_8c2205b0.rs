//! 0x8c2205b0 — r0 = (*r4)*32 + **0x8c23d468; r4 = r0; *(r4+28) = 0; rts delay r0 = 0.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[2] = 0x8c23d468;
    cpu.r[1] = 0;
    cpu.r[0] = mem.read_u32(cpu.r[4]).unwrap_or(0);
    cpu.r[3] = mem.read_u32(cpu.r[2]).unwrap_or(0);
    cpu.r[0] = ((cpu.r[0] << 2) << 2) << 1; // *32
    cpu.r[3] = mem.read_u32(cpu.r[3]).unwrap_or(0);
    cpu.r[0] = cpu.r[0].wrapping_add(cpu.r[3]);
    cpu.r[4] = cpu.r[0];
    let _ = mem.write_u32(cpu.r[4] + 28, cpu.r[1]);
    cpu.r[0] = cpu.r[1]; // rts delay slot
}
