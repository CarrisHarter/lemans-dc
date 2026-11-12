//! 0x8c0b4990 — scale a three-float vector by `[r4+32]`, then transform it by
//! the 3x3 matrix at `r4+4` and store the three results at the caller's out
//! pointer.
//!
//! The prologue saves r14 and pr, reserves 12 bytes of scratch and pushes the
//! address of that scratch. 0x8c0b4e30 then scales the three floats at r5 by
//! fr4 (loaded here from `[r4+32]`) into the scratch; 0x8c0b3d84 takes the
//! matrix at `r4+4`, the scratch as its vector and the out pointer from the
//! stack argument — the word at the entry r15, re-read as `@(20,r15)` once the
//! frame is built — and writes `out[0..3]`. Both calls are a literal-pool load
//! plus `jsr` with a register-move delay slot; pr becomes 0x8c0b49a8 and
//! 0x8c0b49ba, and the epilogue restores it together with r14.
//!
//! Leftovers come from the matrix callee: r0=out+4, r1=temp+8, r3=r4+8,
//! r4=r4+16, r5=temp+4, r6=out+8; r2, r7-r13 and fr12-fr15 are untouched and
//! r15 is back at its entry value.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    // mov.l r14,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[14]);
    // sts.l pr,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr);
    // add #-12,r15
    cpu.r[15] = cpu.r[15].wrapping_sub(12);
    // mov r4,r14
    cpu.r[14] = cpu.r[4];
    // mov r15,r0 ; mov.l r0,@-r15 — scratch pointer for the scaling callee
    cpu.r[0] = cpu.r[15];
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[0]);
    // mov r14,r0 ; add #32,r0 ; fmov @r0,fr4
    cpu.r[0] = cpu.r[14];
    cpu.r[0] = cpu.r[0].wrapping_add(32);
    cpu.fr[4] = mem.read_f32(cpu.r[0]).unwrap_or(0.0);
    // mov.l @(8,pc),r0 (0x8c0b49c4 -> 0x8c0b4e30) ; jsr @r0
    cpu.r[0] = mem.read_u32(0x8c0b49c4).unwrap_or(0);
    cpu.pr = 0x8c0b49a8;
    cpu.r[4] = cpu.r[5]; // delay slot: mov r5,r4
    crate::registry::p_8c0b4e30::run(cpu, mem);
    // add #4,r15
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    // mov.l @(20,r15),r0 — the caller's out pointer (word at the entry r15)
    cpu.r[0] = mem.read_u32(cpu.r[15].wrapping_add(20)).unwrap_or(0);
    // mov.l r0,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[0]);
    // add #4,r14
    cpu.r[14] = cpu.r[14].wrapping_add(4);
    // mov r15,r5
    cpu.r[5] = cpu.r[15];
    // mov.l @(5,pc),r0 (0x8c0b49c8 -> 0x8c0b3d84)
    cpu.r[0] = mem.read_u32(0x8c0b49c8).unwrap_or(0);
    // mov r14,r4
    cpu.r[4] = cpu.r[14];
    // jsr @r0
    cpu.pr = 0x8c0b49ba;
    cpu.r[5] = cpu.r[5].wrapping_add(4); // delay slot: add #4,r5
    crate::registry::p_8c0b3d84::run(cpu, mem);
    // add #4,r15 ; add #12,r15
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[15] = cpu.r[15].wrapping_add(12);
    // lds.l @r15+,pr
    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    // rts
    // delay slot: mov.l @r15+,r14
    cpu.r[14] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}
