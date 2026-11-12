//! 0x8c014ee4 — copy one 8-byte record out of a fixed table.
//!
//! `r4` indexes the table at `[0x8c014f00]` (0x8c1cd290) in 8-byte units and
//! `r5` is the destination. The two arguments are spilled to the 8-byte scratch
//! frame and reloaded swapped, so the tail-called memmove at `[0x8c014f04]`
//! (0x8c010e84) sees `r4 = dst`, `r5 = src`, `r6 = 8`. The index is scaled with
//! `shll2`/`shll` (32-bit wrap) and added to the table base. The `jmp` is a
//! tail call: the delay slot restores `r15` before the callee runs, so `r15` is
//! entry-equal when 0x8c010e84 pushes its frame, and its `rts` is the wrapper's
//! return. `pr` is untouched; `r2` keeps the callee address, `r3` the table
//! base, for whichever path inside memmove does not overwrite them.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[15] = cpu.r[15].wrapping_sub(8); // add #-8,r15
    cpu.r[3] = mem.read_u32(0x8c014f00).unwrap_or(0); // mov.l @(0x8c014f00),r3
    cpu.r[6] = 8; // mov #8,r6
    let _ = mem.write_u32(cpu.r[15].wrapping_add(4), cpu.r[4]); // mov.l r4,@(4,r15)
    let _ = mem.write_u32(cpu.r[15], cpu.r[5]); // mov.l r5,@r15
    cpu.r[5] = mem.read_u32(cpu.r[15].wrapping_add(4)).unwrap_or(0); // mov.l @(4,r15),r5
    cpu.r[4] = mem.read_u32(cpu.r[15]).unwrap_or(0); // mov.l @r15,r4
    cpu.r[5] = cpu.r[5].wrapping_shl(2); // shll2 r5
    cpu.r[2] = mem.read_u32(0x8c014f04).unwrap_or(0); // mov.l @(0x8c014f04),r2
    cpu.r[5] = cpu.r[5].wrapping_shl(1); // shll r5
    cpu.r[5] = cpu.r[5].wrapping_add(cpu.r[3]); // add r3,r5
    cpu.r[15] = cpu.r[15].wrapping_add(8); // delay slot of `jmp @r2`
    crate::registry::p_8c010e84::run(cpu, mem);
}
