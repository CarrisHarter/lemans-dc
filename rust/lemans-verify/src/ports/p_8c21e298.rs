//! 0x8c21e298 — byte[0xffd80000]=0; r0=byte[0xffd80004]|4; rts delay byte[0xffd80004]=r0.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[1] = 0xffd80000;
    cpu.r[2] = 0;
    let _ = mem.write_u8(cpu.r[1], 0);
    cpu.r[3] = 0xffd80004;
    cpu.r[0] = mem.read_u8(cpu.r[3]).unwrap_or(0) as u32 | 4;
    let _ = mem.write_u8(cpu.r[3], cpu.r[0] as u8); // rts delay
}
