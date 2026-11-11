//! 0x8c01471c — float-to-int conversion of the ratio `fr4/fr5` (with
//! NaN/inf guards), storing the truncated int at `*r4` and the signed
//! remainder float in `fr0`.
//!
//! Original (abridged, see disassembly for the full listing):
//! ```text
//! mov.l r14,@-r15 ; add #-28,r15 ; mov r15,r6 ; add #24,r6
//! mov.l (0x7f800000),r5 ; mov r15,r14 ; add #20,r14
//! fmov fr4,@r6 ; fmov fr5,@r14 ; mov r15,r7
//! mov.l @r6,r3 ; and r5,r3 ; cmp/eq r5,r3 ; bt.s 0x8c014754 ; add #16,r7
//! mov.l @(20,r15),r3 ; and r5,r3 ; cmp/eq r5,r3 ; bf 0x8c01474a
//! mov.l @(20,r15),r2 ; mov.l (0x007fffff),r3 ; tst r3,r2 ; bf 0x8c014754
//! mov #20,r0 ; fldi0 fr3 ; fmov @(r0,r15),fr2 ; fcmp/eq fr3,fr2 ; bf 0x8c014760
//! 0x8c014754: mov.l (0x8c1ccf98),r3 ; mov #0,r2 ; mov.l r2,@r4
//!             fmov @r3,fr3 ; bra 0x8c0147e8 ; fmov fr3,@r7
//! 0x8c014760: mov.l @r14,r1 ; and r5,r1 ; cmp/eq r5,r1 ; bf 0x8c014770
//!             mov #0,r2 ; mov.l r2,@r4 ; bra 0x8c0147e8 ; fmov fr4,@r7
//! 0x8c014770: mov.l @r6,r2 ; mov.l (0x80000000),r3 ; and r3,r2 ; mov.l r2,@(8,r15)
//!             mov.l @r6,r1 ; mov.l (0x7fffffff),r2 ; and r2,r1 ; mov.l r1,@r6
//!             mov.l @r14,r0 ; and r2,r0 ; mov.l r0,@r14 ; mov #12,r0
//!             fmov @r14,fr3 ; fmov @r6,fr4 ; fdiv fr3,fr4 ; fmov fr4,@(r0,r15)
//!             ... ftrc/float saturation ... fcmp/gt with 2^31 ...
//!             sts fpul,r2 ; mov.l r2,@r4 ; ... fsub ... ; fmov fr3,@r7
//!             mov.l @r7,r2 ; mov.l @(8,r15),r3 ; or r3,r2 ; mov.l r2,@r7
//! 0x8c0147e8: fmov @r7,fr0 ; add #28,r15 ; rts ; mov.l @r15+,r14
//! ```
//! Side effects the C hides: `r6`/`r14` keep stack addresses (holding the
//! abs bit-patterns of fr4/fr5), `r7` ends at r15+40 (holding the result),
//! `r5` keeps the masked exponent of the quotient, `r0` keeps a mova target
//! or 12/20, `r1`/`r2`/`r3` keep intermediate words, `fpul` keeps the final
//! `ftrc` result, a store to `*r4` happens on every path, and `r14` is
//! restored from the stack on return.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    // mov.l r14,@-r15 ; add #-28,r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[14]);
    cpu.r[15] = cpu.r[15].wrapping_sub(28);
    // mov r15,r6 ; add #24,r6
    cpu.r[6] = cpu.r[15].wrapping_add(24);
    // mov.l 0x8c0147f0,r5
    cpu.r[5] = 0x7f80_0000;
    // mov r15,r14 ; add #20,r14
    cpu.r[14] = cpu.r[15].wrapping_add(20);
    // fmov fr4,@r6 ; fmov fr5,@r14 ; mov r15,r7
    let _ = mem.write_f32(cpu.r[6], cpu.fr[4]);
    let _ = mem.write_f32(cpu.r[14], cpu.fr[5]);
    cpu.r[7] = cpu.r[15];

    // mov.l @r6,r3 ; and r5,r3 ; cmp/eq r5,r3 ; bt.s 0x8c014754 ; add #16,r7
    cpu.r[3] = mem.read_u32(cpu.r[6]).unwrap_or(0) & cpu.r[5];
    cpu.set_t(cpu.r[3] == 0x7f80_0000);
    cpu.r[7] = cpu.r[7].wrapping_add(16); // delay slot runs either way
    if cpu.r[3] == 0x7f80_0000 {
        finish(cpu, mem, 0x8c1c_cf98, true);
        return; // NaN constant path
    }
    // mov.l @(20,r15),r3 ; and r5,r3 ; cmp/eq r5,r3 ; bf 0x8c01474a
    cpu.r[3] = mem.read_u32(cpu.r[15].wrapping_add(20)).unwrap_or(0) & cpu.r[5];
    cpu.set_t(cpu.r[3] == 0x7f80_0000);
    if cpu.r[3] != 0x7f80_0000 {
        // bf taken -> 0x8c01474a: mov #20,r0 ; fldi0 fr3 ; fmov @(r0,r15),fr2
        cpu.r[0] = 20;
        cpu.fr[3] = 0.0;
        cpu.fr[2] = mem
            .read_f32(cpu.r[0].wrapping_add(cpu.r[15]))
            .unwrap_or(0.0);
        // fcmp/eq fr3,fr2 ; bf 0x8c014760
        cpu.set_t(cpu.fr[3] == cpu.fr[2]);
        if cpu.fr[3] != cpu.fr[2] {
            return check_fr5(cpu, mem);
        }
        finish(cpu, mem, 0x8c1c_cf98, true);
        return;
    }
    // fall through: mov.l @(20,r15),r2 ; mov.l (0x007fffff),r3 ; tst r3,r2 ; bf 0x8c014754
    cpu.r[2] = mem.read_u32(cpu.r[15].wrapping_add(20)).unwrap_or(0);
    cpu.r[3] = 0x007f_ffff;
    cpu.set_t(cpu.r[2] & cpu.r[3] == 0);
    if cpu.r[2] & cpu.r[3] != 0 {
        finish(cpu, mem, 0x8c1c_cf98, true);
        return;
    }
    check_fr5(cpu, mem)
}

/// 0x8c014760: fr5 inf/nan guard, else the main ratio path.
fn check_fr5(cpu: &mut Cpu, mem: &mut Memory) {
    // mov.l @r14,r1 ; and r5,r1 ; cmp/eq r5,r1 ; bf 0x8c014770
    cpu.r[1] = mem.read_u32(cpu.r[14]).unwrap_or(0) & 0x7f80_0000;
    cpu.set_t(cpu.r[1] == 0x7f80_0000);
    if cpu.r[1] == 0x7f80_0000 {
        // mov #0,r2 ; mov.l r2,@r4 ; bra 0x8c0147e8 ; fmov fr4,@r7
        cpu.r[2] = 0;
        let _ = mem.write_u32(cpu.r[4], cpu.r[2]);
        let _ = mem.write_f32(cpu.r[7], cpu.fr[4]);
        finish(cpu, mem, 0, false);
        return;
    }
    main_path(cpu, mem)
}

/// 0x8c014770..0x8c0147e6: abs values, divide, saturate, truncate, remainder.
fn main_path(cpu: &mut Cpu, mem: &mut Memory) {
    // mov.l @r6,r2 ; mov.l (0x80000000),r3 ; and r3,r2 ; mov.l r2,@(8,r15)
    cpu.r[2] = mem.read_u32(cpu.r[6]).unwrap_or(0) & 0x8000_0000;
    cpu.r[3] = 0x8000_0000;
    let _ = mem.write_u32(cpu.r[15].wrapping_add(8), cpu.r[2]);
    // mov.l @r6,r1 ; mov.l (0x7fffffff),r2 ; and r2,r1 ; mov.l r1,@r6
    cpu.r[2] = 0x7fff_ffff;
    cpu.r[1] = mem.read_u32(cpu.r[6]).unwrap_or(0) & cpu.r[2];
    let _ = mem.write_u32(cpu.r[6], cpu.r[1]);
    // mov.l @r14,r0 ; and r2,r0 ; mov.l r0,@r14 ; mov #12,r0
    cpu.r[0] = mem.read_u32(cpu.r[14]).unwrap_or(0) & cpu.r[2];
    let _ = mem.write_u32(cpu.r[14], cpu.r[0]);
    cpu.r[0] = 12;
    // fmov @r14,fr3 ; fmov @r6,fr4 ; fdiv fr3,fr4 ; fmov fr4,@(r0,r15)
    cpu.fr[3] = mem.read_f32(cpu.r[14]).unwrap_or(0.0);
    cpu.fr[4] = mem.read_f32(cpu.r[6]).unwrap_or(0.0);
    cpu.fr[4] = cpu.fr[4] / cpu.fr[3];
    let _ = mem.write_f32(cpu.r[0].wrapping_add(cpu.r[15]), cpu.fr[4]);
    // mov.l @(12,r15),r1 ; mov.l r1,@(4,r15) ; and r3,r1 ; mov.l r1,@r15
    cpu.r[1] = mem.read_u32(cpu.r[15].wrapping_add(12)).unwrap_or(0);
    let _ = mem.write_u32(cpu.r[15].wrapping_add(4), cpu.r[1]);
    cpu.r[1] &= cpu.r[3];
    let _ = mem.write_u32(cpu.r[15], cpu.r[1]);
    // mov.l @(4,r15),r1 ; mov.l (0x4b000000),r3 ; and r1,r5 ; cmp/ge r3,r5 ; bt 0x8c0147ae
    cpu.r[1] = mem.read_u32(cpu.r[15].wrapping_add(4)).unwrap_or(0);
    cpu.r[3] = 0x4b00_0000;
    cpu.r[5] = cpu.r[1] & 0x7f80_0000;
    cpu.set_t((cpu.r[5] as i32) >= (cpu.r[3] as i32));
    if (cpu.r[5] as i32) < (cpu.r[3] as i32) {
        // ftrc fr4,fpul ; sts fpul,r0 ; lds r0,fpul ; mov #12,r0
        // float fpul,fr3 ; fmov fr3,@(r0,r15)
        cpu.fpul = cpu.fr[4] as i32 as u32;
        cpu.r[0] = cpu.fpul;
        cpu.fpul = cpu.r[0];
        cpu.r[0] = 12;
        cpu.fr[3] = cpu.fpul as i32 as f32;
        let _ = mem.write_f32(cpu.r[0].wrapping_add(cpu.r[15]), cpu.fr[3]);
    }
    // mov #12,r0 ; mov.l @(12,r15),r2 ; mov.l @r15,r3 ; or r3,r2 ; mov.l r2,@(12,r15)
    cpu.r[0] = 12;
    cpu.r[2] = mem.read_u32(cpu.r[15].wrapping_add(12)).unwrap_or(0);
    cpu.r[3] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[2] |= cpu.r[3];
    let _ = mem.write_u32(cpu.r[15].wrapping_add(12), cpu.r[2]);
    // fmov @(r0,r15),fr0 ; mova 0x8c014808,r0 ; fmov @r0,fr2 ; fcmp/gt fr2,fr0 ; bf 0x8c0147cc
    cpu.fr[0] = mem
        .read_f32(cpu.r[0].wrapping_add(cpu.r[15]))
        .unwrap_or(0.0);
    cpu.r[0] = 0x8c01_4808;
    cpu.fr[2] = mem.read_f32(cpu.r[0]).unwrap_or(0.0);
    cpu.set_t(cpu.fr[0] > cpu.fr[2]);
    if cpu.fr[0] > cpu.fr[2] {
        // mova 0x8c01480c,r0 ; fmov fr0,fr3 ; fmov @r0,fr1 ; bra 0x8c0147ce ; fadd fr1,fr3
        cpu.r[0] = 0x8c01_480c;
        cpu.fr[3] = cpu.fr[0];
        cpu.fr[1] = mem.read_f32(cpu.r[0]).unwrap_or(0.0);
        cpu.fr[3] = cpu.fr[3] + cpu.fr[1];
    } else {
        // fmov fr0,fr3
        cpu.fr[3] = cpu.fr[0];
    }
    // ftrc fr3,fpul ; fmov fr0,fr2 ; sts fpul,r2 ; mov.l r2,@r4
    cpu.fpul = cpu.fr[3] as i32 as u32;
    cpu.fr[2] = cpu.fr[0];
    cpu.r[2] = cpu.fpul;
    let _ = mem.write_u32(cpu.r[4], cpu.r[2]);
    // fmov @r14,fr3 ; fmul fr3,fr2 ; fmov @r6,fr3 ; fsub fr2,fr3 ; fmov fr3,@r7
    cpu.fr[3] = mem.read_f32(cpu.r[14]).unwrap_or(0.0);
    cpu.fr[2] = cpu.fr[2] * cpu.fr[3];
    cpu.fr[3] = mem.read_f32(cpu.r[6]).unwrap_or(0.0);
    cpu.fr[3] = cpu.fr[3] - cpu.fr[2];
    let _ = mem.write_f32(cpu.r[7], cpu.fr[3]);
    // mov.l @r7,r2 ; mov.l @(8,r15),r3 ; or r3,r2 ; mov.l r2,@r7
    cpu.r[2] = mem.read_u32(cpu.r[7]).unwrap_or(0);
    cpu.r[3] = mem.read_u32(cpu.r[15].wrapping_add(8)).unwrap_or(0);
    cpu.r[2] |= cpu.r[3];
    let _ = mem.write_u32(cpu.r[7], cpu.r[2]);
    finish(cpu, mem, 0, false)
}

/// 0x8c0147e8..0x8c0147ee: load the result, unwind the frame.
/// `const_addr` is non-zero on the NaN-constant path (fr3 loaded from it).
fn finish(cpu: &mut Cpu, mem: &mut Memory, const_addr: u32, load_nan: bool) {
    if load_nan {
        // 0x8c014754: mov.l (0x8c1ccf98),r3 ; mov #0,r2 ; mov.l r2,@r4
        //             fmov @r3,fr3 ; bra ; fmov fr3,@r7
        cpu.r[3] = const_addr;
        cpu.r[2] = 0;
        let _ = mem.write_u32(cpu.r[4], cpu.r[2]);
        cpu.fr[3] = mem.read_f32(cpu.r[3]).unwrap_or(0.0);
        let _ = mem.write_f32(cpu.r[7], cpu.fr[3]);
    }
    // fmov @r7,fr0 ; add #28,r15 ; rts ; mov.l @r15+,r14
    cpu.fr[0] = mem.read_f32(cpu.r[7]).unwrap_or(0.0);
    cpu.r[15] = cpu.r[15].wrapping_add(28);
    cpu.r[14] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}
