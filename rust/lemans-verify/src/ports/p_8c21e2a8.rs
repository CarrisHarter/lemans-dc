//! 0x8c21e2a8 — r3=0xffd80004; r0=byte[r3]&0xfb; rts delay byte[r3]=r0.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[3] = 0xffd80004;
    cpu.r[0] = mem.read_u8(cpu.r[3]).unwrap_or(0) as u32 & 0xfb;
    let _ = mem.write_u8(cpu.r[3], cpu.r[0] as u8); // rts delay
}
