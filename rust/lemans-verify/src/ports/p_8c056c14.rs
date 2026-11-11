//! 0x8c056c14 — return *DAT_0x8c23dd50.
//!
//! ```text
//! mov.l 0x8c056c1c,r0   ; r0 = 0x8c23dd50 (literal pool)
//! rts ; mov.l @r0,r0    ; delay slot does the load
//! ```
//! The load is in the `rts` delay slot, so r0 ends as [0x8c23dd50], not the
//! pool pointer. Verified against fixtures/lemans24-ext.bin (the global at
//! 0x8c23dd50 lies past the end of analysis/lemans24.bin).

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let g = mem.read_u32(0x8c056c1c).unwrap_or(0); // 0x8c23dd50
    cpu.r[0] = mem.read_u32(g).unwrap_or(0); // delay slot of `rts`: mov.l @r0,r0
}
