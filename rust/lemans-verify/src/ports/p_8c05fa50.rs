//! 0x8c05fa50 — `mov.l 0x8c05fa58,r0 ; rts ; mov.l @r0,r0`.
//!
//! The literal at 0x8c05fa58 is 0x8c23de98, so the function returns
//! mem32[mem32[0x8c05fa58]] — the actual load is the rts delay slot, and the
//! intermediate pointer never appears in the C beyond `*DAT_8c05fa58`.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let ptr = mem.read_u32(0x8c05fa58).unwrap_or(0); // mov.l 0x8c05fa58,r0
    cpu.r[0] = ptr;
    cpu.r[0] = mem.read_u32(cpu.r[0]).unwrap_or(0); // rts delay slot: mov.l @r0,r0
}
