//! 0x8c0dfc18 — zero a 0x190-byte block, stamp a header, publish it in a global.
//!
//! `mov r4,r14`; the pointers 0x8c27a928 (published slot), 0x190 (length and the
//! zeroed dword offset) and 0x194 (flag byte offset) come from the pool, and the
//! `jsr` delay slot stores r4 into the global *before* the memset runs. Then
//! `[r4+0x190] = 0` (long) and `[r4+0x194] = 1` (byte). `r0 = r14` at the end;
//! r14 is pushed, so it ends at its entry value.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let object = cpu.r[4]; // mov r4,r14
    let slot = mem.read_u32(0x8c0dfc40).unwrap_or(0x8c27a928); // mov.l 0x8c0dfc40,r1
    let len = mem.read_u32(0x8c0dfc44).unwrap_or(0x190); // mov.l 0x8c0dfc44,r6
    let memset = mem.read_u32(0x8c0dfc48).unwrap_or(0x8c010f88); // mov.l 0x8c0dfc48,r0
    let flag_off = mem.read_u32(0x8c0dfc4c).unwrap_or(0x194); // mov.l 0x8c0dfc4c,r0
    cpu.r[1] = slot;
    cpu.r[5] = 0; // mov #0,r5
    cpu.r[6] = len;
    cpu.r[0] = memset;
    let _ = mem.write_u32(cpu.r[1], object); // delay slot: mov.l r14,@r1
    cpu.pr = 0x8c0dfc2a;
    crate::registry::p_8c010f88::run(cpu, mem); // jsr @r0

    cpu.r[0] = len;
    cpu.r[1] = 0; // mov #0,r1
    let _ = mem.write_u32(object.wrapping_add(cpu.r[0]), cpu.r[1]); // mov.l r1,@(r0,r14)
    cpu.r[0] = flag_off;
    cpu.r[1] = 1; // mov #1,r1
    let _ = mem.write_u8(object.wrapping_add(cpu.r[0]), cpu.r[1] as u8); // mov.b r1,@(r0,r14)
    cpu.r[0] = object; // mov r14,r0
}
