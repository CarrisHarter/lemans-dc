//! 0x8c0174c0 — `old = *g_slot; *g_slot = r4; return old;`
//!
//! The slot pointer is read from the literal cell 0x8c0174d0 at runtime
//! (0x8c23da50 in the ROM, redirected by the spec), so the port loads it
//! from memory like the original. Hidden side effects: r1 keeps the slot
//! pointer and r0 keeps the old value; r14 is saved/restored around the
//! prologue, T is never touched.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let r1 = mem.read_u32(0x8c0174d0).unwrap_or(0);
    let r0 = mem.read_u32(r1).unwrap_or(0);
    let _ = mem.write_u32(r1, cpu.r[4]);
    cpu.r[0] = r0;
    cpu.r[1] = r1;
}
