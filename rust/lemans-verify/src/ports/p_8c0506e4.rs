//! 0x8c0506e4 — store the two ground-plane projection scalars from 0x8c050cd0.
//!
//! ABI: r4 = object. The routine calls 0x8c050cd0 with `r4` = object,
//! `r5` = its own r5 (a 3-float vector pointer) and `r6` = the 4-byte stack
//! local; the callee writes its plane length/height result to `*r6` and
//! returns `sqrt(len^2 - v^2)` (or 0 when it clamps / the delta is degenerate)
//! in fr0. Both halves are then stored into the object:
//!
//! ```text
//! *(f32 *)(r4+0x3c) = fr0                     ; callee's return value
//! *(f32 *)(r4+0x38) = *(f32 *)local / *(f32 *)(*(r4+8)+0x2c)
//! ```
//!
//! The second numerator is the callee's `*r6` output, not its fr0, so the
//! clamped/over arms divide the plain length instead of the zeroed return.
//! The divisor lives at `base+0x2c` with `base = *(r4+8)`; no guard, so a zero
//! there stores an infinity/NaN exactly as the hardware does.
//!
//! ```text
//! mov.l r14,@-r15 ; sts.l pr,@-r15 ; add #-4,r15 ; mov r4,r14
//! mov.l @(0x8c050714),r0 ; jsr @r0 ; delay: mov r15,r6
//! mov r14,r0 ; add #60,r0 ; fmov fr0,@r0
//! mov #0,r2 ; mov.l @(8,r14),r0 ; add r15,r2 ; fmov @r2,fr1
//! add #44,r0 ; fmov @r0,fr0 ; add #56,r14 ; fdiv fr0,fr1
//! fmov fr1,@r14 ; add #4,r15 ; lds.l @r15+,pr ; rts ; delay: mov.l @r15+,r14
//! ```
//!
//! Register residue: r0 = `base+0x2c` (the divisor address), r1 = `base+0x14`,
//! r2 = the stack local address, r3 = `base+0x18`, r4..r7 and fr2..fr4 are the
//! callee's residue, fr0 = the divisor, fr1 = the stored quotient. r14 is
//! restored by the `rts` delay slot (the `add #56,r14` value is transient), and
//! r8..r13, r15, pr and the other FPU registers keep their entry values.

use sh4_oracle::{Cpu, Memory};

/// Entry of the callee, from the literal pool word at 0x8c050714.
const CALLEE: u32 = 0x8c050cd0;

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[15] = cpu.r[15].wrapping_sub(4); // mov.l r14,@-r15
    let _ = mem.write_u32(cpu.r[15], cpu.r[14]);
    cpu.r[15] = cpu.r[15].wrapping_sub(4); // sts.l pr,@-r15
    let _ = mem.write_u32(cpu.r[15], cpu.pr);
    cpu.r[15] = cpu.r[15].wrapping_sub(4); // add #-4,r15
    cpu.r[14] = cpu.r[4]; // mov r4,r14
    cpu.r[0] = mem.read_u32(0x8c050714).unwrap_or(CALLEE); // mov.l @(0x8c050714),r0
    cpu.pr = 0x8c0506f2; // jsr @r0
    cpu.r[6] = cpu.r[15]; // delay slot: mov r15,r6 — result lands in the local
    crate::registry::p_8c050cd0::run(cpu, mem);

    cpu.r[0] = cpu.r[14]; // mov r14,r0
    cpu.r[0] = cpu.r[0].wrapping_add(60); // add #60,r0
    let _ = mem.write_f32(cpu.r[0], cpu.fr[0]); // fmov fr0,@r0 — +0x3c
    cpu.r[2] = 0; // mov #0,r2
    cpu.r[0] = mem.read_u32(cpu.r[14].wrapping_add(8)).unwrap_or(0); // mov.l @(8,r14),r0
    cpu.r[2] = cpu.r[2].wrapping_add(cpu.r[15]); // add r15,r2
    cpu.fr[1] = mem.read_f32(cpu.r[2]).unwrap_or(0.0); // fmov @r2,fr1 — callee's *r6
    cpu.r[0] = cpu.r[0].wrapping_add(44); // add #44,r0
    cpu.fr[0] = mem.read_f32(cpu.r[0]).unwrap_or(0.0); // fmov @r0,fr0
    cpu.r[14] = cpu.r[14].wrapping_add(56); // add #56,r14
    cpu.fr[1] = cpu.fr[1] / cpu.fr[0]; // fdiv fr0,fr1
    let _ = mem.write_f32(cpu.r[14], cpu.fr[1]); // fmov fr1,@r14 — +0x38
    cpu.r[15] = cpu.r[15].wrapping_add(4); // add #4,r15
    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0); // lds.l @r15+,pr
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[14] = mem.read_u32(cpu.r[15]).unwrap_or(0); // delay slot of `rts`: mov.l @r15+,r14
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}
