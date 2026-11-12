//! 0x8c0ed79c — initialise one object: vtable, then `16.0 / log10(0.5)` at +4.
//!
//! Stores 0x8c1ec934 and then 0x8c1ec920 through the incoming pointer `r4`
//! (base-class ctor inline, then this class's vtable; the first store is dead),
//! calls 0x8c010d8c with `fr4 = 0.5` (pool word 0x8c0ed7d4) so that the callee
//! leaves `fr0 = log10(0.5)`, and stores `16.0 / fr0` at `p + 4`.
//!
//! Register residue: `r0 = p` (the returned pointer), `r1 = p + 4`, and
//! `r3 = entry r15 - 16` — the callee sets `r3` to its own frame base, which is
//! its entry `r15` minus 8 (one word for PR, one for the locals slot), and this
//! function's entry `r15` is another 8 above the callee's because of its own
//! `sts.l`/`add #-4` prologue. The rest (r2/r4/r5/r6/r7, fr0-fr7, fpul) is the
//! callee's residue, so the port simply calls the ported callee.
//!
//! No branch, so the T bit entering the call is the callee's final T.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr); // sts.l pr,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4); // add #-4,r15
    let _ = mem.write_u32(cpu.r[15], cpu.r[4]); // mov.l r4,@r15
    cpu.r[1] = mem.read_u32(cpu.r[15]).unwrap_or(0); // mov.l @r15,r1
    cpu.r[0] = mem.read_u32(0x8c0ed7cc).unwrap_or(0); // 0x8c1ec934
    cpu.r[3] = mem.read_u32(0x8c0ed7d0).unwrap_or(0); // 0x8c1ec920
    let _ = mem.write_u32(cpu.r[1], cpu.r[0]); // mov.l r0,@r1
    cpu.r[4] = mem.read_u32(cpu.r[15]).unwrap_or(0); // mov.l @r15,r4
    cpu.r[0] = 0x8c0ed7d4; // mova 0x8c0ed7d4,r0
    cpu.r[1] = mem.read_u32(0x8c0ed7d8).unwrap_or(0); // 0x8c010d8c
    let _ = mem.write_u32(cpu.r[4], cpu.r[3]); // mov.l r3,@r4
    cpu.pr = 0x8c0ed7b6; // jsr @r1
    cpu.fr[4] = mem.read_f32(cpu.r[0]).unwrap_or(0.0); // delay slot: fmov @r0,fr4 = 0.5
    crate::registry::p_8c010d8c::run(cpu, mem);
    cpu.r[0] = 0x8c0ed7dc; // mova 0x8c0ed7dc,r0
    cpu.r[1] = mem.read_u32(cpu.r[15]).unwrap_or(0); // mov.l @r15,r1
    cpu.fr[1] = mem.read_f32(cpu.r[0]).unwrap_or(0.0); // fmov @r0,fr1 = 16.0
    cpu.r[1] = cpu.r[1].wrapping_add(4); // add #4,r1
    cpu.fr[1] = cpu.fr[1] / cpu.fr[0]; // fdiv fr0,fr1
    let _ = mem.write_f32(cpu.r[1], cpu.fr[1]); // fmov fr1,@r1
    cpu.r[0] = mem.read_u32(cpu.r[15]).unwrap_or(0); // mov.l @r15,r0
    cpu.r[15] = cpu.r[15].wrapping_add(4); // add #4,r15
    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0); // lds.l @r15+,pr
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}
