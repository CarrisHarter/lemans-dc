//! 0x8c0d00ec — sibling of 0x8c0d0074 with offsets +8/+40 and flag byte +21.
//!
//! Branch semantics (oracle encoding: fcmp/gt sets T = fr4 > frN): first
//! test `fr4 > 10.0` (const at 0x8c0d0154), second `fr4 > 0.01`
//! (0x8c0d015c). On the first taken path: *(*r4+8) = 1.0f, r3 = 1,
//! *(*r4+40) = 0.4f (0x8c0d0158) * *(float*)(*r4+32); r0 = 21, r1 = *r4,
//! fr0 = 0.4f, fr1 = product, T = 1. Store order: +8, +40, then the rts
//! delay slot writes the flag byte at *r4+21.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let r4 = cpu.r[4];
    let fr4 = cpu.fr[4];
    let c0 = f32::from_bits(mem.read_u32(0x8c0d0154).unwrap_or(0)); // 10.0
    cpu.fr[0] = c0; // fmov @r0,fr0
    let t = fr4 > c0; // fcmp/gt fr0,fr4 -> T = fr4 > fr0
    cpu.set_t(t);
    let mut r3: u32;
    if t {
        // 8c0d00f4: param > 10.0
        let mut r1 = mem.read_u32(r4).unwrap_or(0); // mov.l @r4,r1
        cpu.fr[0] = 1.0; // fldi1 fr0
        r1 = r1.wrapping_add(8); // add #8,r1
        let _ = mem.write_f32(r1, cpu.fr[0]); // fmov fr0,@r1
        r3 = 1; // mov #1,r3
        r1 = mem.read_u32(r4).unwrap_or(0); // mov.l @r4,r1
        let fr0 = f32::from_bits(mem.read_u32(0x8c0d0158).unwrap_or(0)); // 0.4f
        cpu.fr[0] = fr0; // fmov @r0,fr0
        let r0 = r1; // mov r1,r0
        r1 = r1.wrapping_add(32); // add #32,r1
        let fr1 = f32::from_bits(mem.read_u32(r1).unwrap_or(0)); // fmov @r1,fr1
        cpu.fr[1] = fr1;
        let r0 = r0.wrapping_add(40); // add #40,r0
        cpu.fr[1] = fr0 * fr1; // fmul fr0,fr1
        let _ = mem.write_f32(r0, cpu.fr[1]); // bra delay slot: fmov fr1,@r0
    } else {
        let c1 = f32::from_bits(mem.read_u32(0x8c0d015c).unwrap_or(0)); // 0.01f
        cpu.fr[1] = c1; // fmov @r0,fr1
        let t2 = fr4 > c1; // fcmp/gt fr1,fr4 -> T = fr4 > fr1
        cpu.set_t(t2);
        if t2 {
            // 8c0d011a: param > 0.01
            let r1 = mem.read_u32(r4).unwrap_or(0); // mov.l @r4,r1
            let fr2 = f32::from_bits(mem.read_u32(0x8c0d0160).unwrap_or(0)); // 9.99f
            cpu.fr[2] = fr2; // fmov @r0,fr2
            let mut r1 = r1.wrapping_add(8); // add #8,r1
            cpu.fr[0] = 1.0; // fldi1 fr0
            let _ = mem.write_f32(r1, cpu.fr[0]); // fmov fr0,@r1
            let mut fr4 = fr4;
            fr4 = fr4 - c1; // fsub fr1,fr4
            let r3v = mem.read_u32(r4).unwrap_or(0); // mov.l @r4,r3
            fr4 = fr4 / fr2; // fdiv fr2,fr4
            let fr1 = f32::from_bits(mem.read_u32(0x8c0d0164).unwrap_or(0)); // 0.1f
            cpu.fr[1] = fr1; // fmov @r0,fr1
            let r1 = r3v; // mov r3,r1
            let r3p = r3v.wrapping_add(32); // add #32,r3
            fr4 = fr4 * fr1; // fmul fr1,fr4
            let fr0 = f32::from_bits(mem.read_u32(0x8c0d0168).unwrap_or(0)); // 0.3f
            cpu.fr[0] = fr0; // fmov @r0,fr0
            let r1 = r1.wrapping_add(40); // add #40,r1
            let fr1b = f32::from_bits(mem.read_u32(r3p).unwrap_or(0)); // fmov @r3,fr1
            fr4 = fr4 + fr0; // fadd fr0,fr4
            cpu.fr[1] = fr1b * fr4; // fmul fr4,fr1
            cpu.fr[4] = fr4;
            let _ = mem.write_f32(r1, cpu.fr[1]); // fmov fr1,@r1
            r3 = 1; // bra delay slot: mov #1,r3
        } else {
            r3 = 0; // mov #0,r3
        }
    }
    let r1 = mem.read_u32(r4).unwrap_or(0); // mov.l @r4,r1
    cpu.r[0] = 21; // mov #21,r0
    let _ = mem.write_u8(r1.wrapping_add(21), r3 as u8); // rts delay slot: mov.b r3,@(r0,r1)
    cpu.r[1] = r1;
    cpu.r[3] = r3;
}
