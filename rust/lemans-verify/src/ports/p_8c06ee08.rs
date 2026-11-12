//! 0x8c06ee08 — zero a struct, clear four words, and store 0.0 at a global.
//!
//! Calls 0x8c0ed854, then writes 0 at `[r14+80]`, `[r14+84]`, `[r14+0x828]` and
//! `[r14+0x82c]`. `fldi0 fr0` then `fmov fr0,@r1` stores +0.0 at the literal
//! address 0x8c240450. `r0 = r14` at exit; r14 and pr are restored. `fldi0`
//! does not set T.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let saved14 = cpu.r[14];
    cpu.r[15] = cpu.r[15].wrapping_sub(8);
    cpu.r[14] = cpu.r[4];
    cpu.r[0] = mem.read_u32(0x8c06ee34).unwrap_or(0x8c0ed854);
    cpu.pr = 0x8c06ee14;
    crate::registry::p_8c0ed854::run(cpu, mem);
    cpu.r[0] = 80;
    cpu.r[3] = 0;
    let _ = mem.write_u32(cpu.r[14].wrapping_add(cpu.r[0]), cpu.r[3]);
    cpu.fr[0] = 0.0; // fldi0 fr0
    cpu.r[1] = mem.read_u32(0x8c06ee38).unwrap_or(0x8c240450);
    cpu.r[0] = 84;
    let _ = mem.write_u32(cpu.r[14].wrapping_add(cpu.r[0]), cpu.r[3]);
    let _ = mem.write_f32(cpu.r[1], cpu.fr[0]);
    cpu.r[0] = mem.read_u32(0x8c06ee3c).unwrap_or(0x828);
    let _ = mem.write_u32(cpu.r[14].wrapping_add(cpu.r[0]), cpu.r[3]);
    cpu.r[0] = mem.read_u32(0x8c06ee40).unwrap_or(0x82c);
    let _ = mem.write_u32(cpu.r[14].wrapping_add(cpu.r[0]), cpu.r[3]);
    cpu.r[0] = cpu.r[14];
    cpu.r[15] = cpu.r[15].wrapping_add(8);
    cpu.r[14] = saved14;
}
