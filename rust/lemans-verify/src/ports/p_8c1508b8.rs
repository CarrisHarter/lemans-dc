//! 0x8c1508b8 — zero the 12-byte header at r4 and register it: `*0x8c2abc88 = r4`.
//!
//! Original:
//! ```text
//! mov #0,r3 ; mov.l r3,@r4 ; mov r4,r0 ; mov.l 0x8c1508c8,r1
//! mov.l r3,@(4,r4) ; mov.l r3,@(8,r4) ; rts ; mov.l r4,@r1
//! ```
//! The global store is the rts delay slot; r1 = 0x8c2abc88 from the literal pool.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let r4 = cpu.r[4];
    let g = mem.read_u32(0x8c1508c8).unwrap_or(0); // 0x8c2abc88
    cpu.r[3] = 0;
    let _ = mem.write_u32(r4, cpu.r[3]);
    cpu.r[0] = r4;
    cpu.r[1] = g;
    let _ = mem.write_u32(r4 + 4, cpu.r[3]);
    let _ = mem.write_u32(r4 + 8, cpu.r[3]);
    let _ = mem.write_u32(g, r4); // rts delay slot: mov.l r4,@r1
}
