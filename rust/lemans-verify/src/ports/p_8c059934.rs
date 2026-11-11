//! 0x8c059934 — `*(char *)(*(int *)(param_1 + 4) + 5) = 0;`
//!
//! `mov.l @(4,r4),r1 ; mov #0,r0 ; rts ; mov.b r0,@(5,r1)` — the C hides that
//! r1 keeps the loaded inner pointer, r0 is zeroed, and the store is the
//! `rts` delay slot.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[1] = mem.read_u32(cpu.r[4].wrapping_add(4)).unwrap(); // mov.l @(4,r4),r1
    cpu.r[0] = 0; // mov #0,r0
    let _ = mem.write_u8(cpu.r[1].wrapping_add(5), cpu.r[0] as u8); // delay slot of `rts`
}
