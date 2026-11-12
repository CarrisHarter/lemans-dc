//! 0x8c0b4838 — add_three_floats_at_20.
//!
//! Saves r14 and pr, then calls 0x8c0b4f34 with `r4 += 20`, which adds the
//! three floats at `[r4+20..r4+28]` to the three floats at `[r5..r5+8]` in
//! place and leaves `r4 = old r4+28`, `r5 = old r5+8`. Finally clears the byte
//! at the original `[r4]` and restores r14/pr.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let base = cpu.r[4];
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[14]);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr);
    cpu.r[14] = base;
    cpu.r[0] = mem.read_u32(0x8c0b4850).unwrap_or(0);
    cpu.r[4] = cpu.r[4].wrapping_add(20); // delay slot of `jsr`
    cpu.pr = 0x8c0b4844;
    crate::registry::p_8c0b4f34::run(cpu, mem);
    cpu.r[0] = 0;
    let _ = mem.write_u8(cpu.r[14], 0);
    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[14] = mem.read_u32(cpu.r[15]).unwrap_or(0); // delay slot of `rts`
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}
