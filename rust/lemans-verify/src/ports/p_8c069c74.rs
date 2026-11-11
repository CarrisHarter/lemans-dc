//! 0x8c069c74 — scale a 4-float vector: `out[i] = in[i] * fr4` (single precision).
//!
//! Original (25 instructions, no branches):
//! ```text
//! mov.l @r15,r6 ; mov r4,r0 ; mov r4,r5 ; mov r4,r3 ; add #12,r0
//! fmov @r0,fr3  ; add #8,r5  ; add #4,r3  ; fmov @r4,fr0 ; mov r6,r1
//! fmov @r5,fr2  ; mov r6,r0  ; fmov @r3,fr1 ; fmul fr4,fr0 ; add #4,r1
//! fmov fr0,@r6  ; fmul fr4,fr1 ; fmov fr1,@r1 ; fmul fr4,fr2 ; add #8,r0
//! fmov fr2,@r0  ; fmul fr4,fr3 ; add #12,r6 ; rts ; fmov fr3,@r6
//! ```
//! The C shows FPSCR SZ/PR double-precision paths, but the machine code only
//! runs the single-precision one. Side effects the C hides: the out pointer is
//! loaded from `[r15]` into r6 and r6 ends at out+12; r0 = out+8, r1 = out+4,
//! r3 = in+4, r5 = in+8; fr0..fr3 are overwritten with the scaled values; the
//! last store `[out+12] = fr3` is the `rts` delay slot. Store order is
//! out+0, out+4, out+8, out+12.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let src = cpu.r[4];
    let out = mem.read_u32(cpu.r[15]).unwrap(); // mov.l @r15,r6
    cpu.r[6] = out;
    cpu.r[0] = src;
    cpu.r[5] = src;
    cpu.r[3] = src;
    cpu.r[0] = cpu.r[0].wrapping_add(12); // add #12,r0
    cpu.fr[3] = mem.read_f32(cpu.r[0]).unwrap(); // fmov @r0,fr3
    cpu.r[5] = cpu.r[5].wrapping_add(8); // add #8,r5
    cpu.r[3] = cpu.r[3].wrapping_add(4); // add #4,r3
    cpu.fr[0] = mem.read_f32(src).unwrap(); // fmov @r4,fr0
    cpu.r[1] = cpu.r[6]; // mov r6,r1
    cpu.fr[2] = mem.read_f32(cpu.r[5]).unwrap(); // fmov @r5,fr2
    cpu.r[0] = cpu.r[6]; // mov r6,r0
    cpu.fr[1] = mem.read_f32(cpu.r[3]).unwrap(); // fmov @r3,fr1
    cpu.fr[0] = cpu.fr[0] * cpu.fr[4]; // fmul fr4,fr0
    cpu.r[1] = cpu.r[1].wrapping_add(4); // add #4,r1
    let _ = mem.write_f32(cpu.r[6], cpu.fr[0]); // fmov fr0,@r6
    cpu.fr[1] = cpu.fr[1] * cpu.fr[4]; // fmul fr4,fr1
    let _ = mem.write_f32(cpu.r[1], cpu.fr[1]); // fmov fr1,@r1
    cpu.fr[2] = cpu.fr[2] * cpu.fr[4]; // fmul fr4,fr2
    cpu.r[0] = cpu.r[0].wrapping_add(8); // add #8,r0
    let _ = mem.write_f32(cpu.r[0], cpu.fr[2]); // fmov fr2,@r0
    cpu.fr[3] = cpu.fr[3] * cpu.fr[4]; // fmul fr4,fr3
    cpu.r[6] = cpu.r[6].wrapping_add(12); // add #12,r6
    let _ = mem.write_f32(cpu.r[6], cpu.fr[3]); // delay slot of rts
}
