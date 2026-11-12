//! 0x8c0b4b04 — compose the transform at `r4` with the one at `r5`:
//! `r4 = r5 ∘ r4`. The record is `{flag: u8, quat: 4×f32 @+4,
//! offset: 3×f32 @+20, scale: f32 @+32}`; the result is written in place at
//! `r4` after the source fields have been read:
//!
//! 1. 0x8c0b4000 — `quat = q(r5+4) · q(r4+4)` (Hamilton product, 16 bytes)
//!    into the scratch at `sp+36`, then copied to `r4+4..r4+16`.
//! 2. 0x8c0b3d84 — `offset = M(q(r5+4)) · offset(r4+20)` into `sp`.
//! 3. 0x8c0b4e0c — scale that vector by `scale(r5+32)` into `sp+12`.
//! 4. 0x8c0b4f94 — add `offset(r5+20)` into `sp+24`, then copy to
//!    `r4+20..r4+28`.
//!
//! Finally `r4+32 = r4+32 * r5+32` and the flag byte `r4+0` is cleared.
//!
//! `r14`, `r13` and `pr` are pushed and restored, so they return to their
//! caller values and `r15` returns to its entry value (the `mov.l @r15+,r14`
//! is the `rts` delay slot).
//!
//! Leftover registers: r0=0, r1=r4+32, r2=sp-24 (from the first callee),
//! r3=scratch4+8, r4=scratch4, r5=r4+20, r6=r5_source+28, r7=scratch3+8;
//! fr0=r5_source+32, fr1=product at r4+32, fr2/fr3 from the vector add,
//! fr4=r5_source+32, fr5..fr11 from the matrix/vector product,
//! fr12..fr15 untouched.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    // 0x8c0b4b04: mov.l r14,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[14]);
    // 0x8c0b4b06: mov.l r13,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[13]);
    // 0x8c0b4b08: mov r5,r14
    cpu.r[14] = cpu.r[5];
    // 0x8c0b4b0a: sts.l pr,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr);
    // 0x8c0b4b0c: add #-52,r15
    cpu.r[15] = cpu.r[15].wrapping_sub(52);
    // 0x8c0b4b0e: mov r4,r13
    cpu.r[13] = cpu.r[4];
    // 0x8c0b4b10: mov r15,r0
    cpu.r[0] = cpu.r[15];
    // 0x8c0b4b12: add #36,r0
    cpu.r[0] = cpu.r[0].wrapping_add(36);
    // 0x8c0b4b14: mov.l r0,@-r15  (stack argument: quad destination)
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[0]);
    // 0x8c0b4b16: mov.l 0x8c0b4ba8,r0
    cpu.r[0] = mem.read_u32(0x8c0b4ba8).unwrap_or(0); // 0x8c0b4000
    // 0x8c0b4b18: mov r13,r5
    cpu.r[5] = cpu.r[13];
    // 0x8c0b4b1a: mov r14,r4
    cpu.r[4] = cpu.r[14];
    // 0x8c0b4b1c: add #4,r5
    cpu.r[5] = cpu.r[5].wrapping_add(4);
    // 0x8c0b4b1e: jsr @r0 ; 0x8c0b4b20: add #4,r4 (delay slot)
    cpu.pr = 0x8c0b4b22;
    cpu.r[4] = cpu.r[4].wrapping_add(4);
    crate::registry::p_8c0b4000::run(cpu, mem);
    // 0x8c0b4b22: add #4,r15
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    // 0x8c0b4b24: mov r15,r4
    cpu.r[4] = cpu.r[15];
    // 0x8c0b4b26: add #36,r4
    cpu.r[4] = cpu.r[4].wrapping_add(36);
    // 0x8c0b4b28: mov r13,r5
    cpu.r[5] = cpu.r[13];
    // 0x8c0b4b2a: mov.l @r4,r3
    cpu.r[3] = mem.read_u32(cpu.r[4]).unwrap_or(0);
    // 0x8c0b4b2c: add #4,r5
    cpu.r[5] = cpu.r[5].wrapping_add(4);
    // 0x8c0b4b2e: mov.l @(4,r4),r1
    cpu.r[1] = mem.read_u32(cpu.r[4].wrapping_add(4)).unwrap_or(0);
    // 0x8c0b4b30: mov r15,r0
    cpu.r[0] = cpu.r[15];
    // 0x8c0b4b32: mov.l r3,@r5
    let _ = mem.write_u32(cpu.r[5], cpu.r[3]);
    // 0x8c0b4b34: mov.l r1,@(4,r5)
    let _ = mem.write_u32(cpu.r[5].wrapping_add(4), cpu.r[1]);
    // 0x8c0b4b36: mov.l @(8,r4),r3
    cpu.r[3] = mem.read_u32(cpu.r[4].wrapping_add(8)).unwrap_or(0);
    // 0x8c0b4b38: mov.l @(12,r4),r1
    cpu.r[1] = mem.read_u32(cpu.r[4].wrapping_add(12)).unwrap_or(0);
    // 0x8c0b4b3a: mov.l r3,@(8,r5)
    let _ = mem.write_u32(cpu.r[5].wrapping_add(8), cpu.r[3]);
    // 0x8c0b4b3c: mov.l r1,@(12,r5)
    let _ = mem.write_u32(cpu.r[5].wrapping_add(12), cpu.r[1]);
    // 0x8c0b4b3e: mov.l r0,@-r15  (stack argument: rotated-offset destination)
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[0]);
    // 0x8c0b4b40: mov r13,r5
    cpu.r[5] = cpu.r[13];
    // 0x8c0b4b42: mov r14,r4
    cpu.r[4] = cpu.r[14];
    // 0x8c0b4b44: mov.l 0x8c0b4bac,r0
    cpu.r[0] = mem.read_u32(0x8c0b4bac).unwrap_or(0); // 0x8c0b3d84
    // 0x8c0b4b46: add #20,r5
    cpu.r[5] = cpu.r[5].wrapping_add(20);
    // 0x8c0b4b48: jsr @r0 ; 0x8c0b4b4a: add #4,r4 (delay slot)
    cpu.pr = 0x8c0b4b4c;
    cpu.r[4] = cpu.r[4].wrapping_add(4);
    crate::registry::p_8c0b3d84::run(cpu, mem);
    // 0x8c0b4b4c: add #4,r15
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    // 0x8c0b4b4e: mov r15,r0
    cpu.r[0] = cpu.r[15];
    // 0x8c0b4b50: add #12,r0
    cpu.r[0] = cpu.r[0].wrapping_add(12);
    // 0x8c0b4b52: mov.l r0,@-r15  (stack argument: scaled destination)
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[0]);
    // 0x8c0b4b54: mov r14,r0
    cpu.r[0] = cpu.r[14];
    // 0x8c0b4b56: add #32,r0
    cpu.r[0] = cpu.r[0].wrapping_add(32);
    // 0x8c0b4b58: fmov @r0,fr4  (scale of the parent transform)
    cpu.fr[4] = mem.read_f32(cpu.r[0]).unwrap_or(0.0);
    // 0x8c0b4b5a: mov r15,r4
    cpu.r[4] = cpu.r[15];
    // 0x8c0b4b5c: mov.l 0x8c0b4bb0,r0
    cpu.r[0] = mem.read_u32(0x8c0b4bb0).unwrap_or(0); // 0x8c0b4e0c
    // 0x8c0b4b5e: jsr @r0 ; 0x8c0b4b60: add #4,r4 (delay slot)
    cpu.pr = 0x8c0b4b62;
    cpu.r[4] = cpu.r[4].wrapping_add(4);
    crate::registry::p_8c0b4e0c::run(cpu, mem);
    // 0x8c0b4b62: add #4,r15
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    // 0x8c0b4b64: mov r15,r0
    cpu.r[0] = cpu.r[15];
    // 0x8c0b4b66: add #24,r0
    cpu.r[0] = cpu.r[0].wrapping_add(24);
    // 0x8c0b4b68: mov.l r0,@-r15  (stack argument: translated destination)
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[0]);
    // 0x8c0b4b6a: mov r14,r5
    cpu.r[5] = cpu.r[14];
    // 0x8c0b4b6c: mov r15,r4
    cpu.r[4] = cpu.r[15];
    // 0x8c0b4b6e: mov.l 0x8c0b4bb4,r0
    cpu.r[0] = mem.read_u32(0x8c0b4bb4).unwrap_or(0); // 0x8c0b4f94
    // 0x8c0b4b70: add #20,r5
    cpu.r[5] = cpu.r[5].wrapping_add(20);
    // 0x8c0b4b72: jsr @r0 ; 0x8c0b4b74: add #16,r4 (delay slot)
    cpu.pr = 0x8c0b4b76;
    cpu.r[4] = cpu.r[4].wrapping_add(16);
    crate::registry::p_8c0b4f94::run(cpu, mem);
    // 0x8c0b4b76: add #4,r15
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    // 0x8c0b4b78: mov r15,r4
    cpu.r[4] = cpu.r[15];
    // 0x8c0b4b7a: add #24,r4
    cpu.r[4] = cpu.r[4].wrapping_add(24);
    // 0x8c0b4b7c: mov r13,r5
    cpu.r[5] = cpu.r[13];
    // 0x8c0b4b7e: mov.l @r4,r0
    cpu.r[0] = mem.read_u32(cpu.r[4]).unwrap_or(0);
    // 0x8c0b4b80: add #20,r5
    cpu.r[5] = cpu.r[5].wrapping_add(20);
    // 0x8c0b4b82: mov.l @(4,r4),r3
    cpu.r[3] = mem.read_u32(cpu.r[4].wrapping_add(4)).unwrap_or(0);
    // 0x8c0b4b84: mov r13,r1
    cpu.r[1] = cpu.r[13];
    // 0x8c0b4b86: add #32,r1
    cpu.r[1] = cpu.r[1].wrapping_add(32);
    // 0x8c0b4b88: mov.l r0,@r5
    let _ = mem.write_u32(cpu.r[5], cpu.r[0]);
    // 0x8c0b4b8a: add #32,r14
    cpu.r[14] = cpu.r[14].wrapping_add(32);
    // 0x8c0b4b8c: mov.l r3,@(4,r5)
    let _ = mem.write_u32(cpu.r[5].wrapping_add(4), cpu.r[3]);
    // 0x8c0b4b8e: mov #0,r0
    cpu.r[0] = 0;
    // 0x8c0b4b90: mov.l @(8,r4),r3
    cpu.r[3] = mem.read_u32(cpu.r[4].wrapping_add(8)).unwrap_or(0);
    // 0x8c0b4b92: mov.l r3,@(8,r5)
    let _ = mem.write_u32(cpu.r[5].wrapping_add(8), cpu.r[3]);
    // 0x8c0b4b94: fmov @r1,fr1
    cpu.fr[1] = mem.read_f32(cpu.r[1]).unwrap_or(0.0);
    // 0x8c0b4b96: fmov @r14,fr0
    cpu.fr[0] = mem.read_f32(cpu.r[14]).unwrap_or(0.0);
    // 0x8c0b4b98: fmul fr0,fr1
    cpu.fr[1] = cpu.fr[1] * cpu.fr[0];
    // 0x8c0b4b9a: fmov fr1,@r1
    let _ = mem.write_f32(cpu.r[1], cpu.fr[1]);
    // 0x8c0b4b9c: mov.b r0,@r13
    let _ = mem.write_u8(cpu.r[13], cpu.r[0] as u8);
    // 0x8c0b4b9e: add #52,r15
    cpu.r[15] = cpu.r[15].wrapping_add(52);
    // 0x8c0b4ba0: lds.l @r15+,pr
    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    // 0x8c0b4ba2: mov.l @r15+,r13
    cpu.r[13] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    // 0x8c0b4ba4: rts ; 0x8c0b4ba6: mov.l @r15+,r14 (delay slot)
    cpu.r[14] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}
