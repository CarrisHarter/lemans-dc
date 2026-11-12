//! 0x8c19c95c - clear a float/byte pair, then bump the refcount at `[r4+20]`.
//!
//! 0x8c19d33c(r4) clears `[r4+16]` and byte `[r4+8]`; then 0x8c19d48c runs on
//! `[r14+20]`, which increments that refcount and initialises its bits when it
//! was zero.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[14]);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr);
    cpu.r[14] = cpu.r[4]; // mov r4,r14
    cpu.r[0] = mem.read_u32(0x8c19c974).unwrap_or(0x8c19d33c); // mov.l 0x8c19c974,r0
    cpu.pr = 0x8c19c968; // jsr delay is nop
    crate::registry::p_8c19d33c::run(cpu, mem);
    cpu.r[0] = mem.read_u32(0x8c19c978).unwrap_or(0x8c19d48c); // mov.l 0x8c19c978,r0
    cpu.r[4] = mem.read_u32(cpu.r[14].wrapping_add(20)).unwrap_or(0); // jsr delay: mov.l @(20,r14),r4
    cpu.pr = 0x8c19c96e;
    crate::registry::p_8c19d48c::run(cpu, mem);
    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[14] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}
