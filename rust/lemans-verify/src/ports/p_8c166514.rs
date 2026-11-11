//! 0x8c166514 — small FPU transform: reads three floats at r7 and five at
//! the stack-arg struct (r12 = mem[r15-20+52]), branches on the byte at
//! 0x8c2b2790, then stores fr8/fr0 bits to *r5/*(r5+4) and 1-fr9 to *r6.
//!
//! byte != 0 (path A): fr5 = ((fr0*fr6 + fr3*fr1)*fr7 + fr6b), fr8 = fr5,
//! fr0 = (fr0*(-fr1) + fr3*fr6) * lit + lit, fr9 = 0.
//! byte == 0 (path B): fr9 = fr5^4, fr8 = fr5*fr7 + fr6b,
//! fr0 = (lit2*fr0) / (fr4b + lit1) ... per the original fdiv/fmul chain.
//! The bt.s delay (fadd fr3,fr0) runs on both paths.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    // prologue: mov.l r14,r13,r12,@-r15 ; add #-8,r15
    for n in [14usize, 13, 12] {
        cpu.r[15] = cpu.r[15].wrapping_sub(4);
        let v = cpu.r[n];
        let _ = mem.write_u32(cpu.r[15], v);
    }
    cpu.r[15] = cpu.r[15].wrapping_sub(8);
    let sp = cpu.r[15];
    let r12 = mem.read_u32(sp.wrapping_add(52)).unwrap_or(0); // mov.l @(52,r15),r12
    let r7e = cpu.r[7];
    let r5e = cpu.r[5];
    let r6e = cpu.r[6];

    cpu.fr[3] = mem.read_f32(r7e.wrapping_add(8)).unwrap_or(0.0); // fmov @(r7+8),fr3
    cpu.fr[0] = mem.read_f32(r7e).unwrap_or(0.0); // fmov @r7,fr0
    let r14 = r12.wrapping_add(124);
    let fr1 = mem.read_f32(r14).unwrap_or(0.0); // fmov @r14,fr1
    let r3 = r12.wrapping_add(0x80); // mov.l 0x8c1665c4,r4 = 0x80 ; add r4,r3
    let r13 = r12.wrapping_add(112);
    let r1 = r12.wrapping_add(120);
    let byte = mem.read_u8(mem.read_u32(0x8c1665c8).unwrap_or(0)).unwrap_or(0) as u32; // mov.b @0x8c2b2790,r0 ; extu.b
    let fr6 = mem.read_f32(r3).unwrap_or(0.0); // fmov @r3,fr6 (r12+128)
    cpu.fr[2] = cpu.fr[3];
    cpu.fr[2] = cpu.fr[2] * fr1; // fmul fr1,fr2
    let fr5 = cpu.fr[0];
    cpu.fr[1] = -fr1; // fneg fr1
    cpu.fr[5] = fr5 * fr6; // fmul fr6,fr5
    let r12n = r12.wrapping_add(116);
    cpu.fr[0] = cpu.fr[0] * cpu.fr[1]; // fmul fr1,fr0
    cpu.fr[3] = cpu.fr[3] * fr6; // fmul fr6,fr3
    let fr4 = mem.read_f32(r7e.wrapping_add(4)).unwrap_or(0.0); // fmov @(r7+4),fr4
    cpu.fr[5] = cpu.fr[5] + cpu.fr[2]; // fadd fr2,fr5
    let fr1b = mem.read_f32(r12n).unwrap_or(0.0); // fmov @r12,fr1 (r12+116)
    let t = byte == 0; // tst r0,r0
    let fr7 = mem.read_f32(r13).unwrap_or(0.0); // fmov @r13,fr7 (r12+112)
    let fr4b = fr4 * fr1b; // fmul fr1,fr4
    let fr6b = mem.read_f32(r1).unwrap_or(0.0); // fmov @r1,fr6 (r12+120)
    cpu.fr[0] = cpu.fr[0] + cpu.fr[3]; // bt.s delay: fadd fr3,fr0 (always runs)
    cpu.set_t(t);
    if !t {
        // path A (byte != 0)
        let fr1c = mem.read_f32(0x8c1665cc).unwrap_or(0.0);
        cpu.fr[5] = cpu.fr[5] * fr7;
        cpu.fr[5] = cpu.fr[5] + fr6b;
        cpu.fr[0] = cpu.fr[0] * fr1c;
        cpu.fr[8] = cpu.fr[5];
        cpu.fr[0] = cpu.fr[0] + fr1c;
        cpu.fr[9] = 0.0; // bra delay: fldi0 fr9
        cpu.fr[1] = fr1c;
        cpu.fr[4] = fr4b;
        cpu.fr[6] = fr6b;
    } else {
        // path B (byte == 0)
        cpu.fr[9] = cpu.fr[5];
        cpu.fr[8] = cpu.fr[5];
        let fr1d = mem.read_f32(0x8c20b200).unwrap_or(0.0);
        cpu.fr[9] = cpu.fr[9] * cpu.fr[5];
        cpu.fr[8] = cpu.fr[8] * fr7;
        cpu.fr[9] = cpu.fr[9] * cpu.fr[5];
        let fr4c = fr4b + fr1d; // fadd fr1,fr4
        let fr2 = mem.read_f32(0x8c20b1f8).unwrap_or(0.0);
        let fr3n = 1.0f32 / fr4c; // fldi1 fr3 ; fdiv fr4,fr3
        let fr1e = mem.read_f32(0x8c1665cc).unwrap_or(0.0);
        let fr2b = fr2 * cpu.fr[0]; // fmul fr0,fr2
        let fr3n = fr3n * fr2b; // fmul fr2,fr3
        let fr3n = fr3n + fr1e; // fadd fr1,fr3
        cpu.fr[0] = fr3n; // fmov fr3,fr0
        cpu.fr[8] = cpu.fr[8] + fr6b; // fadd fr6,fr8
        cpu.fr[9] = cpu.fr[9] * cpu.fr[5];
        cpu.fr[1] = fr1e;
        cpu.fr[2] = fr2b;
        cpu.fr[3] = fr3n;
        cpu.fr[4] = fr4c;
        cpu.fr[6] = fr6b;
    }
    cpu.fr[7] = fr7; // fmov @r13,fr7 is never overwritten
    // common tail
    let _ = mem.write_f32(sp, cpu.fr[8]); // fmov fr8,@(r0,r15), r0 = 0
    let fr1f = 1.0f32 - cpu.fr[9]; // fldi1 fr1 ; fsub fr9,fr1
    let _ = mem.write_f32(sp.wrapping_add(4), cpu.fr[0]); // r0 = 4
    let r1 = mem.read_u32(sp).unwrap_or(0);
    let r0f = mem.read_u32(sp.wrapping_add(4)).unwrap_or(0);
    let _ = mem.write_u32(r5e, r1); // mov.l r1,@r5
    let _ = mem.write_u32(r5e.wrapping_add(4), r0f); // mov.l r0,@(4,r5)
    let _ = mem.write_f32(r6e, fr1f); // fmov fr1,@r6
    // epilogue: add #8,r15 ; pop r12,r13 ; rts ; delay pop r14
    cpu.r[15] = cpu.r[15].wrapping_add(8);
    for n in [12usize, 13, 14] {
        let v = mem.read_u32(cpu.r[15]).unwrap_or(0);
        cpu.r[15] = cpu.r[15].wrapping_add(4);
        cpu.r[n] = v;
    }
    cpu.r[0] = r0f;
    cpu.r[1] = r1;
    cpu.r[3] = r12.wrapping_add(128);
    cpu.r[4] = 0x80;
    cpu.r[7] = r7e.wrapping_add(4);
    cpu.fr[1] = fr1f;
}
