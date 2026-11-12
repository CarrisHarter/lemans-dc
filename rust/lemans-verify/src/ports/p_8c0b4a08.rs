//! 0x8c0b4a08 — `dst = M·(s·v) + t`, the whole chain in one call:
//!
//! 1. 0x8c0b4e30 scales the three floats `v = [r5..r5+8]` by `s = [r4+32]`
//!    into the scratch at `r15-32` (first stack argument).
//! 2. 0x8c0b3d84 multiplies the matrix `[r4+4..r4+16]` by that vector into
//!    the next 12-byte scratch at `r15-20`.
//! 3. 0x8c0b4f94 adds the vector `t = [r4+20..r4+28]` to it and stores the
//!    result to the pointer passed as the caller's first stack argument
//!    (`[r15]` at entry), read back as `@(32,r15)` after the scratch pops.
//!
//! `r14` and `pr` are pushed and restored; the `mov.l @r15+,r14` is the `rts`
//! delay slot, so `r14` returns to its caller value and `r15` to its entry.
//!
//! Leftover registers: r0=dst+4, r1=t+4, r3=scratch2+4, r4=scratch2,
//! r5=t, r6=t+8, r7=scratch2+8; fr0..fr3 from the vector add, fr4..fr11 from
//! the matrix product, fr12..fr15 untouched.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    // 0x8c0b4a08: mov.l r14,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[14]);
    // 0x8c0b4a0a: sts.l pr,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr);
    // 0x8c0b4a0c: add #-24,r15
    cpu.r[15] = cpu.r[15].wrapping_sub(24);
    // 0x8c0b4a0e: mov r4,r14
    cpu.r[14] = cpu.r[4];
    // 0x8c0b4a10: mov r15,r0
    cpu.r[0] = cpu.r[15];
    // 0x8c0b4a12: mov.l r0,@-r15  (stack argument: first scratch dst)
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[0]);
    // 0x8c0b4a14: mov r14,r0
    cpu.r[0] = cpu.r[14];
    // 0x8c0b4a16: add #32,r0
    cpu.r[0] = cpu.r[0].wrapping_add(32);
    // 0x8c0b4a18: fmov @r0,fr4
    cpu.fr[4] = mem.read_f32(cpu.r[0]).unwrap_or(0.0);
    // 0x8c0b4a1a: mov.l 0x8c0b4a50,r0
    cpu.r[0] = mem.read_u32(0x8c0b4a50).unwrap_or(0); // 0x8c0b4e30
    // 0x8c0b4a1c: jsr @r0 ; 0x8c0b4a1e: mov r5,r4 (delay slot)
    cpu.pr = 0x8c0b4a20;
    cpu.r[4] = cpu.r[5];
    crate::registry::p_8c0b4e30::run(cpu, mem);
    // 0x8c0b4a20: add #4,r15
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    // 0x8c0b4a22: mov r15,r0
    cpu.r[0] = cpu.r[15];
    // 0x8c0b4a24: add #12,r0
    cpu.r[0] = cpu.r[0].wrapping_add(12);
    // 0x8c0b4a26: mov.l r0,@-r15  (stack argument: second scratch dst)
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[0]);
    // 0x8c0b4a28: mov r14,r4
    cpu.r[4] = cpu.r[14];
    // 0x8c0b4a2a: mov r15,r5
    cpu.r[5] = cpu.r[15];
    // 0x8c0b4a2c: mov.l 0x8c0b4a54,r0
    cpu.r[0] = mem.read_u32(0x8c0b4a54).unwrap_or(0); // 0x8c0b3d84
    // 0x8c0b4a2e: add #4,r5
    cpu.r[5] = cpu.r[5].wrapping_add(4);
    // 0x8c0b4a30: jsr @r0 ; 0x8c0b4a32: add #4,r4 (delay slot)
    cpu.pr = 0x8c0b4a34;
    cpu.r[4] = cpu.r[4].wrapping_add(4);
    crate::registry::p_8c0b3d84::run(cpu, mem);
    // 0x8c0b4a34: add #4,r15
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    // 0x8c0b4a36: mov.l @(32,r15),r0  (caller's stack argument)
    cpu.r[0] = mem.read_u32(cpu.r[15].wrapping_add(32)).unwrap_or(0);
    // 0x8c0b4a38: mov.l r0,@-r15  (stack argument: final dst)
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[0]);
    // 0x8c0b4a3a: add #20,r14
    cpu.r[14] = cpu.r[14].wrapping_add(20);
    // 0x8c0b4a3c: mov r15,r4
    cpu.r[4] = cpu.r[15];
    // 0x8c0b4a3e: mov.l 0x8c0b4a58,r0
    cpu.r[0] = mem.read_u32(0x8c0b4a58).unwrap_or(0); // 0x8c0b4f94
    // 0x8c0b4a40: mov r14,r5
    cpu.r[5] = cpu.r[14];
    // 0x8c0b4a42: jsr @r0 ; 0x8c0b4a44: add #16,r4 (delay slot)
    cpu.pr = 0x8c0b4a46;
    cpu.r[4] = cpu.r[4].wrapping_add(16);
    crate::registry::p_8c0b4f94::run(cpu, mem);
    // 0x8c0b4a46: add #4,r15
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    // 0x8c0b4a48: add #24,r15
    cpu.r[15] = cpu.r[15].wrapping_add(24);
    // 0x8c0b4a4a: lds.l @r15+,pr
    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    // 0x8c0b4a4c: rts ; 0x8c0b4a4e: mov.l @r15+,r14 (delay slot)
    cpu.r[14] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}
