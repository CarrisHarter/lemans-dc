//! 0x8c06de78 — zero a struct, then clear two words at +80 and +84.
//!
//! Calls 0x8c0ed854 (zero-and-return) with the same r4, then stores 0 at
//! `[r14+80]` and `[r14+84]`. `r0 = r14` at exit; r14 and pr are restored.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let saved14 = cpu.r[14];
    cpu.r[15] = cpu.r[15].wrapping_sub(8);
    cpu.r[14] = cpu.r[4];
    cpu.r[0] = mem.read_u32(0x8c06de98).unwrap_or(0x8c0ed854);
    cpu.pr = 0x8c06de84;
    crate::registry::p_8c0ed854::run(cpu, mem);
    cpu.r[0] = 80;
    cpu.r[1] = 0;
    let _ = mem.write_u32(cpu.r[14].wrapping_add(cpu.r[0]), cpu.r[1]);
    cpu.r[0] = 84;
    let _ = mem.write_u32(cpu.r[14].wrapping_add(cpu.r[0]), cpu.r[1]);
    cpu.r[0] = cpu.r[14];
    cpu.r[15] = cpu.r[15].wrapping_add(8);
    cpu.r[14] = saved14;
}
