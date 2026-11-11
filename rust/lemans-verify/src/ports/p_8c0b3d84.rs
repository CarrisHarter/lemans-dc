//! 0x8c0b3d84 — `[r6]..[r6+8] = M^T-ish product`, where the matrix words are
//! m00=[r4], m01=[r4+4], m02=[r4+8], m10=[r4+12], the vector is v0..v2=[r5..+8],
//! and the destination pointer r6 is loaded from `@(20,r15)` — i.e. the word
//! at the entry r15 (the 4 fr pushes + add #-4 make the offset land there).
//! fr12..fr15 are saved/restored through r15.
//!
//! Leftover registers: r0=dst+4, r1=r5+8, r3=r4+4, r4=r4+12, r5=r5+4,
//! r6=dst+8.

use sh4_oracle::{Cpu, Memory};

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

    let a = v0 * m00 - m10 * v1 + m02 * v2; // fr11
    let b = v0 * m01 + m02 * v1 + m10 * v2; // fr2
    let c = v2 * m00 - m02 * v0 + m01 * v1; // fr0
    let d = v1 * m00 - m01 * v2 + m10 * v0; // fr13
    let out0 = a * m00 + b * m01 + c * m02 - d * m10; // fr10
    let out1 = d * m00 + b * m02 + a * m10 - c * m01; // fr7
    let out2 = c * m00 + b * m10 + d * m01 - a * m02; // fr3

    let _ = mem.write_f32(dst, out0);
    let _ = mem.write_f32(dst + 4, out1);
    let _ = mem.write_f32(dst + 8, out2);

    cpu.fr[0] = c;
    cpu.fr[1] = a * m02;
    cpu.fr[2] = b;
    cpu.fr[3] = out2;
    cpu.fr[4] = d * m01;
    cpu.fr[5] = c * m01;
    cpu.fr[6] = a * m10;
    cpu.fr[7] = out1;
    cpu.fr[8] = c * m02;
    cpu.fr[9] = b * m01;
    cpu.fr[10] = out0;
    cpu.fr[11] = a;

    cpu.r[0] = dst + 4;
    cpu.r[1] = v + 8;
    cpu.r[3] = m + 4;
    cpu.r[4] = m + 12;
    cpu.r[5] = v + 4;
    cpu.r[6] = dst + 8;
}
