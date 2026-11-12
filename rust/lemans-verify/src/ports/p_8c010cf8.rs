//! 0x8c010cf8 — float conversion prologue; the fr4 == 0.0 branch stores 0 at @r4.
//!
//! Non-zero fr4 splits three ways on fr4_bits: a canonical inf (r6 ==
//! 0x7f800000) writes 0x44c / 0x44d to the RAM global at 0x8c23d8b8 and
//! copies fr4 to fr0; anything else reconstructs a float in *r4 and the stack
//! (exponent biased by -126, sign/mantissa ORed with 0x3f000000) and reloads
//! fr0 from the stack. Original side effects on the zero path: r15 -4/+4
//! (stack window, excluded), fr3 := 0, r5 := r15, r3 := 0x7f800000,
//! r6 := fr4_bits & 0x7f800000, r1 := 0, *r4 := 0, fr0 := fr4, T = 1.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    cpu.fr[3] = 0.0; // fldi0 fr3
    let zero = cpu.fr[4] == 0.0;
    cpu.set_t(zero); // fcmp/eq fr3,fr4
    cpu.r[5] = cpu.r[15]; // mov r15,r5
    cpu.r[3] = mem.read_u32(0x8c010d58).unwrap_or(0x7f800000); // mov.l literal
    let _ = mem.write_u32(cpu.r[5], cpu.fr[4].to_bits()); // fmov fr4,@r5 (stack)
    let bits = mem.read_u32(cpu.r[5]).unwrap_or(0);
    cpu.r[6] = bits & cpu.r[3]; // bf.s delay slot: and r3,r6
    if zero {
        cpu.r[1] = 0;
        let _ = mem.write_u32(cpu.r[4], 0); // mov.l r1,@r4
        cpu.fr[0] = cpu.fr[4]; // fmov fr4,fr0
    } else {
        cpu.r[3] = mem.read_u32(0x8c010d58).unwrap_or(0x7f800000); // mov.l literal
        cpu.r[7] = mem.read_u32(0x8c010d5c).unwrap_or(0x7fffff); // mov.l literal
        cpu.set_t(cpu.r[6] == cpu.r[3]); // cmp/eq r3,r6
        if cpu.r[6] == cpu.r[3] {
            cpu.r[1] = bits; // mov.l @r5,r1
            cpu.set_t(cpu.r[1] & cpu.r[7] == 0); // tst r1,r7
            if cpu.r[1] & cpu.r[7] == 0 {
                cpu.r[2] = mem.read_u32(0x8c010d60).unwrap_or(0x8c23d8b8); // mov.l literal
                cpu.r[0] = 0x44c; // mov.w literal
                let _ = mem.write_u32(cpu.r[2], cpu.r[0]); // bra delay slot
            } else {
                cpu.r[1] = mem.read_u32(0x8c010d60).unwrap_or(0x8c23d8b8); // mov.l literal
                cpu.r[2] = 0x44d; // mov.w literal
                let _ = mem.write_u32(cpu.r[1], cpu.r[2]);
            }
            cpu.fr[0] = cpu.fr[4]; // fmov fr4,fr0
        } else {
            cpu.r[3] = (-23i32) as u32; // mov #-23,r3
            cpu.r[2] = mem.read_u32(0x8c010d64).unwrap_or(0x80000000); // mov.l literal
            cpu.r[6] = ((cpu.r[6] as i32) >> 23) as u32; // shad r3,r6
            cpu.r[3] = mem.read_u32(0x8c010d68).unwrap_or(0x3f000000); // mov.l literal
            cpu.r[6] = cpu.r[6].wrapping_add((-126i32) as u32); // add #-126,r6
            let _ = mem.write_u32(cpu.r[4], cpu.r[6]); // mov.l r6,@r4
            cpu.r[4] = mem.read_u32(cpu.r[5]).unwrap_or(bits); // mov.l @r5,r4
            cpu.r[7] &= cpu.r[4]; // and r4,r7
            cpu.r[2] &= cpu.r[4]; // and r4,r2
            cpu.r[2] |= cpu.r[7]; // or r7,r2
            cpu.r[2] |= cpu.r[3]; // or r3,r2
            let _ = mem.write_u32(cpu.r[5], cpu.r[2]); // mov.l r2,@r5 (stack)
            cpu.fr[0] = f32::from_bits(cpu.r[2]); // fmov @r5,fr0
        }
    }
    cpu.r[15] = cpu.r[15].wrapping_add(4); // rts delay slot
}
