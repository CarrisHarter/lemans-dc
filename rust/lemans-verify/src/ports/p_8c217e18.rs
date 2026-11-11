//! 0x8c217e18 — load_word_from_bytes.
//!
//! Assembles a little-endian word from the four bytes at `r4`:
//! `b0 | b1<<8 | b2<<16 | b3<<24`.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[0] = mem.read_u8(cpu.r[4] + 1).unwrap_or(0) as u32;
    cpu.r[2] = mem.read_u8(cpu.r[4]).unwrap_or(0) as u32;
    cpu.r[0] = cpu.r[0] & 0xff;
    cpu.r[0] = cpu.r[0] << 8;
    cpu.r[3] = cpu.r[0];
    cpu.r[0] = mem.read_u8(cpu.r[4] + 2).unwrap_or(0) as u32;
    cpu.r[2] = cpu.r[2] & 0xff;
    cpu.r[0] = cpu.r[0] & 0xff;
    cpu.r[0] = cpu.r[0] << 16;
    cpu.r[3] = cpu.r[3] | cpu.r[0];
    cpu.r[0] = mem.read_u8(cpu.r[4] + 3).unwrap_or(0) as u32;
    cpu.r[0] = cpu.r[0] & 0xff;
    cpu.r[0] = cpu.r[0] << 16;
    cpu.r[0] = cpu.r[0] << 8;
    cpu.r[3] = cpu.r[3] | cpu.r[0];
    cpu.r[4] = cpu.r[3];
    cpu.r[4] = cpu.r[4] | cpu.r[2];
    cpu.r[0] = cpu.r[4]; // delay slot of `rts`
}
