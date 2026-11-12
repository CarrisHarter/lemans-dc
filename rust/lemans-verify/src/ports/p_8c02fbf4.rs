//! 0x8c02fbf4 — scale `*(r4+0x40)` by 0.02, then index a float
//! table at `*(r4+4)` by its integer part and interpolate.
//!
//! Original:
//! ```text
//! fmov fr15,@-r15 ; mov.l r14,@-r15 ; sts.l pr,@-r15 ; mov r4,r14
//! mov r14,r1 ; add #64,r1 ; mova 0x8c02fc5c,r0
//! fmov @r1,fr15 ; fmov @r0,fr0 ; fcmp/gt fr15,fr0 ; bt 0x8c02fc12
//! mov.l @(4,r14),r0 ; add #108,r0 ; bra 0x8c02fc54 ; fmov @r0,fr0
//! 0x8c02fc12: fldi0 fr0 ; fcmp/gt fr0,fr15 ; bt 0x8c02fc20
//! mov.l @(4,r14),r0 ; add #80,r0 ; bra 0x8c02fc54 ; fmov @r0,fr0
//! 0x8c02fc20: mova 0x8c02fc60,r0 ; fmov @r0,fr1 ; fldi1 fr5
//! mov.l 0x8c02fc64,r1 ; mova 0x8c02fc68,r0 ; fmov @r0,fr0
//! fmul fr1,fr15 ; fmul fr0,fr15 ; jsr @r1 ; fmov fr15,fr4
//! fsub fr0,fr15 ; mov.l @(4,r14),r1 ; ftrc fr15,fpul ; fldi1 fr1
//! mov r1,r0 ; add #80,r1 ; sts fpul,r3 ; shll2 r3 ; add #84,r0
//! add r3,r1 ; add r3,r0 ; fsub fr0,fr1 ; fmov @r1,fr2 ; fmov @r0,fr0
//! fmul fr1,fr2 ; fadd fr2,fr0
//! 0x8c02fc54: lds.l @r15+,pr ; mov.l @r15+,r14 ; rts ; fmov @r15+,fr15
//! ```
//! Literals: [0x8c02fc5c] = 97.22222 (upper clamp), [0x8c02fc60] =
//! 0.0028571428 (1/350), [0x8c02fc64] = 0x8c010b88, [0x8c02fc68] = 7.0.
//!
//! Semantics: x = *(r4+0x40); fr15 is preserved across the call. When
//! x >= 97.22222 the result is table[0x6c]; when x <= 0 the result is
//! table[0x50]; else t = x/50, (int, frac) = split(t) via 0x8c010b88,
//! and the result is table[0x54+4*int] + table[0x50+4*int]*frac.
//!
//! Side effects the C hides: the fldi1 at 0x8c02fc24 leaves 1.0 in fr5
//! on the main path only; fmul/fsub overwrite the fr0 literal with
//! (frac, 1-frac); r0/r1/r3 keep table addresses and idx*4; fpul keeps
//! the truncated index; T ends as the last fcmp/gt (or the callee's T
//! on the main path); the rts delay slot restores fr15.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let _ = mem.write_f32(cpu.r[15].wrapping_sub(4), cpu.fr[15]);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[14]);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr);
    cpu.r[14] = cpu.r[4];
    cpu.r[1] = cpu.r[14];
    cpu.r[1] = cpu.r[1].wrapping_add(64);
    cpu.r[0] = 0x8c02fc5c;
    cpu.fr[15] = mem.read_f32(cpu.r[1]).unwrap_or(0.0);
    cpu.fr[0] = mem.read_f32(cpu.r[0]).unwrap_or(0.0);
    cpu.set_t(cpu.fr[0] > cpu.fr[15]); // fcmp/gt fr15,fr0
    if cpu.fr[0] <= cpu.fr[15] {
        // x >= 97.22222: fr0 = table[0x6c] (bra delay slot runs first).
        cpu.r[0] = mem.read_u32(cpu.r[14].wrapping_add(4)).unwrap_or(0);
        cpu.r[0] = cpu.r[0].wrapping_add(108);
        cpu.fr[0] = mem.read_f32(cpu.r[0]).unwrap_or(0.0);
        return finish(cpu, mem);
    }
    cpu.fr[0] = 0.0; // fldi0 fr0
    cpu.set_t(cpu.fr[15] > cpu.fr[0]); // fcmp/gt fr0,fr15
    if cpu.fr[15] <= cpu.fr[0] {
        // x <= 0: fr0 = table[0x50] (bra delay slot runs first).
        cpu.r[0] = mem.read_u32(cpu.r[14].wrapping_add(4)).unwrap_or(0);
        cpu.r[0] = cpu.r[0].wrapping_add(80);
        cpu.fr[0] = mem.read_f32(cpu.r[0]).unwrap_or(0.0);
        return finish(cpu, mem);
    }
    cpu.r[0] = 0x8c02fc60;
    cpu.fr[1] = mem.read_f32(cpu.r[0]).unwrap_or(0.0);
    cpu.fr[5] = 1.0; // fldi1 fr5
    cpu.r[1] = mem.read_u32(0x8c02fc64).unwrap_or(0);
    cpu.r[0] = 0x8c02fc68;
    cpu.fr[0] = mem.read_f32(cpu.r[0]).unwrap_or(0.0);
    cpu.fr[15] = cpu.fr[15] * cpu.fr[1];
    cpu.fr[15] = cpu.fr[15] * cpu.fr[0];
    cpu.fr[4] = cpu.fr[15]; // delay slot of `jsr`
    cpu.pr = 0x8c02fc34;
    crate::registry::p_8c010b88::run(cpu, mem);
    cpu.fr[15] = cpu.fr[15] - cpu.fr[0];
    cpu.r[1] = mem.read_u32(cpu.r[14].wrapping_add(4)).unwrap_or(0);
    cpu.fpul = cpu.fr[15] as i32 as u32; // ftrc fr15,fpul
    cpu.fr[1] = 1.0; // fldi1 fr1
    cpu.r[0] = cpu.r[1];
    cpu.r[1] = cpu.r[1].wrapping_add(80);
    cpu.r[3] = cpu.fpul; // sts fpul,r3
    cpu.r[3] = cpu.r[3] << 2; // shll2 r3
    cpu.r[0] = cpu.r[0].wrapping_add(84);
    cpu.r[1] = cpu.r[1].wrapping_add(cpu.r[3]);
    cpu.r[0] = cpu.r[0].wrapping_add(cpu.r[3]);
    cpu.fr[1] = cpu.fr[1] - cpu.fr[0];
    cpu.fr[2] = mem.read_f32(cpu.r[1]).unwrap_or(0.0);
    cpu.fr[0] = mem.read_f32(cpu.r[0]).unwrap_or(0.0);
    cpu.fr[2] = cpu.fr[2] * cpu.fr[1];
    cpu.fr[0] = cpu.fr[0] + cpu.fr[2];
    finish(cpu, mem);
}

/// 0x8c02fc54: restore pr/r14, `rts`; the delay slot restores fr15.
fn finish(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[14] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.fr[15] = mem.read_f32(cpu.r[15]).unwrap_or(0.0); // rts delay slot
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}
