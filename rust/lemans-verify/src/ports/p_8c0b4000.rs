//! 0x8c0b4000 — 4-component dot-product style write: with a0..a3=[r4..+12],
//! b0..b3=[r5..+12] and the destination r7 loaded from `@(32,r15)` (the word
//! at entry r15, after 4 fr pushes + add #-16), stores four words to `[r7]`.
//! Intermediate products spill to the stack window (not compared). fr12..15
//! are saved/restored.
//!
//! Leftover registers: r0=r7+8, r1=r7+4, r2=r15-20, r3=r5+12, r4=r4+8,
//! r5=r5+4, r6=r5+8, r7=r7+12.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let a = cpu.r[4];
    let b = cpu.r[5];
    let dst = mem.read_u32(cpu.r[15]).unwrap_or(0);
    let a0 = mem.read_f32(a).unwrap_or(0.0);
    let a1 = mem.read_f32(a + 4).unwrap_or(0.0);
    let a2 = mem.read_f32(a + 8).unwrap_or(0.0);
    let a3 = mem.read_f32(a + 12).unwrap_or(0.0);
    let b0 = mem.read_f32(b).unwrap_or(0.0);
    let b1 = mem.read_f32(b + 4).unwrap_or(0.0);
    let b2 = mem.read_f32(b + 8).unwrap_or(0.0);
    let b3 = mem.read_f32(b + 12).unwrap_or(0.0);

    let d0 = a0 * b0 - a1 * b1 - a2 * b2 - a3 * b3; // fr1
    let d4 = a0 * b1 + a1 * b0 + a2 * b3 - a3 * b2; // fr0
    let d8 = a0 * b2 + a2 * b0 + a3 * b1 - a1 * b3; // fr11
    let d12 = a0 * b3 + a3 * b0 + a1 * b2 - a2 * b1; // fr5

    let _ = mem.write_f32(dst, d0);
    let _ = mem.write_f32(dst + 4, d4);
    let _ = mem.write_f32(dst + 8, d8);
    let _ = mem.write_f32(dst + 12, d12);

    cpu.fr[0] = d4;
    cpu.fr[1] = a3 * b2;
    cpu.fr[2] = a2 * b1;
    cpu.fr[3] = a1 * b2;
    cpu.fr[4] = a3 * b0;
    cpu.fr[5] = d12;
    cpu.fr[6] = b1;
    cpu.fr[7] = a2 * b0;
    cpu.fr[8] = b0;
    cpu.fr[9] = a3 * b1;
    cpu.fr[10] = b3;
    cpu.fr[11] = d8;

    cpu.r[0] = dst + 8;
    cpu.r[1] = dst + 4;
    cpu.r[2] = cpu.r[15].wrapping_sub(20);
    cpu.r[3] = b + 12;
    cpu.r[4] = a + 8;
    cpu.r[5] = b + 4;
    cpu.r[6] = b + 8;
    cpu.r[7] = dst + 12;
}
