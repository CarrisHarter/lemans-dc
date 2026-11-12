//! 0x8c091a80 — zero a two-block record: `memset(r4, 0, 0xb8)` then
//! `memset(r4+0xb8, 0, 0x90)` (0x8c010f88), returning `r4`.
//!
//! The two literals next to the code are the sizes (0xb8, 0x90) and the memset
//! entry (0x8c010f88). r6 is clobbered by both literal loads and ends at 0x90;
//! the second memset leaves r4 at r4+0xb8, r3 = 0, r7 = 0x90 and T set. r14 is
//! saved on the stack and restored by the `rts` delay slot, so it keeps its
//! entry value.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[14]);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr);
    cpu.r[14] = cpu.r[4];
    cpu.r[6] = mem.read_u32(0x8c091aa4).unwrap_or(0);
    cpu.r[0] = mem.read_u32(0x8c091aa8).unwrap_or(0);
    cpu.pr = 0x8c091a8e;
    cpu.r[5] = 0; // delay slot of `jsr`
    crate::registry::p_8c010f88::run(cpu, mem); // memset(r4, 0, 0xb8)
    cpu.r[0] = mem.read_u32(0x8c091aa4).unwrap_or(0);
    cpu.r[4] = cpu.r[14];
    cpu.r[6] = mem.read_u32(0x8c091aac).unwrap_or(0);
    cpu.r[4] = cpu.r[4].wrapping_add(cpu.r[0]);
    cpu.r[0] = mem.read_u32(0x8c091aa8).unwrap_or(0);
    cpu.pr = 0x8c091a9c;
    cpu.r[5] = 0; // delay slot of `jsr`
    crate::registry::p_8c010f88::run(cpu, mem); // memset(r4+0xb8, 0, 0x90)
    cpu.r[0] = cpu.r[14];
    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[14] = mem.read_u32(cpu.r[15]).unwrap_or(0); // delay slot of `rts`
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}
