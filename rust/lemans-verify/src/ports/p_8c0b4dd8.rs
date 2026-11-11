//! 0x8c0b4dd8 — scale the 4-float vector `[r4]` by fr4 into r6=`[r15]`.
//! The store to `[r6+12]` is the rts delay slot after `add #12,r6`.
//!
//! Leftover registers: r0=r6+8, r1=r6+4, r3=r4+4, r5=r4+8, r6=r6+12;
//! fr0..fr3 hold the four scaled values.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let src = cpu.r[4];
    let s = cpu.fr[4];
    let dst = mem.read_u32(cpu.r[15]).unwrap_or(0);

    let r0 = mem.read_f32(src).unwrap_or(0.0) * s;
    let r1 = mem.read_f32(src + 4).unwrap_or(0.0) * s;
    let r2 = mem.read_f32(src + 8).unwrap_or(0.0) * s;
    let r3 = mem.read_f32(src + 12).unwrap_or(0.0) * s;

    let _ = mem.write_f32(dst, r0);
    let _ = mem.write_f32(dst + 4, r1);
    let _ = mem.write_f32(dst + 8, r2);
    let _ = mem.write_f32(dst + 12, r3); // delay slot of `rts`

    cpu.fr[0] = r0;
    cpu.fr[1] = r1;
    cpu.fr[2] = r2;
    cpu.fr[3] = r3;
    cpu.r[0] = dst + 8;
    cpu.r[1] = dst + 4;
    cpu.r[3] = src + 4;
    cpu.r[5] = src + 8;
    cpu.r[6] = dst + 12;
}
