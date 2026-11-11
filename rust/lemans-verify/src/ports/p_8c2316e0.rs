//! 0x8c2316e0 — *0x8c2c8a58=*r15; r0=*0x8c2c8a50|1; *0x8c2c8a50=r0; rts delay r0=0.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[2] = 0x8c2c8a58;
    cpu.r[3] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    let _ = mem.write_u32(cpu.r[2], cpu.r[3]);
    cpu.r[3] = 0x8c2c8a50;
    cpu.r[0] = mem.read_u32(cpu.r[3]).unwrap_or(0) | 1;
    let _ = mem.write_u32(cpu.r[3], cpu.r[0]);
    cpu.r[0] = 0; // rts delay
}
