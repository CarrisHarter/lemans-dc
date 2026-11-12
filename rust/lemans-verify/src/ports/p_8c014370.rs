//! 0x8c014370 — arccos: π/2 − asin(fr4).
//!
//! ```text
//! sts.l pr,@-r15 ; mov.l 0x8c014384,r3 (0x8c01438c) ; jsr @r3 ; nop
//! mov.l 0x8c014388,r3 (0x8c1ccfc0 = π/2) ; lds.l @r15+,pr
//! fmov fr0,fr3 ; fmov @r3,fr0 ; rts ; fsub fr3,fr0
//! ```
//!
//! Saves `pr` around the 0x8c01438c arcsin call (return address
//! 0x8c014378), then returns π/2 − asin(fr4) in fr0. r3 ends as the literal
//! pointer 0x8c1ccfc0; T is left exactly as the callee left it.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    // 0x8c014370: sts.l pr,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr);
    // 0x8c014372: mov.l 0x8c014384,r3
    cpu.r[3] = mem.read_u32(0x8c014384).unwrap_or(0);
    // 0x8c014374: jsr @r3 ; 0x8c014376: nop (delay slot)
    cpu.pr = 0x8c014378;
    crate::registry::p_8c01438c::run(cpu, mem);
    // 0x8c014378: mov.l 0x8c014388,r3
    cpu.r[3] = mem.read_u32(0x8c014388).unwrap_or(0);
    // 0x8c01437a: lds.l @r15+,pr
    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    // 0x8c01437c: fmov fr0,fr3
    cpu.fr[3] = cpu.fr[0];
    // 0x8c01437e: fmov @r3,fr0
    cpu.fr[0] = mem.read_f32(cpu.r[3]).unwrap_or(0.0);
    // 0x8c014380: rts ; 0x8c014382: fsub fr3,fr0 (delay slot)
    cpu.fr[0] = cpu.fr[0] - cpu.fr[3];
}
