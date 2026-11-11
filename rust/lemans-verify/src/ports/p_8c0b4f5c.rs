//! 0x8c0b4f5c — 3-component vector subtract: dst (stack arg at @r15) gets
//! a[i] - b[i] for i = 0..2 from r4 (a) and r5 (b); r14 is pushed and popped,
//! so it ends restored. The third store is the `rts` delay slot.
//!
//! Register residue: r0 = dst+4, r1 = r5+4, r3 = r4+4, r6 = r5+8, r7 = r4+8;
//! fr0 = b[1], fr1 = a0-b0, fr2 = a1-b1, fr3 = a2-b2. T is never touched.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    // mov.l r14,@-r15 ; mov.l @(4,r15),r14 — dst = [r15 at entry]
    let dst = mem.read_u32(cpu.r[15]).unwrap_or(0);
    let r4 = cpu.r[4];
    let r5 = cpu.r[5];
    cpu.fr[1] = mem.read_f32(r4).unwrap_or(0.0); // fmov @r4,fr1
    let mut r7 = r4;
    cpu.fr[0] = mem.read_f32(r5).unwrap_or(0.0); // fmov @r5,fr0
    let mut r6 = r5;
    r7 = r7.wrapping_add(8); // add #8,r7
    r6 = r6.wrapping_add(8); // add #8,r6
    cpu.fr[1] = cpu.fr[1] - cpu.fr[0]; // fsub fr0,fr1
    let mut r3 = r4;
    let mut r1 = r5;
    r3 = r3.wrapping_add(4); // add #4,r3
    cpu.fr[3] = mem.read_f32(r7).unwrap_or(0.0); // fmov @r7,fr3
    let mut r0 = dst; // mov r14,r0
    cpu.fr[0] = mem.read_f32(r6).unwrap_or(0.0); // fmov @r6,fr0
    r1 = r1.wrapping_add(4); // add #4,r1
    cpu.fr[2] = mem.read_f32(r3).unwrap_or(0.0); // fmov @r3,fr2
    cpu.fr[3] = cpu.fr[3] - cpu.fr[0]; // fsub fr0,fr3
    cpu.fr[0] = mem.read_f32(r1).unwrap_or(0.0); // fmov @r1,fr0
    r0 = r0.wrapping_add(4); // add #4,r0
    let _ = mem.write_f32(dst, cpu.fr[1]); // fmov fr1,@r14
    cpu.fr[2] = cpu.fr[2] - cpu.fr[0]; // fsub fr0,fr2
    let _ = mem.write_f32(r0, cpu.fr[2]); // fmov fr2,@r0
    let r14 = dst.wrapping_add(8); // add #8,r14
    // rts ; fmov fr3,@r14 (delay slot)
    let _ = mem.write_f32(r14, cpu.fr[3]);
    cpu.r[0] = r0;
    cpu.r[1] = r1;
    cpu.r[3] = r3;
    cpu.r[6] = r6;
    cpu.r[7] = r7;
    // mov.l @r15+,r14 restores the entry r14; r15 is net unchanged
}
