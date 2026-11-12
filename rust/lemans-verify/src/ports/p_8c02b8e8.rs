//! 0x8c02b8e8 — blend the flag-selected component floats of an object into a
//! smoothstep-shaped factor.
//!
//! Arguments: r4 = object base, r5 = index (element stride 0xac), r6 = pointer
//! to the two weights. Returns a float in fr0.
//!
//! CFG (every path ends at the single epilogue 0x8c02b9f4):
//! ```text
//! fr4 = |*(f32*)(r4+64)|                    (call 0x8c0b2780)
//! if 5.0 > fr4 -> return 0.0                (bra 0x8c02b9f4 ; fldi0 fr0)
//! g  = *(u32*)0x8c2b7578
//! fl = *(u32*)(r4 + 108 + 0xac*r5)          (r13 = macl after mul.l)
//! fl == 0            -> return *(f32*)(g+24)
//! fl & 0x008         -> return *(f32*)(g+44)
//! fl & 0x004         -> return *(f32*)(g+64)
//! s = 0
//! fl & 0x010 -> s += *(f32*)(g+84)
//! fl & 0x040 -> s += *(f32*)(g+124)
//! fl & 0x020 -> s += *(f32*)(g+104)
//! fl & 0x100 -> s += *(f32*)(g+124)         (same field as 0x040, adds twice)
//! fl & 0x200 -> s += *(f32*)(g+104)         (same field as 0x020, adds twice)
//! if !(s > 0) -> return 0.0
//! a = w[0]*(s + 2.93f) + s                  (call 0x8c010b38)
//! b = w[1]*(s + 1.71f) + s                  (call 0x8c010b38)
//! t = ((a - floor(a)) + (b - floor(b))) * 0.5
//! t = |t|                                   (call 0x8c0b2780)
//! return s * t*t*(3 - 2t)
//! ```
//!
//! `0x8c010b38` floors its argument (0x8c0146bc truncates towards zero and
//! subtracts 1.0 for negatives, leaving |x| >= 2^23 alone), so the two `a`/`b`
//! differences are the fractional parts of the weighted sums.
//!
//! Side effects the decompiled C hides:
//! - `mul.l r0,r13` writes both `mach` (high word) and `macl` (low word) with
//!   the signed 0xac*r5 product; `r13` then keeps `macl`.
//! - `r12` becomes `r4+108`; `r4` is overwritten with the global pointer
//!   `*(u32*)0x8c2b7578` and then advanced by 24/44/64/104 on the early and
//!   accumulate paths, so it never returns to its entry value.
//! - `r1` keeps 0x100 once the last flag test loads it; `r0` keeps the last
//!   literal/`mova` address (0x8c02ba30 on the computing path).
//! - `fr13` is the accumulator (pushed/restored), `fr14`/`fr15` keep the two
//!   weighted sums (also pushed/restored); `fr0`..`fr4` are scratch.
//! - `pr` is borrowed by four `jsr`s (0x8c02b904 -> 0x8c02b908,
//!   0x8c02b9b2 -> 0x8c02b9b6, 0x8c02b9ca -> 0x8c02b9ce,
//!   0x8c02b9da -> 0x8c02b9de) and the epilogue restores the caller's.
//! - T is left as the last flag test / `fcmp/gt fr0,fr13` result.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    // 0x8c02b8e8 prologue: push fr15,fr14,fr13,r14,r13,r12,pr and keep the
    // arguments in r12/r13/r14.
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_f32(cpu.r[15], cpu.fr[15]);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_f32(cpu.r[15], cpu.fr[14]);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_f32(cpu.r[15], cpu.fr[13]);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[14]);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[13]);
    cpu.r[14] = cpu.r[6];
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[12]);
    cpu.r[13] = cpu.r[5];
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr);
    cpu.r[12] = cpu.r[4];

    // mov r12,r0 ; add #64,r0 ; fmov @r0,fr4
    cpu.r[0] = cpu.r[12].wrapping_add(64);
    cpu.fr[4] = mem.read_f32(cpu.r[0]).unwrap_or(0.0);
    // mov.l 0x8c02ba04,r0 (8c0b2780) ; jsr @r0 ; nop
    cpu.r[0] = 0x8c0b2780;
    cpu.pr = 0x8c02b908;
    crate::registry::p_8c0b2780::run(cpu, mem);

    // mova 0x8c02ba08,r0 ; fmov @r0,fr1 (5.0) ; fcmp/gt fr0,fr1 ; bf
    cpu.r[0] = 0x8c02ba08;
    cpu.fr[1] = mem.read_f32(cpu.r[0]).unwrap_or(0.0);
    cpu.set_t(cpu.fr[1] > cpu.fr[0]);
    if cpu.t() {
        // bra 0x8c02b9f4 ; fldi0 fr0
        cpu.fr[0] = 0.0;
        return epilogue(cpu, mem);
    }

    // mov.l 0x8c02ba0c,r0 (0xac) ; mul.l r0,r13 ; sts macl,r13
    cpu.r[0] = 0x0000_00ac;
    let p = (cpu.r[13] as i32 as i64) * (cpu.r[0] as i32 as i64);
    cpu.mach = (p >> 32) as u32;
    cpu.macl = p as u32;
    cpu.r[13] = cpu.macl;
    // mov.l 0x8c02ba10,r0 (8c2b7578) ; add #108,r12 ; mov.l @r0,r4
    cpu.r[0] = 0x8c2b_7578;
    cpu.r[12] = cpu.r[12].wrapping_add(108);
    cpu.r[4] = mem.read_u32(cpu.r[0]).unwrap_or(0);
    // mov r13,r0 ; mov.l @(r0,r12),r3
    cpu.r[0] = cpu.r[13];
    cpu.r[3] = mem.read_u32(cpu.r[0].wrapping_add(cpu.r[12])).unwrap_or(0);
    // tst r3,r3 ; bf.s 0x8c02b930 ; fldi0 fr13 (delay slot runs either way)
    cpu.set_t(cpu.r[3] == 0);
    cpu.fr[13] = 0.0;
    if cpu.t() {
        // add #24,r4 ; bra 0x8c02b9f4 ; fmov @r4,fr0
        cpu.r[4] = cpu.r[4].wrapping_add(24);
        cpu.fr[0] = mem.read_f32(cpu.r[4]).unwrap_or(0.0);
        return epilogue(cpu, mem);
    }

    // mov r3,r0 ; and #8,r0 ; tst r0,r0 ; bt 0x8c02b93e
    cpu.r[0] = cpu.r[3] & 8;
    cpu.set_t(cpu.r[0] == 0);
    if !cpu.t() {
        // add #44,r4 ; bra 0x8c02b9f4 ; fmov @r4,fr0
        cpu.r[4] = cpu.r[4].wrapping_add(44);
        cpu.fr[0] = mem.read_f32(cpu.r[4]).unwrap_or(0.0);
        return epilogue(cpu, mem);
    }

    // mov r3,r0 ; and #4,r0 ; tst r0,r0 ; bt 0x8c02b94c
    cpu.r[0] = cpu.r[3] & 4;
    cpu.set_t(cpu.r[0] == 0);
    if !cpu.t() {
        // add #64,r4 ; bra 0x8c02b9f4 ; fmov @r4,fr0
        cpu.r[4] = cpu.r[4].wrapping_add(64);
        cpu.fr[0] = mem.read_f32(cpu.r[4]).unwrap_or(0.0);
        return epilogue(cpu, mem);
    }

    // and #16: s += g[84]
    cpu.r[0] = cpu.r[3] & 16;
    cpu.set_t(cpu.r[0] == 0);
    if !cpu.t() {
        cpu.r[0] = cpu.r[4].wrapping_add(84);
        cpu.fr[0] = mem.read_f32(cpu.r[0]).unwrap_or(0.0);
        cpu.fr[13] = cpu.fr[13] + cpu.fr[0];
    }

    // and #64: s += g[124]
    cpu.r[0] = cpu.r[3] & 64;
    cpu.set_t(cpu.r[0] == 0);
    if !cpu.t() {
        cpu.r[0] = cpu.r[4].wrapping_add(124);
        cpu.fr[0] = mem.read_f32(cpu.r[0]).unwrap_or(0.0);
        cpu.fr[13] = cpu.fr[13] + cpu.fr[0];
    }

    // and #32: s += g[104]
    cpu.r[0] = cpu.r[3] & 32;
    cpu.set_t(cpu.r[0] == 0);
    if !cpu.t() {
        cpu.r[0] = cpu.r[4].wrapping_add(104);
        cpu.fr[0] = mem.read_f32(cpu.r[0]).unwrap_or(0.0);
        cpu.fr[13] = cpu.fr[13] + cpu.fr[0];
    }

    // mov.l 0x8c02ba14,r1 (0x100) ; mov r3,r0 ; and r1,r0 ; tst r0,r0
    cpu.r[1] = 0x0000_0100;
    cpu.r[0] = cpu.r[3] & cpu.r[1];
    cpu.set_t(cpu.r[0] == 0);
    if !cpu.t() {
        cpu.r[0] = cpu.r[4].wrapping_add(124);
        cpu.fr[0] = mem.read_f32(cpu.r[0]).unwrap_or(0.0);
        cpu.fr[13] = cpu.fr[13] + cpu.fr[0];
    }

    // mov.l 0x8c02ba18,r0 (0x200) ; and r0,r3 ; tst r3,r3
    cpu.r[0] = 0x0000_0200;
    cpu.r[3] &= cpu.r[0];
    cpu.set_t(cpu.r[3] == 0);
    if !cpu.t() {
        cpu.r[4] = cpu.r[4].wrapping_add(104);
        cpu.fr[0] = mem.read_f32(cpu.r[4]).unwrap_or(0.0);
        cpu.fr[13] = cpu.fr[13] + cpu.fr[0];
    }

    // fldi0 fr0 ; fcmp/gt fr0,fr13 ; bf 0x8c02b9f4
    cpu.fr[0] = 0.0;
    cpu.set_t(cpu.fr[13] > cpu.fr[0]);
    if !cpu.t() {
        return epilogue(cpu, mem);
    }

    // mova 0x8c02ba1c,r0 ; fmov @r0,fr1 (2.93) ; fmov fr13,fr0
    cpu.r[0] = 0x8c02ba1c;
    cpu.fr[1] = mem.read_f32(cpu.r[0]).unwrap_or(0.0);
    cpu.fr[0] = cpu.fr[13];
    // fmov @r14,fr14 ; fadd fr1,fr0
    cpu.fr[14] = mem.read_f32(cpu.r[14]).unwrap_or(0.0);
    cpu.fr[0] = cpu.fr[0] + cpu.fr[1];
    // mov.l 0x8c02ba20,r0 (8c010b38) ; fmul fr0,fr14 ; fadd fr13,fr14
    cpu.r[0] = 0x8c010b38;
    cpu.fr[14] = cpu.fr[14] * cpu.fr[0];
    cpu.fr[14] = cpu.fr[14] + cpu.fr[13];
    // jsr @r0 ; fmov fr14,fr4
    cpu.fr[4] = cpu.fr[14];
    cpu.pr = 0x8c02b9b6;
    crate::registry::p_8c010b38::run(cpu, mem);

    // mova 0x8c02ba24,r0 ; add #4,r14 ; fmov @r0,fr2 (1.71) ; fmov fr13,fr1
    cpu.r[0] = 0x8c02ba24;
    cpu.r[14] = cpu.r[14].wrapping_add(4);
    cpu.fr[2] = mem.read_f32(cpu.r[0]).unwrap_or(0.0);
    cpu.fr[1] = cpu.fr[13];
    // fmov @r14,fr15 ; fadd fr2,fr1
    cpu.fr[15] = mem.read_f32(cpu.r[14]).unwrap_or(0.0);
    cpu.fr[1] = cpu.fr[1] + cpu.fr[2];
    // mov.l 0x8c02ba20,r0 (8c010b38) ; fmul fr1,fr15 ; fadd fr13,fr15
    cpu.r[0] = 0x8c010b38;
    cpu.fr[15] = cpu.fr[15] * cpu.fr[1];
    cpu.fr[15] = cpu.fr[15] + cpu.fr[13];
    // fsub fr0,fr14
    cpu.fr[14] = cpu.fr[14] - cpu.fr[0];
    // jsr @r0 ; fmov fr15,fr4
    cpu.fr[4] = cpu.fr[15];
    cpu.pr = 0x8c02b9ce;
    crate::registry::p_8c010b38::run(cpu, mem);

    // fsub fr0,fr15
    cpu.fr[15] = cpu.fr[15] - cpu.fr[0];
    // mova 0x8c02ba28,r0 ; fmov @r0,fr0 (0.5)
    cpu.r[0] = 0x8c02ba28;
    cpu.fr[0] = mem.read_f32(cpu.r[0]).unwrap_or(0.0);
    // fadd fr15,fr14 ; fmul fr0,fr14
    cpu.fr[14] = cpu.fr[14] + cpu.fr[15];
    cpu.fr[14] = cpu.fr[14] * cpu.fr[0];
    // mov.l 0x8c02ba04,r0 (8c0b2780) ; jsr @r0 ; fmov fr14,fr4
    cpu.r[0] = 0x8c0b2780;
    cpu.fr[4] = cpu.fr[14];
    cpu.pr = 0x8c02b9de;
    crate::registry::p_8c0b2780::run(cpu, mem);

    // mova 0x8c02ba2c,r0 ; fmov @r0,fr2 (2.0) ; fmov fr0,fr3 ; fmul fr0,fr3
    cpu.r[0] = 0x8c02ba2c;
    cpu.fr[2] = mem.read_f32(cpu.r[0]).unwrap_or(0.0);
    cpu.fr[3] = cpu.fr[0];
    cpu.fr[3] = cpu.fr[3] * cpu.fr[0];
    // mova 0x8c02ba30,r0 ; fmov @r0,fr1 (3.0)
    cpu.r[0] = 0x8c02ba30;
    cpu.fr[1] = mem.read_f32(cpu.r[0]).unwrap_or(0.0);
    // fmul fr2,fr0 ; fsub fr0,fr1 ; fmul fr1,fr3 ; fmul fr3,fr13 ; fmov fr13,fr0
    cpu.fr[0] = cpu.fr[0] * cpu.fr[2];
    cpu.fr[1] = cpu.fr[1] - cpu.fr[0];
    cpu.fr[3] = cpu.fr[3] * cpu.fr[1];
    cpu.fr[13] = cpu.fr[13] * cpu.fr[3];
    cpu.fr[0] = cpu.fr[13];

    epilogue(cpu, mem);
}

/// Epilogue at 0x8c02b9f4: pop the saved registers in reverse order; the `rts`
/// at 0x8c02ba00 has no work left for the port, and its delay slot pops fr15.
fn epilogue(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[12] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[13] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[14] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.fr[13] = mem.read_f32(cpu.r[15]).unwrap_or(0.0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.fr[14] = mem.read_f32(cpu.r[15]).unwrap_or(0.0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.fr[15] = mem.read_f32(cpu.r[15]).unwrap_or(0.0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}
