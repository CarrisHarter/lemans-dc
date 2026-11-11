//! 0x8c2316a0 — *0x8c2c8a5c=r4; r0=*0x8c2c8a50|2; *0x8c2c8a50=r0; rts delay r0=0.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[3] = 0x8c2c8a5c;
    let _ = mem.write_u32(cpu.r[3], cpu.r[4]);
    cpu.r[2] = 0x8c2c8a50;
    cpu.r[0] = mem.read_u32(cpu.r[2]).unwrap_or(0) | 2;
    let _ = mem.write_u32(cpu.r[2], cpu.r[0]);
    cpu.r[0] = 0; // rts delay
}
