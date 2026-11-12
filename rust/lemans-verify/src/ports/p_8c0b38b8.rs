//! 0x8c0b38b8 — 3x3 transform of a vector delta into three output floats.
//!
//! `m = r4`, `v = r5`, `out = *(sp)` (the stack argument). 0x8c0b4f5c
//! subtracts `v[i] - m[9+i]` — the vector at `r4+36` — into a 12-byte stack
//! local `d`. This routine then writes `out[i] = sum_j m[3i+j] * d[j]` with the
//! original single-precision multiply/add order; the third store is the `rts`
//! delay slot.
//!
//! Register residue: r0 = out+4, r1 = m+4, r2 = frame+8, r3 = m+20, r4 = m+16,
//! r5 = m+32, r6 = m+28, r7 = m+24; r13/r14/r15/pr restored from the frame.
//! T is never touched. Final fr: fr0 = m2*d2, fr1 = m5*d2, fr2 = out0,
//! fr3 = m4*d1, fr4 = out1, fr5 = m8*d2, fr6 = m7*d1, fr7 = out2, fr8 = d2.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    // mov.l r14,@-r15 ; mov.l r13,@-r15 ; mov r4,r14 ; sts.l pr,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[14]);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[13]);
    cpu.r[14] = cpu.r[4];
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr);
    cpu.r[15] = cpu.r[15].wrapping_sub(12); // add #-12,r15
    cpu.r[13] = mem.read_u32(cpu.r[15].wrapping_add(24)).unwrap_or(0); // mov.l @(24,r15),r13
    cpu.r[4] = cpu.r[5]; // mov r5,r4 — callee `a`
    cpu.r[0] = cpu.r[15]; // mov r15,r0 — address of the 12-byte stack local
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[0]); // mov.l r0,@-r15 — callee `dst`
    cpu.r[0] = 0x8c0b4f5c; // mov.l @(0x8c0b394c),r0
    cpu.r[5] = cpu.r[14]; // mov r14,r5 — callee `b`
    cpu.pr = 0x8c0b38d2; // jsr @r0 -> 0x8c0b4f5c
    cpu.r[5] = cpu.r[5].wrapping_add(36); // delay slot: add #36,r5
    crate::registry::p_8c0b4f5c::run(cpu, mem);
    cpu.r[15] = cpu.r[15].wrapping_add(4); // add #4,r15

    cpu.r[2] = 0; // mov #0,r2
    cpu.r[2] = cpu.r[2].wrapping_add(cpu.r[15]); // add r15,r2 — d0
    cpu.fr[0] = mem.read_f32(cpu.r[2]).unwrap_or(0.0); // fmov @r2,fr0
    cpu.r[7] = cpu.r[14]; // mov r14,r7
    cpu.r[0] = cpu.r[14]; // mov r14,r0
    cpu.r[2] = 4; // mov #4,r2
    cpu.r[2] = cpu.r[2].wrapping_add(cpu.r[15]); // add r15,r2 — d1
    cpu.r[1] = cpu.r[14]; // mov r14,r1
    cpu.r[0] = cpu.r[0].wrapping_add(12); // add #12,r0
    cpu.fr[4] = mem.read_f32(cpu.r[0]).unwrap_or(0.0); // fmov @r0,fr4 — m3
    cpu.r[7] = cpu.r[7].wrapping_add(24); // add #24,r7
    cpu.fr[5] = mem.read_f32(cpu.r[2]).unwrap_or(0.0); // fmov @r2,fr5 — d1
    cpu.r[6] = cpu.r[14]; // mov r14,r6
    cpu.r[4] = cpu.r[14]; // mov r14,r4
    cpu.fr[2] = mem.read_f32(cpu.r[14]).unwrap_or(0.0); // fmov @r14,fr2 — m0
    cpu.r[1] = cpu.r[1].wrapping_add(4); // add #4,r1
    cpu.fr[7] = mem.read_f32(cpu.r[7]).unwrap_or(0.0); // fmov @r7,fr7 — m6
    cpu.r[4] = cpu.r[4].wrapping_add(16); // add #16,r4
    cpu.r[6] = cpu.r[6].wrapping_add(28); // add #28,r6
    cpu.fr[1] = mem.read_f32(cpu.r[1]).unwrap_or(0.0); // fmov @r1,fr1 — m1
    cpu.r[5] = cpu.r[14]; // mov r14,r5
    cpu.r[3] = cpu.r[14]; // mov r14,r3
    cpu.fr[3] = mem.read_f32(cpu.r[4]).unwrap_or(0.0); // fmov @r4,fr3 — m4
    cpu.r[2] = 8; // mov #8,r2
    cpu.fr[6] = mem.read_f32(cpu.r[6]).unwrap_or(0.0); // fmov @r6,fr6 — m7
    cpu.r[2] = cpu.r[2].wrapping_add(cpu.r[15]); // add r15,r2 — d2
    cpu.r[0] = cpu.r[13]; // mov r13,r0
    cpu.r[14] = cpu.r[14].wrapping_add(8); // add #8,r14
    cpu.fr[2] = cpu.fr[2] * cpu.fr[0]; // fmul fr0,fr2 — m0*d0
    cpu.fr[1] = cpu.fr[1] * cpu.fr[5]; // fmul fr5,fr1 — m1*d1
    cpu.fr[4] = cpu.fr[4] * cpu.fr[0]; // fmul fr0,fr4 — m3*d0
    cpu.fr[7] = cpu.fr[7] * cpu.fr[0]; // fmul fr0,fr7 — m6*d0
    cpu.fr[3] = cpu.fr[3] * cpu.fr[5]; // fmul fr5,fr3 — m4*d1
    cpu.fr[6] = cpu.fr[6] * cpu.fr[5]; // fmul fr5,fr6 — m7*d1
    cpu.r[5] = cpu.r[5].wrapping_add(32); // add #32,r5
    cpu.fr[8] = mem.read_f32(cpu.r[2]).unwrap_or(0.0); // fmov @r2,fr8 — d2
    cpu.r[3] = cpu.r[3].wrapping_add(20); // add #20,r3
    cpu.fr[0] = mem.read_f32(cpu.r[14]).unwrap_or(0.0); // fmov @r14,fr0 — m2
    cpu.fr[2] = cpu.fr[2] + cpu.fr[1]; // fadd fr1,fr2
    cpu.fr[5] = mem.read_f32(cpu.r[5]).unwrap_or(0.0); // fmov @r5,fr5 — m8
    cpu.fr[0] = cpu.fr[0] * cpu.fr[8]; // fmul fr8,fr0 — m2*d2
    cpu.fr[1] = mem.read_f32(cpu.r[3]).unwrap_or(0.0); // fmov @r3,fr1 — m5
    cpu.fr[2] = cpu.fr[2] + cpu.fr[0]; // fadd fr0,fr2 — out0
    cpu.fr[4] = cpu.fr[4] + cpu.fr[3]; // fadd fr3,fr4
    let _ = mem.write_f32(cpu.r[13], cpu.fr[2]); // fmov fr2,@r13
    cpu.fr[1] = cpu.fr[1] * cpu.fr[8]; // fmul fr8,fr1 — m5*d2
    cpu.fr[4] = cpu.fr[4] + cpu.fr[1]; // fadd fr1,fr4 — out1
    cpu.r[0] = cpu.r[0].wrapping_add(4); // add #4,r0
    let _ = mem.write_f32(cpu.r[0], cpu.fr[4]); // fmov fr4,@r0
    cpu.fr[7] = cpu.fr[7] + cpu.fr[6]; // fadd fr6,fr7
    cpu.fr[5] = cpu.fr[5] * cpu.fr[8]; // fmul fr8,fr5 — m8*d2
    cpu.fr[7] = cpu.fr[7] + cpu.fr[5]; // fadd fr5,fr7 — out2
    cpu.r[13] = cpu.r[13].wrapping_add(8); // add #8,r13
    let _ = mem.write_f32(cpu.r[13], cpu.fr[7]); // fmov fr7,@r13 (rts delay slot)

    // epilogue: add #12,r15 ; lds.l @r15+,pr ; mov.l @r15+,r13 ; rts ;
    // mov.l @r15+,r14 (delay slot)
    cpu.r[15] = cpu.r[15].wrapping_add(12);
    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[13] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[14] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}
