//! 0x8c0b4d74 — cross product: with a=[r4..+8], b=[r5..+8] and r3=`[r15]`,
//! stores a1*b2-a2*b1, a2*b0-a0*b2, a0*b1-a1*b0 to [r3], [r3+4], [r3+8]; the
//! third store is the rts delay slot after `add #8,r3`.
//!
//! Leftover registers: r0=r3+4, r1=r5+4, r3=r3+8, r4=r4+8, r5=r5+8;
//! fr0..fr8 hold the loaded operands and products.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let a = cpu.r[4];
    let b = cpu.r[5];
    let dst = mem.read_u32(cpu.r[15]).unwrap_or(0);
    let a0 = mem.read_f32(a).unwrap_or(0.0);
    let a1 = mem.read_f32(a + 4).unwrap_or(0.0);
    let a2 = mem.read_f32(a + 8).unwrap_or(0.0);
    let b0 = mem.read_f32(b).unwrap_or(0.0);
    let b1 = mem.read_f32(b + 4).unwrap_or(0.0);
    let b2 = mem.read_f32(b + 8).unwrap_or(0.0);

    let cross0 = a1 * b2 - a2 * b1; // fr6
    let cross1 = a2 * b0 - a0 * b2; // fr0
    let cross2 = a0 * b1 - a1 * b0; // fr4

    let _ = mem.write_f32(dst, cross0);
    let _ = mem.write_f32(dst + 4, cross1);
    let _ = mem.write_f32(dst + 8, cross2); // delay slot of `rts`

    cpu.fr[0] = cross1;
    cpu.fr[1] = b2;
    cpu.fr[2] = a2;
    cpu.fr[3] = a1 * b0;
    cpu.fr[4] = cross2;
    cpu.fr[5] = b0;
    cpu.fr[6] = cross0;
    cpu.fr[7] = b1;
    cpu.fr[8] = a0 * b2;
    cpu.fr[2] = a2 * b1;
    cpu.r[0] = dst + 4;
    cpu.r[1] = b + 4;
    cpu.r[3] = dst + 8;
    cpu.r[4] = a + 8;
    cpu.r[5] = b + 8;
}
