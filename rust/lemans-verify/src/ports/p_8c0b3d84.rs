//! 0x8c0b3d84 — `[r6]..[r6+8] = M^T-ish product`, where the matrix words are
//! m00=[r4], m01=[r4+4], m02=[r4+8], m10=[r4+12], the vector is v0..v2=[r5..+8],
//! and the destination pointer r6 is loaded from `@(20,r15)` — i.e. the word
//! at the entry r15 (the 4 fr pushes + add #-4 make the offset land there).
//! fr12..fr15 are saved/restored through r15.
//!
//! Leftover registers: r0=dst+4, r1=r5+8, r3=r4+4, r4=r4+12, r5=r5+4,
//! r6=dst+8.
//!
//! NaN propagation: same rule as 0x8c06f6cc — the oracle keeps the first
//! operand's NaN bits when both inputs are NaN, else the second's; this was
//! established by probing the oracle, not from an SH-4 manual. Every product
//! below is written in the original `fmul frX,frY` operand order (frY first),
//! so the helpers see the same first NaN the original does. See
//! notes/address-map.md if that rule ever changes.

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

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let m = cpu.r[4];
    let v = cpu.r[5];
    let dst = mem.read_u32(cpu.r[15]).unwrap_or(0);
    let m00 = mem.read_f32(m).unwrap_or(0.0);
    let m01 = mem.read_f32(m + 4).unwrap_or(0.0);
    let m02 = mem.read_f32(m + 8).unwrap_or(0.0);
    let m10 = mem.read_f32(m + 12).unwrap_or(0.0);
    let v0 = mem.read_f32(v).unwrap_or(0.0);
    let v1 = mem.read_f32(v + 4).unwrap_or(0.0);
    let v2 = mem.read_f32(v + 8).unwrap_or(0.0);

    let a = fadd(fsub(fmul(v0, m00), fmul(v1, m10)), fmul(v2, m02)); // fr11
    let b = fadd(fadd(fmul(v0, m01), fmul(v1, m02)), fmul(v2, m10)); // fr2
    let c = fadd(fsub(fmul(v2, m00), fmul(v0, m02)), fmul(v1, m01)); // fr0
    let d = fadd(fsub(fmul(v1, m00), fmul(v2, m01)), fmul(v0, m10)); // fr13
    let out0 = fsub(fadd(fadd(fmul(m00, a), fmul(m01, b)), fmul(m02, c)), fmul(m10, d)); // fr10
    let out1 = fsub(fadd(fadd(fmul(m00, d), fmul(m02, b)), fmul(m10, a)), fmul(m01, c)); // fr7
    let out2 = fsub(fadd(fadd(fmul(m00, c), fmul(m10, b)), fmul(m01, d)), fmul(m02, a)); // fr3

    let _ = mem.write_f32(dst, out0);
    let _ = mem.write_f32(dst + 4, out1);
    let _ = mem.write_f32(dst + 8, out2);

    cpu.fr[0] = c;
    cpu.fr[1] = fmul(m02, a);
    cpu.fr[2] = b;
    cpu.fr[3] = out2;
    cpu.fr[4] = fmul(m01, d);
    cpu.fr[5] = fmul(m01, c);
    cpu.fr[6] = fmul(m10, a);
    cpu.fr[7] = out1;
    cpu.fr[8] = fmul(m02, c);
    cpu.fr[9] = fmul(m01, b);
    cpu.fr[10] = out0;
    cpu.fr[11] = a;

    cpu.r[0] = dst + 4;
    cpu.r[1] = v + 8;
    cpu.r[3] = m + 4;
    cpu.r[4] = m + 12;
    cpu.r[5] = v + 4;
    cpu.r[6] = dst + 8;
}
