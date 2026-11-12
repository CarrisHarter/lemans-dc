//! 0x8c0d5658 — classify eight 16-byte float records against ±1.
//!
//! Called with the record array base in `r4` and a three-float source vector
//! in `r5`. Eight times it clears the word at record+12, calls 0x8c0b4f34
//! (record[0..3] += vector, advancing both pointers by 8), then tests each of
//! the record's first three floats:
//!
//! * `x > 1.0f`            -> set bit 1 (offset 0), 4 (offset 4), 16 (offset 8)
//! * `-1.0f > x`            -> set bit 2 (offset 0), 8 (offset 4), 32 (offset 8)
//!
//! The `-1.0f` comes from the literal at 0x8c0d56ec, reached by `mova`; the
//! callee address at 0x8c0d56e8 is a PC-relative literal. Both branches for a
//! float are mutually exclusive, so at most one bit per half-byte is set.
//!
//! The loop counter runs 0..8 (`add #1,r13` then `cmp/gt r13,r0` with r0 = 8),
//! ending with T = 0. `fr0` ends as 1.0 if the last record's offset-8 float was
//! above 1.0, otherwise -1.0 (loaded by the else path); `fr1` ends as that
//! float. `r1` stays at the callee's record+4, `r4` at the last record base + 8,
//! and `r5` at the source vector + 8.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    // mov.l r14,@-r15 ; mov.l r13,@-r15 ; mov.l r12,@-r15 ; sts.l pr,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[14]);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[13]);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[12]);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr);

    cpu.r[12] = cpu.r[5];
    cpu.r[14] = cpu.r[4];
    cpu.r[13] = 0;

    loop {
        // mov #0,r0 ; mov.l r0,@(12,r14)
        cpu.r[0] = 0;
        let _ = mem.write_u32(cpu.r[14].wrapping_add(12), 0);
        // mov r12,r5 ; mov.l 0x8c0d56e8,r0 ; jsr @r0 ; mov r14,r4
        cpu.r[5] = cpu.r[12];
        cpu.r[0] = mem.read_u32(0x8C0D56E8).unwrap_or(0); // 0x8c0b4f34
        cpu.pr = 0x8C0D5672;
        cpu.r[4] = cpu.r[14]; // delay slot of `jsr @r0`
        crate::registry::p_8c0b4f34::run(cpu, mem);

        // fmov @r14,fr1 ; fldi1 fr0 ; fcmp/gt fr0,fr1
        check(cpu, mem, 0, 1, 2);
        // mov r14,r0 ; add #4,r0 ; fmov @r0,fr1 ; fldi1 fr0 ; fcmp/gt fr0,fr1
        check(cpu, mem, 4, 4, 8);
        // mov r14,r0 ; add #8,r0 ; fmov @r0,fr1 ; fldi1 fr0 ; fcmp/gt fr0,fr1
        check(cpu, mem, 8, 16, 32);

        // add #1,r13 ; mov #8,r0 ; cmp/gt r13,r0
        cpu.r[13] = cpu.r[13].wrapping_add(1);
        cpu.r[0] = 8;
        cpu.set_t(cpu.r[0] > cpu.r[13]);
        if !cpu.t() {
            break;
        }
        cpu.r[14] = cpu.r[14].wrapping_add(16); // delay slot of `bt.s`
    }

    // lds.l @r15+,pr ; mov.l @r15+,r12 ; mov.l @r15+,r13 ; rts ; mov.l @r15+,r14
    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[12] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[13] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[14] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}

/// One float test: `fr1 = record[off]`, `fr0 = 1.0`, `fcmp/gt fr0,fr1`;
/// if that is false, reload `fr0 = -1.0` and test `fcmp/gt fr1,fr0`.
#[inline]
fn check(cpu: &mut Cpu, mem: &mut Memory, off: u32, above_bit: u32, below_bit: u32) {
    cpu.r[0] = cpu.r[14].wrapping_add(off);
    cpu.fr[1] = mem.read_f32(cpu.r[0]).unwrap_or(0.0);
    cpu.fr[0] = 1.0;
    cpu.set_t(cpu.fr[1] > cpu.fr[0]);
    if cpu.t() {
        cpu.r[0] = mem.read_u32(cpu.r[14].wrapping_add(12)).unwrap_or(0);
        cpu.r[0] |= above_bit;
        let _ = mem.write_u32(cpu.r[14].wrapping_add(12), cpu.r[0]); // delay slot of `bra`
    } else {
        cpu.r[0] = 0x8C0D56EC; // mova @(disp,pc),r0
        cpu.fr[0] = mem.read_f32(cpu.r[0]).unwrap_or(0.0); // -1.0f
        cpu.set_t(cpu.fr[0] > cpu.fr[1]);
        if cpu.t() {
            cpu.r[0] = mem.read_u32(cpu.r[14].wrapping_add(12)).unwrap_or(0);
            cpu.r[0] |= below_bit;
            let _ = mem.write_u32(cpu.r[14].wrapping_add(12), cpu.r[0]);
        }
    }
}
