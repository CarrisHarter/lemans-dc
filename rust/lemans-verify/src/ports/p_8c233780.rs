//! 0x8c233780 — r2=*r4 & 0xdfffffff; rts delay *r4=r2.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[2] = mem.read_u32(cpu.r[4]).unwrap_or(0);
    cpu.r[3] = 0xdfffffff;
    cpu.r[2] &= cpu.r[3];
    let _ = mem.write_u32(cpu.r[4], cpu.r[2]); // rts delay
}
