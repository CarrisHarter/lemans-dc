//! 0x8c013fc0 — pow-style kernel over the double in dr4 (fr4 = low word,
//! fr5 = high word): classify both halves with the sign/zero/Inf/NaN bit
//! tricks, short-circuit the special cases, otherwise compute
//! 2^(high * log2(|low|)) through 0x8c013e94 (log2) and 0x8c013d1c (exp2),
//! then fix the sign for negative bases.
//!
//! Flag bits accumulated per half (r13 for fr4, r14 for fr5): 8 = sign,
//! 4 = zero, 2 = Inf, 1 = NaN. Dispatch:
//! ```text
//! either NaN → fr0 = f32[0x8c1ccf98] (NaN)
//! low == 0   → high > 0 ? f32[0x8c1ccf9c] (0.0) : NaN
//! high == 0  → f32[0x8c1ccfa4] (1.0)
//! low > 0    → fr0 = exp2(high * log2(low))                    (0x8c0140e4)
//! low < 0    → high must be an integer (else NaN); log2(-low),
//!              exp2(...) then 0x8c014810(high, r4 = -1) = high/2 decides
//!              the sign: negate the result unless high/2 is an integer,
//!              i.e. unless the exponent is even.               (0x8c014068)
//! ```
//! The integer test on the exponent truncates through ftrc/fpul when
//! |x| < 2^23 and restores the sign with `or 0x80000000`, so `fcmp/eq` reads
//! "already integral".
//!
//! Hidden side effects the C does not show: the `fmov @r2,fr4` in the `bf.s`
//! delay slot at 0x8c014042 loads the NaN constant on every path past the
//! flag join (fr4 is NaN even when the branch is not taken); fr14/fr15 start
//! as the two input halves (fr15 is clobbered with the 0x8c014810 result in
//! the negative-base path); fr3 = 0.0 from fldi0 (and the ftrc round-trip on
//! the truncation paths); r12 = 0x7f800000 and r7 = 1 survive until the
//! callees overwrite them; r6 = 8; r5 = 4 or 0x80000000; the `jsr` delay
//! slots feed fr4 and their callees clobber r0-r7/fr0-fr7; the Literal-pool
//! loads of r1/r2/r3 leave the constant addresses behind; the prologue pushes
//! r14/r13/r12/fr15/fr14/pr plus a 4-byte temp and the epilogue restores
//! them (r15 nets out); T ends as the last cmp/tst/fcmp of the taken path.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    // 0x8c013fc0..0x8c013fcc: prologue pushes
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[14]);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[13]);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[12]);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.fr[15].to_bits());
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.fr[14].to_bits());
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr);
    cpu.r[15] = cpu.r[15].wrapping_sub(4); // add #-4,r15
    let _ = mem.write_f32(cpu.r[15], cpu.fr[4]); // fmov fr4,@r15

    // 0x8c013fd0..0x8c013fea: classify fr4 (the low half of dr4)
    cpu.r[4] = 0; // mov #0,r4
    cpu.r[3] = mem.read_u32(cpu.r[15]).unwrap_or(0); // mov.l @r15,r3
    cpu.fr[14] = cpu.fr[4]; // fmov fr4,fr14
    cpu.set_t((cpu.r[3] as i32) >= 0); // cmp/pz r3
    cpu.fr[15] = cpu.fr[5]; // fmov fr5,fr15
    cpu.r[6] = 8; // bt.s delay slot: mov #8,r6
    if !cpu.t() {
        cpu.r[4] = cpu.r[6]; // mov r6,r4
    }
    cpu.fr[3] = 0.0; // fldi0 fr3
    cpu.set_t(cpu.fr[4] == cpu.fr[3]); // fcmp/eq fr3,fr4
    cpu.r[12] = mem.read_u32(0x8c014104).unwrap_or(0); // 0x7f800000
    cpu.r[7] = 1; // mov #1,r7
    cpu.r[5] = 4; // bf.s delay slot: mov #4,r5
    if cpu.t() {
        cpu.r[4] |= cpu.r[5]; // bra delay slot: or r5,r4
    } else {
        // 0x8c013ff0..0x8c014004
        cpu.r[2] = mem.read_u32(cpu.r[15]).unwrap_or(0); // mov.l @r15,r2
        cpu.r[3] = mem.read_u32(0x8c014108).unwrap_or(0); // 0x7fffffff
        cpu.r[2] &= cpu.r[3]; // and r3,r2
        cpu.set_t(cpu.r[2] == cpu.r[12]); // cmp/eq r12,r2
        if cpu.t() {
            cpu.r[1] = 2; // mov #2,r1
            cpu.r[4] |= cpu.r[1]; // bra delay slot: or r1,r4
        } else {
            cpu.set_t(cpu.fr[4] == cpu.fr[4]); // fcmp/eq fr4,fr4
            if !cpu.t() {
                cpu.r[4] |= cpu.r[7]; // or r7,r4
            }
        }
    }

    // 0x8c014006..0x8c014034: classify fr5 (the high half), same bit tricks
    cpu.fr[4] = cpu.fr[15]; // fmov fr15,fr4
    cpu.r[13] = cpu.r[4]; // mov r4,r13
    let _ = mem.write_f32(cpu.r[15], cpu.fr[4]); // fmov fr4,@r15
    cpu.r[3] = mem.read_u32(cpu.r[15]).unwrap_or(0); // mov.l @r15,r3
    cpu.set_t((cpu.r[3] as i32) >= 0); // cmp/pz r3
    cpu.r[4] = 0; // bt.s delay slot: mov #0,r4
    if !cpu.t() {
        cpu.r[4] = cpu.r[6]; // mov r6,r4
    }
    cpu.fr[3] = 0.0; // fldi0 fr3
    cpu.set_t(cpu.fr[4] == cpu.fr[3]); // fcmp/eq fr3,fr4
    if cpu.t() {
        cpu.r[4] |= cpu.r[5]; // bra delay slot: or r5,r4 (r5 = 4)
    } else {
        cpu.r[2] = mem.read_u32(cpu.r[15]).unwrap_or(0); // mov.l @r15,r2
        cpu.r[3] = mem.read_u32(0x8c014108).unwrap_or(0); // 0x7fffffff
        cpu.r[2] &= cpu.r[3]; // and r3,r2
        cpu.set_t(cpu.r[2] == cpu.r[12]); // cmp/eq r12,r2
        if cpu.t() {
            cpu.r[1] = 2; // mov #2,r1
            cpu.r[4] |= cpu.r[1]; // bra delay slot: or r1,r4
        } else {
            cpu.set_t(cpu.fr[4] == cpu.fr[4]); // fcmp/eq fr4,fr4
            if !cpu.t() {
                cpu.r[4] |= cpu.r[7]; // or r7,r4
            }
        }
    }

    // 0x8c014036..0x8c014042: NaN in either half returns the NaN constant
    cpu.r[3] = cpu.r[13]; // mov r13,r3
    cpu.r[14] = cpu.r[4]; // mov r4,r14
    cpu.r[3] |= cpu.r[14]; // or r14,r3
    cpu.r[2] = mem.read_u32(0x8c01410c).unwrap_or(0); // 0x8c1ccf98
    cpu.set_t(cpu.r[7] & cpu.r[3] == 0); // tst r7,r3
    cpu.fr[4] = mem.read_f32(cpu.r[2]).unwrap_or(0.0); // bf.s delay: fmov @r2,fr4
    if !cpu.t() {
        cpu.fr[0] = cpu.fr[4]; // bra delay slot: fmov fr4,fr0
        return epilogue(cpu, mem);
    }

    // 0x8c014044..0x8c01404e
    cpu.r[1] = cpu.r[13]; // mov r13,r1
    cpu.set_t(cpu.r[5] & cpu.r[1] == 0); // tst r5,r1
    if !cpu.t() {
        // low half is zero: positive high → 0.0, else NaN
        cpu.r[0] = 12; // mov #12,r0
        cpu.set_t(cpu.r[14] & cpu.r[0] == 0); // tst r14,r0
        if cpu.t() {
            cpu.r[3] = mem.read_u32(0x8c014110).unwrap_or(0); // 0x8c1ccf9c
            cpu.fr[0] = mem.read_f32(cpu.r[3]).unwrap_or(0.0); // bra delay: fmov @r3,fr0
        } else {
            cpu.fr[0] = cpu.fr[4]; // fmov fr4,fr0 (NaN)
        }
        return epilogue(cpu, mem);
    }

    // 0x8c01405a..0x8c014062: high half zero → 1.0
    cpu.set_t(cpu.r[14] & cpu.r[5] == 0); // tst r14,r5
    if !cpu.t() {
        cpu.r[3] = mem.read_u32(0x8c014114).unwrap_or(0); // 0x8c1ccfa4
        cpu.fr[0] = mem.read_f32(cpu.r[3]).unwrap_or(0.0); // bra delay: fmov @r3,fr0
        return epilogue(cpu, mem);
    }

    cpu.set_t(cpu.r[13] & cpu.r[6] == 0); // tst r13,r6
    if cpu.t() {
        // 0x8c0140e4..0x8c0140f0: positive base
        cpu.r[2] = mem.read_u32(0x8c014120).unwrap_or(0); // 0x8c013e94
        cpu.fr[4] = cpu.fr[14]; // jsr delay slot: fmov fr14,fr4
        crate::registry::p_8c013e94::run(cpu, mem);
        cpu.r[3] = mem.read_u32(0x8c014124).unwrap_or(0); // 0x8c013d1c
        cpu.fr[4] = cpu.fr[0]; // fmov fr0,fr4
        cpu.fr[4] = cpu.fr[4] * cpu.fr[15]; // jsr delay slot: fmul fr15,fr4
        crate::registry::p_8c013d1c::run(cpu, mem);
        return epilogue(cpu, mem);
    }

    // 0x8c014068..0x8c014094: negative base; the exponent must be integral
    cpu.r[5] = mem.read_u32(0x8c014118).unwrap_or(0); // 0x80000000
    cpu.r[3] = mem.read_u32(0x8c01411c).unwrap_or(0); // 0x4b000000
    cpu.fr[4] = cpu.fr[15]; // fmov fr15,fr4
    let _ = mem.write_f32(cpu.r[15], cpu.fr[4]); // fmov fr4,@r15
    cpu.r[4] = mem.read_u32(cpu.r[15]).unwrap_or(0); // mov.l @r15,r4
    cpu.r[5] &= cpu.r[4]; // and r4,r5 (sign)
    cpu.r[4] &= cpu.r[12]; // and r12,r4 (exponent bits)
    cpu.set_t((cpu.r[4] as i32) >= (cpu.r[3] as i32)); // cmp/ge r3,r4
    if !cpu.t() {
        // |high| < 2^23: truncate and round-trip through the FPU
        cpu.fpul = cpu.fr[4] as i32 as u32; // ftrc fr4,fpul
        cpu.r[1] = cpu.fpul; // sts fpul,r1
        cpu.fpul = cpu.r[1]; // lds r1,fpul
        cpu.fr[3] = cpu.fpul as i32 as f32; // float fpul,fr3
        let _ = mem.write_f32(cpu.r[15], cpu.fr[3]); // fmov fr3,@r15
    }
    cpu.r[3] = mem.read_u32(cpu.r[15]).unwrap_or(0); // mov.l @r15,r3
    cpu.r[3] |= cpu.r[5]; // or r5,r3
    let _ = mem.write_u32(cpu.r[15], cpu.r[3]); // mov.l r3,@r15
    cpu.fr[4] = mem.read_f32(cpu.r[15]).unwrap_or(0.0); // fmov @r15,fr4
    cpu.set_t(cpu.fr[4] == cpu.fr[15]); // fcmp/eq fr4,fr15
    if !cpu.t() {
        // 0x8c0140de: non-integral exponent → NaN
        cpu.r[2] = mem.read_u32(0x8c01410c).unwrap_or(0); // 0x8c1ccf98
        cpu.fr[0] = mem.read_f32(cpu.r[2]).unwrap_or(0.0); // bra delay: fmov @r2,fr0
        return epilogue(cpu, mem);
    }
    cpu.r[3] = 2; // mov #2,r3
    cpu.set_t(cpu.r[14] & cpu.r[3] == 0); // tst r14,r3
    if !cpu.t() {
        // +Inf exponent → NaN (same exit)
        cpu.r[2] = mem.read_u32(0x8c01410c).unwrap_or(0);
        cpu.fr[0] = mem.read_f32(cpu.r[2]).unwrap_or(0.0);
        return epilogue(cpu, mem);
    }

    // 0x8c014096..0x8c0140b4: log2(-low), exp2(...), then scale high by 2^-1
    cpu.r[3] = mem.read_u32(0x8c014120).unwrap_or(0); // 0x8c013e94
    cpu.fr[14] = -cpu.fr[14]; // fneg fr14
    cpu.fr[4] = cpu.fr[14]; // jsr delay slot: fmov fr14,fr4
    crate::registry::p_8c013e94::run(cpu, mem);
    cpu.r[2] = mem.read_u32(0x8c014124).unwrap_or(0); // 0x8c013d1c
    cpu.fr[4] = cpu.fr[0]; // fmov fr0,fr4
    cpu.fr[4] = cpu.fr[4] * cpu.fr[15]; // jsr delay slot: fmul fr15,fr4
    crate::registry::p_8c013d1c::run(cpu, mem);
    cpu.r[3] = mem.read_u32(0x8c014128).unwrap_or(0); // 0x8c014810
    cpu.r[4] = 0xffff_ffff; // mov #-1,r4
    cpu.fr[14] = cpu.fr[0]; // fmov fr0,fr14
    cpu.fr[4] = cpu.fr[15]; // jsr delay slot: fmov fr15,fr4
    crate::registry::p_8c014810::run(cpu, mem);

    // 0x8c0140b0..0x8c0140d8: high/2 integral ⇔ high even → keep the sign
    cpu.r[5] = mem.read_u32(0x8c014118).unwrap_or(0); // 0x80000000
    cpu.r[3] = mem.read_u32(0x8c01411c).unwrap_or(0); // 0x4b000000
    cpu.fr[15] = cpu.fr[0]; // fmov fr0,fr15
    let _ = mem.write_f32(cpu.r[15], cpu.fr[15]); // fmov fr15,@r15
    cpu.r[4] = mem.read_u32(cpu.r[15]).unwrap_or(0); // mov.l @r15,r4
    cpu.r[5] &= cpu.r[4]; // and r4,r5 (sign)
    cpu.r[4] &= cpu.r[12]; // and r12,r4 (exponent bits)
    cpu.set_t((cpu.r[4] as i32) >= (cpu.r[3] as i32)); // cmp/ge r3,r4
    if !cpu.t() {
        cpu.fpul = cpu.fr[15] as i32 as u32; // ftrc fr15,fpul
        cpu.r[1] = cpu.fpul; // sts fpul,r1
        cpu.fpul = cpu.r[1]; // lds r1,fpul
        cpu.fr[3] = cpu.fpul as i32 as f32; // float fpul,fr3
        let _ = mem.write_f32(cpu.r[15], cpu.fr[3]); // fmov fr3,@r15
    }
    cpu.r[3] = mem.read_u32(cpu.r[15]).unwrap_or(0); // mov.l @r15,r3
    cpu.r[3] |= cpu.r[5]; // or r5,r3
    let _ = mem.write_u32(cpu.r[15], cpu.r[3]); // mov.l r3,@r15
    cpu.fr[4] = mem.read_f32(cpu.r[15]).unwrap_or(0.0); // fmov @r15,fr4
    cpu.set_t(cpu.fr[4] == cpu.fr[15]); // fcmp/eq fr4,fr15
    if !cpu.t() {
        cpu.fr[14] = -cpu.fr[14]; // fneg fr14 (odd exponent)
    }
    cpu.fr[0] = cpu.fr[14]; // bra delay slot: fmov fr14,fr0
    epilogue(cpu, mem);
}

/// 0x8c0140f2..0x8c014100: restore the callee-saved state and return.
fn epilogue(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[15] = cpu.r[15].wrapping_add(4); // add #4,r15
    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0); // lds.l @r15+,pr
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.fr[14] = mem.read_f32(cpu.r[15]).unwrap_or(0.0); // fmov @r15+,fr14
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.fr[15] = mem.read_f32(cpu.r[15]).unwrap_or(0.0); // fmov @r15+,fr15
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[12] = mem.read_u32(cpu.r[15]).unwrap_or(0); // mov.l @r15+,r12
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[13] = mem.read_u32(cpu.r[15]).unwrap_or(0); // mov.l @r15+,r13
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[14] = mem.read_u32(cpu.r[15]).unwrap_or(0); // rts delay: mov.l @r15+,r14
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}
