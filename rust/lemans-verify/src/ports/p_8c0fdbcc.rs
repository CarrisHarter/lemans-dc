//! 0x8c0fdbcc — convert one material slot: raw 16-byte copy, or float->RGBA8.
//!
//! ```text
//! CFG (no prologue; r15/r14 untouched):
//!   r1=[r5+4] & 0x0001ffff (DAT_8c0fdc4c); T=((r1&4)==0); bt 0x8c0fdbee
//!   PATH A (bit4): 4x mov.l [r6..r6+12] -> [r5+16..r5+28] (last is rts delay)
//!     T=0; final r5=orig+16, r1=[r6+8], r0=[r6+12]
//!   0x8c0fdbee: r1 &= 0x00018000 (DAT_8c0fdc50); T=(r1==0); bt 0x8c0fdc36
//!   PATH B (0x18000): fr3=255.0f (DAT_8c0fdc54); f0..f3=[r6..r6+12]*255,
//!     ftrc each, pack b3<<24|b2<<16|b1<<8|b0 into r3, store at [r5+16]
//!     (rts delay slot); T=0; final r6=orig+12, fpul=int(f3*255)
//!   PATH C (neither): 4x mov.l [r6..r6+12] -> [r5+32..r5+44]; T=1 (!)
//!   rts (delay stores the last word)
//! ```
//! Hidden side effects: r5 is advanced by 16 or 32 on the copy paths, the
//! packed-color path leaves r0/r1/r3 as shift/or leftovers plus fr0..fr3
//! holding the *255 products, and T differs per path (A/B: 0, C: 1).

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let mut r1 = mem.read_u32(cpu.r[5].wrapping_add(4)).unwrap(); // mov.l @(4,r5),r1
    let mask = mem.read_u32(0x8C0FDC4C).unwrap(); // mov.l 0x8c0fdc4c,r0
    r1 &= mask; // and r0,r1
    cpu.r[1] = r1;
    cpu.r[0] = r1; // mov r1,r0
    cpu.r[0] &= 4; // and #4,r0
    cpu.set_t(cpu.r[0] & cpu.r[0] == 0); // tst r0,r0

    if cpu.r[0] != 0 {
        // PATH A, 0x8c0fdbda: copy 16 bytes [r6] -> [r5+16]
        cpu.r[1] = mem.read_u32(cpu.r[6]).unwrap(); // mov.l @r6,r1
        cpu.r[5] = cpu.r[5].wrapping_add(16); // add #16,r5
        cpu.r[0] = mem.read_u32(cpu.r[6].wrapping_add(4)).unwrap(); // mov.l @(4,r6),r0
        let _ = mem.write_u32(cpu.r[5], cpu.r[1]); // mov.l r1,@r5
        let _ = mem.write_u32(cpu.r[5].wrapping_add(4), cpu.r[0]); // mov.l r0,@(4,r5)
        cpu.r[1] = mem.read_u32(cpu.r[6].wrapping_add(8)).unwrap(); // mov.l @(8,r6),r1
        cpu.r[0] = mem.read_u32(cpu.r[6].wrapping_add(12)).unwrap(); // mov.l @(12,r6),r0
        let _ = mem.write_u32(cpu.r[5].wrapping_add(8), cpu.r[1]); // mov.l r1,@(8,r5)
        // rts; delay slot:
        let _ = mem.write_u32(cpu.r[5].wrapping_add(12), cpu.r[0]); // mov.l r0,@(12,r5)
    } else {
        // 0x8c0fdbee
        let mask2 = mem.read_u32(0x8C0FDC50).unwrap(); // mov.l 0x8c0fdc50,r0
        r1 &= mask2; // and r0,r1
        cpu.r[1] = r1;
        cpu.set_t(r1 & r1 == 0); // tst r1,r1
        if r1 != 0 {
            // PATH B, 0x8c0fdbf6: floats at [r6..r6+12] * 255, ftrc, pack RGBA
            cpu.r[0] = 0x8C0FDC54; // mova 0x8c0fdc54,r0
            cpu.fr[0] = mem.read_f32(cpu.r[6]).unwrap(); // fmov @r6,fr0
            cpu.r[3] = cpu.r[6]; // mov r6,r3
            cpu.r[1] = cpu.r[6]; // mov r6,r1
            cpu.fr[3] = mem.read_f32(cpu.r[0]).unwrap(); // fmov @r0,fr3 (255.0)
            cpu.r[1] = cpu.r[1].wrapping_add(8); // add #8,r1
            cpu.r[3] = cpu.r[3].wrapping_add(4); // add #4,r3
            cpu.fr[0] = cpu.fr[0] * cpu.fr[3]; // fmul fr3,fr0
            cpu.r[6] = cpu.r[6].wrapping_add(12); // add #12,r6
            cpu.fr[2] = mem.read_f32(cpu.r[3]).unwrap(); // fmov @r3,fr2
            cpu.fr[1] = mem.read_f32(cpu.r[1]).unwrap(); // fmov @r1,fr1
            cpu.fpul = cpu.fr[0] as i32 as u32; // ftrc fr0,fpul
            cpu.fr[2] = cpu.fr[2] * cpu.fr[3]; // fmul fr3,fr2
            cpu.r[3] = cpu.fpul; // sts fpul,r3
            cpu.fr[0] = mem.read_f32(cpu.r[6]).unwrap(); // fmov @r6,fr0
            cpu.r[3] <<= 16; // shll16 r3
            cpu.fpul = cpu.fr[2] as i32 as u32; // ftrc fr2,fpul
            cpu.fr[1] = cpu.fr[1] * cpu.fr[3]; // fmul fr3,fr1
            cpu.r[1] = cpu.fpul; // sts fpul,r1
            cpu.fpul = cpu.fr[1] as i32 as u32; // ftrc fr1,fpul
            cpu.r[3] <<= 8; // shll8 r3
            cpu.r[1] <<= 16; // shll16 r1
            cpu.fr[0] = cpu.fr[0] * cpu.fr[3]; // fmul fr3,fr0
            cpu.r[0] = cpu.fpul; // sts fpul,r0
            cpu.r[3] |= cpu.r[1]; // or r1,r3
            cpu.r[0] <<= 8; // shll8 r0
            cpu.r[3] |= cpu.r[0]; // or r0,r3
            cpu.fpul = cpu.fr[0] as i32 as u32; // ftrc fr0,fpul
            cpu.r[0] = cpu.fpul; // sts fpul,r0
            cpu.r[3] |= cpu.r[0]; // or r0,r3
            // rts; delay slot:
            let _ = mem.write_u32(cpu.r[5].wrapping_add(16), cpu.r[3]); // mov.l r3,@(16,r5)
        } else {
            // PATH C, 0x8c0fdc36: copy 16 bytes [r6] -> [r5+32]
            cpu.r[1] = mem.read_u32(cpu.r[6]).unwrap(); // mov.l @r6,r1
            cpu.r[5] = cpu.r[5].wrapping_add(32); // add #32,r5
            cpu.r[0] = mem.read_u32(cpu.r[6].wrapping_add(4)).unwrap(); // mov.l @(4,r6),r0
            let _ = mem.write_u32(cpu.r[5], cpu.r[1]); // mov.l r1,@r5
            let _ = mem.write_u32(cpu.r[5].wrapping_add(4), cpu.r[0]); // mov.l r0,@(4,r5)
            cpu.r[1] = mem.read_u32(cpu.r[6].wrapping_add(8)).unwrap(); // mov.l @(8,r6),r1
            cpu.r[0] = mem.read_u32(cpu.r[6].wrapping_add(12)).unwrap(); // mov.l @(12,r6),r0
            let _ = mem.write_u32(cpu.r[5].wrapping_add(8), cpu.r[1]); // mov.l r1,@(8,r5)
            // rts; delay slot:
            let _ = mem.write_u32(cpu.r[5].wrapping_add(12), cpu.r[0]); // mov.l r0,@(12,r5)
        }
    }
}
