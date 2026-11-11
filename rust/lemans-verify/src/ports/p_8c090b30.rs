//! 0x8c090b30 — `p[1] = p[2] = *p;`
//!
//! `mov.l @r4,r0 ; mov.l r0,@(4,r4) ; rts ; mov.l r0,@(8,r4)`
//! The store to p[2] lives in the `rts` delay slot, so both stores happen.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[0] = mem.read_u32(cpu.r[4]).unwrap_or(0); // mov.l @r4,r0
    let _ = mem.write_u32(cpu.r[4] + 4, cpu.r[0]); // mov.l r0,@(4,r4)
    let _ = mem.write_u32(cpu.r[4] + 8, cpu.r[0]); // rts delay slot: mov.l r0,@(8,r4)
}
