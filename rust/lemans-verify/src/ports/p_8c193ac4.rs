//! 0x8c193ac4 — normalized cross-product / plane math in single precision.
//!
//! Loads fr10 = f32[r6], fr9 = f32[r6+4], fr11 = f32[r5], fr6 = f32[r5+4],
//! fr3 = f32[r5+8], and the literals -2.0 (0x8c193b48), 4.0 (0x8c193b4c),
//! 2.0 (0x8c193b50) into fr0/fr2/fr1. Computes
//! `d = fr10*fr10`, `n = ((fr9*-2.0)*fr11)/d`, `t = ((fr9*fr6)/fr10)`,
//! `s = ((fr9*fr9)/d - (fr9*fr6)/fr10 + fr3) * (n*4.0)`, `q = (n*... )` etc.,
//! then stores `fr0 = (q + sqrt(t*t - s)) / (n*2.0)` at r4+4,
//! `fr7 = (q - sqrt(...)) / (n*2.0)` at r4+12, and finally
//! `(stored - fr9)/fr10` at r4+0 and r4+8 (the last in the `rts` delay slot).
//!
//! All fmov.s are plain single-precision (no SZ double-width). Exit:
//! r0 = r4, r1 = r4+8, r3 = r4+12, r5 = r5_entry+8, r6 = r4+4; T untouched.
//!
//! NaN propagation: same rule as 0x8c06f6cc — the oracle keeps the first (the
//! instruction's destination) operand's NaN payload, falling back to the
//! source. Every arithmetic step is issued through the helpers in the exact
//! `FRn op FRm` order of the original; raw Rust `a op b` lets LLVM commute
//! the operands, which changed the surviving NaN payload here.

use sh4_oracle::{Cpu, Memory};

/// Quiet a NaN the way the hardware does when it propagates the payload.
fn quiet(v: f32) -> f32 {
    f32::from_bits(v.to_bits() | 0x0040_0000)
}

/// NaN-preserving FP ops: first (destination) NaN operand wins, else the second.
fn fadd(a: f32, b: f32) -> f32 {
    if a.is_nan() {
        quiet(a)
    } else if b.is_nan() {
        quiet(b)
    } else {
        a + b
    }
}

fn fsub(a: f32, b: f32) -> f32 {
    if a.is_nan() {
        quiet(a)
    } else if b.is_nan() {
        quiet(b)
    } else {
        a - b
    }
}

fn fmul(a: f32, b: f32) -> f32 {
    if a.is_nan() {
        quiet(a)
    } else if b.is_nan() {
        quiet(b)
    } else {
        a * b
    }
}

fn fdiv(a: f32, b: f32) -> f32 {
    if a.is_nan() {
        quiet(a)
    } else if b.is_nan() {
        quiet(b)
    } else {
        a / b
    }
}

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let r4 = cpu.r[4];
    let r5e = cpu.r[5];
    let r6e = cpu.r[6];

    let fr10 = mem.read_f32(r6e).unwrap_or(0.0); // fmov @r6,fr10
    let fr9 = mem.read_f32(r6e.wrapping_add(4)).unwrap_or(0.0); // fmov @r6,fr9 (r6+=4 first)
    let fr11in = mem.read_f32(r5e).unwrap_or(0.0); // fmov @r5,fr11
    let fr6in = mem.read_f32(r5e.wrapping_add(4)).unwrap_or(0.0); // fmov @r1,fr6 (r1 = r5+4)
    let fr0c = mem.read_f32(0x8c193b48).unwrap_or(0.0); // mova 0x8c193b48 -> -2.0
    let fr2 = mem.read_f32(0x8c193b4c).unwrap_or(0.0); // mova 0x8c193b4c -> 4.0
    let fr1c = mem.read_f32(0x8c193b50).unwrap_or(0.0); // mova 0x8c193b50 -> 2.0

    let fr8 = fmul(fr10, fr10); // fmov fr10,fr8; fmul fr10,fr8
    let mut fr7 = fmul(fr9, fr0c); // fmov fr9,fr7; fmul fr0,fr7
    fr7 = fmul(fr7, fr11in); // fmul fr11,fr7
    let mut fr11 = fdiv(fr11in, fr8); // fdiv fr8,fr11
    let fr0n = fr11; // fmov fr11,fr0
    let mut fr4 = fmul(fr9, fr6in); // fmov fr9,fr4; fmul fr6,fr4
    let mut fr5 = fmul(fr9, fr9); // fmov fr9,fr5; fmul fr9,fr5
    fr7 = fdiv(fr7, fr8); // fdiv fr8,fr7
    let mut fr6 = fdiv(fr6in, fr10); // fdiv fr10,fr6
    fr7 = fadd(fr7, fr6); // fadd fr6,fr7
    fr7 = fsub(fr7, 1.0); // fldi1 fr6; fsub fr6,fr7
    fr6 = fmul(fr7, fr7); // fmov fr7,fr6; fmul fr7,fr6
    let fr3 = mem.read_f32(r5e.wrapping_add(8)).unwrap_or(0.0); // fmov @r5,fr3 (r5 += 8)
    fr5 = fdiv(fr5, fr8); // fdiv fr8,fr5
    fr4 = fdiv(fr4, fr10); // fdiv fr10,fr4
    fr5 = fsub(fr5, fr4); // fsub fr4,fr5
    fr5 = fadd(fr5, fr3); // fadd fr3,fr5
    let mut fr0 = fmul(fr0n, fr2); // fmul fr2,fr0
    fr5 = fmul(fr5, fr0); // fmul fr0,fr5
    fr7 = -fr7; // fneg fr7
    fr6 = fsub(fr6, fr5); // fsub fr5,fr6
    fr0 = fr7; // fmov fr7,fr0
    fr6 = fr6.sqrt(); // fsqrt fr6
    fr11 = fmul(fr11, fr1c); // fmul fr1,fr11
    fr0 = fadd(fr0, fr6); // fadd fr6,fr0
    fr7 = fsub(fr7, fr6); // fsub fr6,fr7

    let r6 = r4.wrapping_add(4); // mov r4,r6; add #4,r6
    fr0 = fdiv(fr0, fr11); // fdiv fr11,fr0
    let _ = mem.write_f32(r6, fr0); // fmov fr0,@r6 -> [r4+4]
    let r3 = r4.wrapping_add(12); // mov r4,r3; add #12,r3
    fr7 = fdiv(fr7, fr11); // fdiv fr11,fr7
    let _ = mem.write_f32(r3, fr7); // fmov fr7,@r3 -> [r4+12]
    let r1 = r4.wrapping_add(8); // mov r4,r1; add #8,r1
    let mut fr0b = mem.read_f32(r6).unwrap_or(0.0); // fmov @r6,fr0
    fr0b = fsub(fr0b, fr9); // fsub fr9,fr0
    fr0b = fdiv(fr0b, fr10); // fdiv fr10,fr0
    let _ = mem.write_f32(r4, fr0b); // fmov fr0,@r4 -> [r4+0]
    let mut fr0c2 = mem.read_f32(r3).unwrap_or(0.0); // fmov @r3,fr0
    fr0c2 = fsub(fr0c2, fr9);
    fr0c2 = fdiv(fr0c2, fr10);
    let _ = mem.write_f32(r1, fr0c2); // rts delay slot: fmov fr0,@r1 -> [r4+8]

    cpu.r[0] = r4; // mov r4,r0
    cpu.r[1] = r1;
    cpu.r[3] = r3;
    cpu.r[5] = r5e.wrapping_add(8);
    cpu.r[6] = r6;
    cpu.fr[0] = fr0c2;
    cpu.fr[1] = fr1c;
    cpu.fr[2] = fr2;
    cpu.fr[3] = fr3;
    cpu.fr[4] = fr4;
    cpu.fr[5] = fr5;
    cpu.fr[6] = fr6;
    cpu.fr[7] = fr7;
    cpu.fr[8] = fr8;
    cpu.fr[9] = fr9;
    cpu.fr[10] = fr10;
    cpu.fr[11] = fr11;
}
