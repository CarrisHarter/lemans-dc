//! 0x8c0d524c — three-vec variant of 0x8c0d5210: copies 3 dwords each from
//! r5, r6, r7 into r4[0..8], then stores fr4 at r4+36 and fr5 at r4+40.
//!
//! Residue: r0 = r4_entry+36, r1 = *(r7+8), r3 = *r7, r4 = r4_entry+40;
//! r13/r14 restored by the epilogue pops (r13 before rts, r14 in its delay
//! slot). Store order: +0,+4,+8 (r5), +12,+16,+20 (r6), +24,+28,+32 (r7),
//! then the two fmovs at +36 and +40.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let r4 = cpu.r[4];
    let saved13 = cpu.r[13];
    let saved14 = cpu.r[14];
    let r5 = cpu.r[5];
    let r6 = cpu.r[6];
    let r7 = cpu.r[7];
    let r13 = r4.wrapping_add(12);
    let r14 = r4.wrapping_add(24);
    let r0 = r4.wrapping_add(36);
    let r3 = mem.read_u32(r5).unwrap_or(0); // mov.l @r5,r3
    let r1 = mem.read_u32(r5.wrapping_add(4)).unwrap_or(0); // mov.l @(4,r5),r1
    let _ = mem.write_u32(r4, r3); // mov.l r3,@r4
    let _ = mem.write_u32(r4.wrapping_add(4), r1); // mov.l r1,@(4,r4)
    let r1 = mem.read_u32(r5.wrapping_add(8)).unwrap_or(0); // mov.l @(8,r5),r1
    let _ = mem.write_u32(r4.wrapping_add(8), r1); // mov.l r1,@(8,r4)
    let r3 = mem.read_u32(r6).unwrap_or(0); // mov.l @r6,r3
    let r4n = r4.wrapping_add(40); // add #40,r4
    let r1 = mem.read_u32(r6.wrapping_add(4)).unwrap_or(0); // mov.l @(4,r6),r1
    let _ = mem.write_u32(r13, r3); // mov.l r3,@r13
    let _ = mem.write_u32(r13.wrapping_add(4), r1); // mov.l r1,@(4,r13)
    let r1 = mem.read_u32(r6.wrapping_add(8)).unwrap_or(0); // mov.l @(8,r6),r1
    let _ = mem.write_u32(r13.wrapping_add(8), r1); // mov.l r1,@(8,r13)
    let r3 = mem.read_u32(r7).unwrap_or(0); // mov.l @r7,r3
    let r1 = mem.read_u32(r7.wrapping_add(4)).unwrap_or(0); // mov.l @(4,r7),r1
    let _ = mem.write_u32(r14, r3); // mov.l r3,@r14
    let _ = mem.write_u32(r14.wrapping_add(4), r1); // mov.l r1,@(4,r14)
    let r1 = mem.read_u32(r7.wrapping_add(8)).unwrap_or(0); // mov.l @(8,r7),r1
    let _ = mem.write_u32(r14.wrapping_add(8), r1); // mov.l r1,@(8,r14)
    let _ = mem.write_f32(r0, cpu.fr[4]); // fmov fr4,@r0
    let _ = mem.write_f32(r4n, cpu.fr[5]); // fmov fr5,@r4
    cpu.r[0] = r0;
    cpu.r[1] = r1;
    cpu.r[3] = r3;
    cpu.r[4] = r4n;
    cpu.r[13] = saved13; // mov.l @r15+,r13
    cpu.r[14] = saved14; // mov.l @r15+,r14 (rts delay slot)
}
