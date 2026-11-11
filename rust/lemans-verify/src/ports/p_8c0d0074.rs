//! 0x8c0d0074 — float clamp helper on a struct at *r4 with a double in fr4.
//!
//! Branch semantics (oracle encoding: fcmp/gt sets T = fr4 > frN): the
//! first test is `fr4 > 4.0` (const at 0x8c0d00d8), the second `fr4 > 0.1`
//! (0x8c0d00e0). On the first taken path: *(*r4+12) = 1.0f, r3 = 1,
//! *(*r4+44) = 0.9f (0x8c0d00dc) * *(float*)(*r4+32); r0 = 22, r1 = *r4,
//! fr0 = 0.9f, fr1 = product, T = 1. Store order: +12, +44, then the rts
//! delay slot writes the flag byte at *r4+22.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let r4 = cpu.r[4];
    let fr4 = cpu.fr[4];
    let c0 = f32::from_bits(mem.read_u32(0x8c0d00d8).unwrap_or(0)); // 4.0
    cpu.fr[0] = c0; // fmov @r0,fr0
    let t = fr4 > c0; // fcmp/gt fr0,fr4 -> T = fr4 > fr0
    cpu.set_t(t);
    let mut r3: u32;
    if t {
        // 8c0d007c: param > 4.0
        let mut r1 = mem.read_u32(r4).unwrap_or(0); // mov.l @r4,r1
        cpu.fr[0] = 1.0; // fldi1 fr0
        r1 = r1.wrapping_add(12); // add #12,r1
        let _ = mem.write_f32(r1, cpu.fr[0]); // fmov fr0,@r1
        r3 = 1; // mov #1,r3
        r1 = mem.read_u32(r4).unwrap_or(0); // mov.l @r4,r1
        let fr0 = f32::from_bits(mem.read_u32(0x8c0d00dc).unwrap_or(0)); // 0.9f
        cpu.fr[0] = fr0; // fmov @r0,fr0
        let r0 = r1; // mov r1,r0
        r1 = r1.wrapping_add(32); // add #32,r1
        let fr1 = f32::from_bits(mem.read_u32(r1).unwrap_or(0)); // fmov @r1,fr1
        cpu.fr[1] = fr1;
        let r0 = r0.wrapping_add(44); // add #44,r0
        cpu.fr[1] = fr0 * fr1; // fmul fr0,fr1
        let _ = mem.write_f32(r0, cpu.fr[1]); // bra delay slot: fmov fr1,@r0
    } else {
        let c2 = f32::from_bits(mem.read_u32(0x8c0d00e0).unwrap_or(0)); // 0.1f
        cpu.fr[2] = c2; // fmov @r0,fr2
        let t2 = fr4 > c2; // fcmp/gt fr2,fr4 -> T = fr4 > fr2
        cpu.set_t(t2);
        if t2 {
            // 8c0d00a2: param > 0.1
            let mut r1 = mem.read_u32(r4).unwrap_or(0); // mov.l @r4,r1
            let fr1 = f32::from_bits(mem.read_u32(0x8c0d00e4).unwrap_or(0)); // 3.9f
            cpu.fr[1] = fr1; // fmov @r0,fr1
            cpu.fr[0] = 1.0; // fldi1 fr0
            r1 = r1.wrapping_add(12); // add #12,r1
            let _ = mem.write_f32(r1, cpu.fr[0]); // fmov fr0,@r1
            r1 = mem.read_u32(r4).unwrap_or(0); // mov.l @r4,r1
            let mut fr4 = fr4;
            fr4 = fr4 - c2; // fsub fr2,fr4
            let fr0 = f32::from_bits(mem.read_u32(0x8c0d00e8).unwrap_or(0)); // 0.8f
            cpu.fr[0] = fr0; // fmov @r0,fr0
            fr4 = fr4 / fr1; // fdiv fr1,fr4
            r3 = 1; // mov #1,r3
            let r0 = r1; // mov r1,r0
            fr4 = fr4 * fr0; // fmul fr0,fr4
            r1 = r1.wrapping_add(32); // add #32,r1
            let fr0b = f32::from_bits(mem.read_u32(r1).unwrap_or(0)); // fmov @r1,fr0
            cpu.fr[0] = fr0b;
            fr4 = fr4 + c2; // fadd fr2,fr4
            let r0 = r0.wrapping_add(44); // add #44,r0
            cpu.fr[0] = fr0b * fr4; // fmul fr4,fr0
            cpu.fr[4] = fr4;
            let _ = mem.write_f32(r0, cpu.fr[0]); // bra delay slot: fmov fr0,@r0
        } else {
            r3 = 0; // mov #0,r3
        }
    }
    let r1 = mem.read_u32(r4).unwrap_or(0); // mov.l @r4,r1
    cpu.r[0] = 22; // mov #22,r0
    let _ = mem.write_u8(r1.wrapping_add(22), r3 as u8); // rts delay slot: mov.b r3,@(r0,r1)
    cpu.r[1] = r1;
    cpu.r[3] = r3;
}
