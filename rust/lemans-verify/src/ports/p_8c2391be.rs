//! 0x8c2391be — list node init; rts delay r0=0.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[2] = 0x8c239120;
    cpu.r[6] = 0;
    cpu.r[3] = 0x8c239134;
    let b = cpu.r[4];
    let _ = mem.write_u32(b, cpu.r[5]);
    let _ = mem.write_u32(b + 8, cpu.r[2]);
    cpu.r[2] = 0x8c23917c;
    let _ = mem.write_u32(b + 16, cpu.r[3]);
    cpu.r[3] = 0x8c23d3b0;
    let _ = mem.write_u32(b + 4, cpu.r[6]);
    let _ = mem.write_u32(b + 20, cpu.r[2]);
    cpu.r[2] = 0x8c23d410;
    let _ = mem.write_u32(cpu.r[5] + 4, cpu.r[3]);
    let _ = mem.write_u32(cpu.r[5] + 8, cpu.r[2]);
    cpu.r[0] = cpu.r[6]; // rts delay
}
