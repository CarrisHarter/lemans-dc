//! 0x8c06f6cc — vector/matrix style FPU leaf, 131 instructions, straight-line
//! (no branches at all; Ghidra's PR/SZ split is not real control flow, the
//! oracle always runs the single-precision path).
//!
//! CFG: prologue (push fr15/fr14/fr13, r14..r11) -> one basic block -> epilogue
//! (pop r11..r14, fr13, fr14, rts, delay slot pops fr15). One path only.
//!
//! Reads: floats at r4+0/4/8/12/16/20, a sign-extended u16 index at r4+28, a
//! table pointer from the caller's stack slot [r15_entry], and three table
//! floats at table + idx*12 (+0/+4/+8). Writes three floats each at r5, r6,
//! r7 (base, +4, +8).
//!
//! Hidden side effects the decompiled C does not show:
//! - r0 ends as table + idx*12 (the table walk pointer), r1 = r7+4, r3 = r6+8,
//!   r4 = r6+4, r7 = r7+8; r5/r6 keep their entry values; r11..r14 are restored
//!   by the epilogue pops, so only r0..r4, r7 change.
//! - `shll r0` (8c06f710) sets T from bit 31 of idx; `shll2` does not, so the
//!   final T is just the index's bit 15. No other flag op runs.
//! - fr0..fr12 are clobbered with intermediate results; fr13/fr14/fr15 are
//!   saved/restored through the stack window.
//! - the read of [r15_entry] happens via @(28,r15) after the seven pushes.
//!
//! NaN propagation: the harness's oracle keeps the first NaN operand's raw
//! bits (and only falls back to the second operand's NaN when the first is
//! not NaN). That rule was established by probing the oracle itself, not
//! read from an SH-4 manual; `fadd`/`fsub`/`fmul`/`fdiv` below route through
//! these helpers so the port cannot inherit x86 operand-commutation choices.
//! Keep this port in sync with notes/address-map.md if that rule changes.

use sh4_oracle::{Cpu, Memory};

/// Quiet a NaN the way the hardware does when it propagates the payload.
fn quiet(v: f32) -> f32 {
    f32::from_bits(v.to_bits() | 0x0040_0000)
}

/// NaN-preserving FP ops: first NaN operand wins, else the second.
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
    // prologue: fmov fr15/fr14/fr13,@-r15; mov.l r14/r13/r12/r11,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_f32(cpu.r[15], cpu.fr[15]);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_f32(cpu.r[15], cpu.fr[14]);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_f32(cpu.r[15], cpu.fr[13]);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[14]);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[13]);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[12]);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[11]);

    cpu.fr[7] = mem.read_f32(cpu.r[4]).unwrap(); // fmov @r4,fr7
    cpu.r[11] = cpu.r[4];
    cpu.r[13] = cpu.r[4];
    cpu.r[3] = cpu.r[4];
    cpu.r[1] = cpu.r[4];
    cpu.r[0] = mem.read_u16(cpu.r[4] + 28).unwrap() as i16 as i32 as u32; // mov.w @(28,r4),r0
    cpu.r[11] = cpu.r[11].wrapping_add(16);
    cpu.fr[5] = mem.read_f32(cpu.r[11]).unwrap(); // fmov @r11,fr5  (r4+16)
    cpu.r[4] = cpu.r[4].wrapping_add(20);
    cpu.fr[2] = mem.read_f32(cpu.r[4]).unwrap(); // fmov @r4,fr2   (r4+20)
    cpu.r[3] = cpu.r[3].wrapping_add(8);
    cpu.fr[4] = mem.read_f32(cpu.r[3]).unwrap(); // fmov @r3,fr4   (r4+8)
    cpu.r[1] = cpu.r[1].wrapping_add(12);
    cpu.r[13] = cpu.r[13].wrapping_add(4);
    cpu.fr[3] = mem.read_f32(cpu.r[1]).unwrap(); // fmov @r1,fr3   (r4+12)
    cpu.fr[5] = -cpu.fr[5]; // fneg fr5
    cpu.fr[6] = mem.read_f32(cpu.r[13]).unwrap(); // fmov @r13,fr6 (r4+4)
    cpu.fr[2] = -cpu.fr[2]; // fneg fr2
    cpu.fr[0] = cpu.fr[5];
    cpu.fr[1] = cpu.fr[7];
    cpu.fr[0] = fmul(cpu.fr[0], cpu.fr[3]); // fmul fr3,fr0
    cpu.fr[9] = cpu.fr[4];
    cpu.fr[8] = cpu.fr[2];
    cpu.r[3] = cpu.r[0] & 0xFFFF; // extu.w r0,r3
    cpu.fr[13] = cpu.fr[4];
    cpu.fr[11] = cpu.fr[7];
    cpu.r[0] = cpu.r[3];
    let out = cpu.r[0] >> 31 != 0; // shll r0 (sets T)
    cpu.r[0] = cpu.r[0] << 1;
    cpu.set_t(out);
    cpu.r[0] = cpu.r[0].wrapping_add(cpu.r[3]); // add r3,r0
    cpu.r[11] = cpu.r[5];
    cpu.r[12] = cpu.r[5];
    cpu.r[4] = cpu.r[6];
    cpu.fr[14] = cpu.fr[0];
    cpu.fr[8] = fmul(cpu.fr[8], cpu.fr[6]); // fmul fr6,fr8
    cpu.fr[1] = fmul(cpu.fr[1], cpu.fr[3]); // fmul fr3,fr1
    cpu.fr[9] = fmul(cpu.fr[9], cpu.fr[6]); // fmul fr6,fr9
    cpu.fr[1] = fsub(cpu.fr[1], cpu.fr[9]); // fsub fr9,fr1
    cpu.fr[14] = fsub(cpu.fr[14], cpu.fr[8]); // fsub fr8,fr14
    cpu.r[3] = cpu.r[6];
    cpu.r[13] = mem.read_u32(cpu.r[15] + 28).unwrap(); // mov.l @(28,r15),r13
    cpu.fr[14] = fdiv(cpu.fr[14], cpu.fr[1]); // fdiv fr1,fr14
    cpu.r[0] = cpu.r[0] << 2; // shll2 r0 (no T)
    cpu.r[0] = cpu.r[0].wrapping_add(cpu.r[13]); // add r13,r0
    cpu.r[13] = cpu.r[0];
    cpu.r[14] = cpu.r[0];
    cpu.r[1] = cpu.r[7];
    cpu.fr[13] = fmul(cpu.fr[13], cpu.fr[5]); // fmul fr5,fr13
    cpu.fr[11] = fmul(cpu.fr[11], cpu.fr[2]); // fmul fr2,fr11
    cpu.fr[9] = 1.0; // fldi1 fr9
    cpu.fr[11] = fsub(cpu.fr[11], cpu.fr[13]); // fsub fr13,fr11
    cpu.fr[2] = fadd(cpu.fr[2], cpu.fr[9]); // fadd fr9,fr2
    cpu.fr[10] = cpu.fr[7];
    cpu.fr[10] = fmul(cpu.fr[10], cpu.fr[2]); // fmul fr2,fr10
    cpu.fr[8] = cpu.fr[2];
    cpu.fr[10] = fsub(cpu.fr[10], cpu.fr[13]); // fsub fr13,fr10
    cpu.fr[8] = fmul(cpu.fr[8], cpu.fr[6]); // fmul fr6,fr8
    cpu.fr[0] = fsub(cpu.fr[0], cpu.fr[8]); // fsub fr8,fr0
    cpu.fr[2] = fsub(cpu.fr[2], cpu.fr[9]); // fsub fr9,fr2
    cpu.fr[5] = fadd(cpu.fr[5], cpu.fr[9]); // fadd fr9,fr5
    cpu.fr[8] = cpu.fr[2];
    cpu.fr[9] = cpu.fr[5];
    let _ = mem.write_f32(cpu.r[5], cpu.fr[14]); // fmov fr14,@r5
    cpu.r[11] = cpu.r[11].wrapping_add(4);
    cpu.fr[11] = fdiv(cpu.fr[11], cpu.fr[1]); // fdiv fr1,fr11
    let _ = mem.write_f32(cpu.r[11], cpu.fr[11]); // fmov fr11,@r11
    cpu.r[13] = cpu.r[13].wrapping_add(4);
    cpu.fr[13] = mem.read_f32(cpu.r[0]).unwrap(); // fmov @r0,fr13
    cpu.r[14] = cpu.r[14].wrapping_add(8);
    cpu.fr[11] = mem.read_f32(cpu.r[5]).unwrap(); // fmov @r5,fr11
    cpu.r[12] = cpu.r[12].wrapping_add(8);
    cpu.fr[14] = mem.read_f32(cpu.r[13]).unwrap(); // fmov @r13,fr14
    cpu.fr[13] = fmul(cpu.fr[13], cpu.fr[11]); // fmul fr11,fr13
    cpu.fr[15] = mem.read_f32(cpu.r[11]).unwrap(); // fmov @r11,fr15
    cpu.fr[0] = fdiv(cpu.fr[0], cpu.fr[1]); // fdiv fr1,fr0
    cpu.fr[11] = mem.read_f32(cpu.r[14]).unwrap(); // fmov @r14,fr11
    cpu.fr[14] = fmul(cpu.fr[14], cpu.fr[15]); // fmul fr15,fr14
    cpu.fr[13] = fadd(cpu.fr[13], cpu.fr[14]); // fadd fr14,fr13
    cpu.fr[13] = fadd(cpu.fr[13], cpu.fr[11]); // fadd fr11,fr13
    let _ = mem.write_f32(cpu.r[12], cpu.fr[13]); // fmov fr13,@r12
    cpu.r[4] = cpu.r[4].wrapping_add(4);
    let _ = mem.write_f32(cpu.r[6], cpu.fr[0]); // fmov fr0,@r6
    cpu.fr[10] = fdiv(cpu.fr[10], cpu.fr[1]); // fdiv fr1,fr10
    let _ = mem.write_f32(cpu.r[4], cpu.fr[10]); // fmov fr10,@r4
    cpu.fr[8] = fmul(cpu.fr[8], cpu.fr[6]); // fmul fr6,fr8
    cpu.fr[9] = fmul(cpu.fr[9], cpu.fr[3]); // fmul fr3,fr9
    cpu.fr[7] = fmul(cpu.fr[7], cpu.fr[2]); // fmul fr2,fr7
    cpu.fr[9] = fsub(cpu.fr[9], cpu.fr[8]); // fsub fr8,fr9
    cpu.fr[4] = fmul(cpu.fr[4], cpu.fr[5]); // fmul fr5,fr4
    cpu.fr[7] = fsub(cpu.fr[7], cpu.fr[4]); // fsub fr4,fr7
    cpu.fr[6] = mem.read_f32(cpu.r[0]).unwrap(); // fmov @r0,fr6
    cpu.r[3] = cpu.r[3].wrapping_add(8);
    cpu.fr[0] = mem.read_f32(cpu.r[6]).unwrap(); // fmov @r6,fr0
    cpu.fr[9] = fdiv(cpu.fr[9], cpu.fr[1]); // fdiv fr1,fr9
    cpu.fr[3] = mem.read_f32(cpu.r[13]).unwrap(); // fmov @r13,fr3
    cpu.fr[6] = fmul(cpu.fr[6], cpu.fr[0]); // fmul fr0,fr6
    cpu.fr[2] = mem.read_f32(cpu.r[4]).unwrap(); // fmov @r4,fr2
    cpu.r[1] = cpu.r[1].wrapping_add(4);
    cpu.fr[0] = mem.read_f32(cpu.r[14]).unwrap(); // fmov @r14,fr0
    cpu.fr[3] = fmul(cpu.fr[3], cpu.fr[2]); // fmul fr2,fr3
    cpu.fr[6] = fadd(cpu.fr[6], cpu.fr[3]); // fadd fr3,fr6
    cpu.fr[6] = fadd(cpu.fr[6], cpu.fr[0]); // fadd fr0,fr6
    let _ = mem.write_f32(cpu.r[3], cpu.fr[6]); // fmov fr6,@r3
    cpu.fr[7] = fdiv(cpu.fr[7], cpu.fr[1]); // fdiv fr1,fr7
    let _ = mem.write_f32(cpu.r[7], cpu.fr[9]); // fmov fr9,@r7
    let _ = mem.write_f32(cpu.r[1], cpu.fr[7]); // fmov fr7,@r1
    cpu.fr[0] = mem.read_f32(cpu.r[7]).unwrap(); // fmov @r7,fr0
    cpu.fr[3] = mem.read_f32(cpu.r[0]).unwrap(); // fmov @r0,fr3
    cpu.r[7] = cpu.r[7].wrapping_add(8);
    cpu.fr[2] = mem.read_f32(cpu.r[13]).unwrap(); // fmov @r13,fr2
    cpu.fr[3] = fmul(cpu.fr[3], cpu.fr[0]); // fmul fr0,fr3
    cpu.fr[1] = mem.read_f32(cpu.r[1]).unwrap(); // fmov @r1,fr1
    cpu.fr[0] = mem.read_f32(cpu.r[14]).unwrap(); // fmov @r14,fr0
    cpu.fr[2] = fmul(cpu.fr[2], cpu.fr[1]); // fmul fr1,fr2
    cpu.fr[3] = fadd(cpu.fr[3], cpu.fr[2]); // fadd fr2,fr3
    cpu.fr[3] = fadd(cpu.fr[3], cpu.fr[0]); // fadd fr0,fr3
    let _ = mem.write_f32(cpu.r[7], cpu.fr[3]); // fmov fr3,@r7

    // epilogue: mov.l @r15+,r11/r12/r13/r14; fmov @r15+,fr13/fr14; rts; fmov @r15+,fr15
    cpu.r[11] = mem.read_u32(cpu.r[15]).unwrap();
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[12] = mem.read_u32(cpu.r[15]).unwrap();
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[13] = mem.read_u32(cpu.r[15]).unwrap();
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[14] = mem.read_u32(cpu.r[15]).unwrap();
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.fr[13] = mem.read_f32(cpu.r[15]).unwrap();
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.fr[14] = mem.read_f32(cpu.r[15]).unwrap();
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.fr[15] = mem.read_f32(cpu.r[15]).unwrap(); // rts delay slot
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}
