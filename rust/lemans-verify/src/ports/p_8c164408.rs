//! 0x8c164408 — store_indexed_at_32.
//!
//! `shll2 r6 ; r0=r6 ; add #32,r4 ; rts ; [r4+r0]=r5` — `r6` is scaled in
//! place and `r4` advanced before the indexed store.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[6] = cpu.r[6] << 2;
    cpu.r[0] = cpu.r[6];
    cpu.r[4] = cpu.r[4].wrapping_add(32);
    let _ = mem.write_u32(cpu.r[4].wrapping_add(cpu.r[0]), cpu.r[5]); // delay slot of `rts`
}
