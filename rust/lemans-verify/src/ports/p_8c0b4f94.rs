//! 0x8c0b4f94 — 3-component vector add: dst (stack arg at @r15) gets
//! a[i] + b[i] for i = 0..2 from r4 (a) and r5 (b); identical shape to
//! 0x8c0b4f5c with `fadd` instead of `fsub`. Third store is the `rts` delay
//! slot; r14 is pushed/popped and ends restored.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let dst = mem.read_u32(cpu.r[15]).unwrap_or(0);
    let r4 = cpu.r[4];
    let r5 = cpu.r[5];
    cpu.fr[1] = mem.read_f32(r4).unwrap_or(0.0);
    let mut r7 = r4;
    cpu.fr[0] = mem.read_f32(r5).unwrap_or(0.0);
    let mut r6 = r5;
    r7 = r7.wrapping_add(8);
    r6 = r6.wrapping_add(8);
    cpu.fr[1] = cpu.fr[1] + cpu.fr[0]; // fadd fr0,fr1
    let mut r3 = r4;
    let mut r1 = r5;
    r3 = r3.wrapping_add(4);
    cpu.fr[3] = mem.read_f32(r7).unwrap_or(0.0);
    let mut r0 = dst;
    cpu.fr[0] = mem.read_f32(r6).unwrap_or(0.0);
    r1 = r1.wrapping_add(4);
    cpu.fr[2] = mem.read_f32(r3).unwrap_or(0.0);
    cpu.fr[3] = cpu.fr[3] + cpu.fr[0]; // fadd fr0,fr3
    cpu.fr[0] = mem.read_f32(r1).unwrap_or(0.0);
    r0 = r0.wrapping_add(4);
    let _ = mem.write_f32(dst, cpu.fr[1]);
    cpu.fr[2] = cpu.fr[2] + cpu.fr[0]; // fadd fr0,fr2
    let _ = mem.write_f32(r0, cpu.fr[2]);
    let r14 = dst.wrapping_add(8);
    // rts ; fmov fr3,@r14
    let _ = mem.write_f32(r14, cpu.fr[3]);
    cpu.r[0] = r0;
    cpu.r[1] = r1;
    cpu.r[3] = r3;
    cpu.r[6] = r6;
    cpu.r[7] = r7;
}
