//! 0x8c01ec2c — `return *(u32 *)(param + 0xc);`
//!
//! `rts ; mov.l @(12,r4),r0` — the load lives in the delay slot and is the
//! whole routine: r0 becomes the return value, T and memory are untouched.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[0] = mem.read_u32(cpu.r[4].wrapping_add(12)).unwrap_or(0);
}
