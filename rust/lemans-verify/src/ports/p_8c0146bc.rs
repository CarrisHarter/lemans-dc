//! 0x8c0146bc — round a float towards zero and rebuild it with its sign
//! (`ftrc`-based normalisation), returning the value in `fr0`.
//!
//! Original:
//! ```text
//! add #-8,r15 ; mov r15,r3 ; add #4,r3 ; fmov fr4,@r3
//! mov.l (0x7f800000),r3 ; mov.l (0x4b000000),r2 ; mov.l (0x80000000),r6
//! fmov fr4,@r15 ; mov.l @r15,r5 ; mov r5,r4 ; and r3,r5 ; cmp/ge r2,r5
//! bt.s 0x8c0146e2 ; and r6,r4
//! ftrc fr4,fpul ; sts fpul,r0 ; lds r0,fpul ; float fpul,fr3 ; fmov fr3,@r15
//! mov.l @r15,r3 ; mov r15,r2 ; add #4,r2 ; or r4,r3 ; mov.l r3,@r15
//! mov.l @r2,r3 ; tst r3,r6 ; bt.s 0x8c014704 ; fmov @r15,fr4
//! mov #4,r0 ; fmov @(r0,r15),fr3 ; fcmp/eq fr4,fr3 ; bt 0x8c014704
//! mov.l (0x8c1ccfd4),r3 ; fmov fr4,fr2 ; fmov @r3,fr4 ; fadd fr2,fr4
//! fmov fr4,fr0 ; rts ; add #8,r15
//! ```
//! Side effects the C hides: `r15` is decremented/restored, `r2`/`r3` keep
//! stack addresses and the last tested word, `r4` keeps the sign bit of
//! `fr4`, `r5` keeps the masked exponent, `r6` keeps 0x80000000, `fpul`
//! keeps the `ftrc` result on the small-exponent path, and T ends as the
//! `tst r3,r6` result (1 when `fr4` is positive).

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    // add #-8,r15 ; mov r15,r3 ; add #4,r3 ; fmov fr4,@r3
    cpu.r[15] = cpu.r[15].wrapping_sub(8);
    cpu.r[3] = cpu.r[15].wrapping_add(4);
    let _ = mem.write_f32(cpu.r[3], cpu.fr[4]);
    // mov.l 0x8c014710,r3 ; mov.l 0x8c014714,r2 ; mov.l 0x8c01470c,r6
    cpu.r[3] = 0x7f80_0000; // exponent mask
    cpu.r[2] = 0x4b00_0000; // overflow threshold
    cpu.r[6] = 0x8000_0000; // sign mask
    // fmov fr4,@r15 ; mov.l @r15,r5 ; mov r5,r4 ; and r3,r5 ; cmp/ge r2,r5
    let _ = mem.write_f32(cpu.r[15], cpu.fr[4]);
    cpu.r[5] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[4] = cpu.r[5];
    cpu.r[5] &= cpu.r[3];
    cpu.set_t((cpu.r[5] as i32) >= (cpu.r[2] as i32));
    // bt.s 0x8c0146e2 ; and r6,r4 (delay slot runs either way)
    cpu.r[4] &= cpu.r[6];
    if (cpu.r[5] as i32) < (cpu.r[2] as i32) {
        // ftrc fr4,fpul ; sts fpul,r0 ; lds r0,fpul ; float fpul,fr3 ; fmov fr3,@r15
        cpu.fpul = cpu.fr[4] as i32 as u32;
        cpu.r[0] = cpu.fpul;
        cpu.fpul = cpu.r[0];
        cpu.fr[3] = cpu.fpul as i32 as f32;
        let _ = mem.write_f32(cpu.r[15], cpu.fr[3]);
    }
    // mov.l @r15,r3 ; mov r15,r2 ; add #4,r2 ; or r4,r3 ; mov.l r3,@r15
    cpu.r[3] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[2] = cpu.r[15].wrapping_add(4);
    cpu.r[3] |= cpu.r[4];
    let _ = mem.write_u32(cpu.r[15], cpu.r[3]);
    // mov.l @r2,r3 ; tst r3,r6 ; bt.s 0x8c014704 ; fmov @r15,fr4 (delay slot)
    cpu.r[3] = mem.read_u32(cpu.r[2]).unwrap_or(0);
    cpu.set_t(cpu.r[3] & cpu.r[6] == 0);
    cpu.fr[4] = mem.read_f32(cpu.r[15]).unwrap_or(0.0);
    if cpu.r[3] & cpu.r[6] != 0 {
        // negative: mov #4,r0 ; fmov @(r0,r15),fr3 ; fcmp/eq fr4,fr3 ; bt 0x8c014704
        cpu.r[0] = 4;
        cpu.fr[3] = mem.read_f32(cpu.r[0].wrapping_add(cpu.r[15])).unwrap_or(0.0);
        cpu.set_t(cpu.fr[4] == cpu.fr[3]);
        if cpu.fr[4] != cpu.fr[3] {
            // mov.l 0x8c014718,r3 ; fmov fr4,fr2 ; fmov @r3,fr4 ; fadd fr2,fr4
            cpu.r[3] = 0x8c1c_cfd4; // -1.0
            cpu.fr[2] = cpu.fr[4];
            cpu.fr[4] = mem.read_f32(cpu.r[3]).unwrap_or(0.0);
            cpu.fr[4] = cpu.fr[4] + cpu.fr[2];
        }
    }
    // fmov fr4,fr0 ; rts ; add #8,r15
    cpu.fr[0] = cpu.fr[4];
    cpu.r[15] = cpu.r[15].wrapping_add(8);
}
