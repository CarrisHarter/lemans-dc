//! 0x8c0a8d48 — install an inverted cubic ramp in a 128-float object field.
//!
//! r4 is the object base; fr4..fr7 carry (start, end, from, to). The 512-byte
//! table at base+0x4c is cleared through the memset port 0x8c010f88, a cubic
//! segment `from + t³*(to-from)` (t = (i-fr0)/(fr1-fr0)) is painted for every
//! integer i in `[trunc(fr0), fr1)`, indices from `trunc(fr1)` to 127 become
//! 1.0, and finally every entry is replaced by `1.0 - entry`. The clamped
//! start/end and the two levels are latched at base+0x38/0x3c/0x40/0x44 and
//! the byte at base+0x24c is set to 1.
//!
//! ```text
//! fr1 = max(fr4,0) ; fr0 = min(128,fr1) ; fr13 = max(fr5,fr0)
//! fr1 = min(128,fr13) ; memset(base+0x4c,0,0x200)
//! i0 = trunc(fr0)
//! for i = i0 while fr1 > (float)i :  table[i] = fr6 + t³*(fr7-fr6)
//! for i = trunc(fr1) while 128 > i : table[i] = 1.0
//! for every entry                  : table[i] = 1.0 - table[i]
//! ```
//!
//! Delay-slot notes (the oracle runs the slot instruction even when `bt.s`
//! is not taken): the `add #4,r4` / `add #4,r5` slots of both table loops and
//! the second `fmov.s` of the complement loop therefore execute once per
//! iteration, including the last. fr12..fr15 are callee-saved and come back
//! from the pushes; r15 is unchanged on exit.

use sh4_oracle::{Cpu, Memory};

const TABLE: u32 = 0x4C; // base+0x4c: 128 single-precision entries
const LEN: u32 = 0x80; // 128 entries, the `cmp/gt` bound and the loop constant
const CLEAR: u32 = 0x200; // 128 * 4 bytes for the memset call

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let base = cpu.r[4];
    let (a, b, c, d) = (cpu.fr[4], cpu.fr[5], cpu.fr[6], cpu.fr[7]);

    // Prologue pushes: fr15..fr12, r14, pr (24 bytes; the stack window is not
    // part of the compared state, but the pops below restore the same values).
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_f32(cpu.r[15], cpu.fr[15]);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_f32(cpu.r[15], cpu.fr[14]);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_f32(cpu.r[15], cpu.fr[13]);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_f32(cpu.r[15], cpu.fr[12]);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[14]);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr);

    cpu.r[14] = base; // mov r4,r14
    cpu.fr[13] = b; // fmov fr5,fr13
    cpu.fr[12] = a; // fmov fr4,fr12
    cpu.fr[14] = c; // fmov fr6,fr14
    cpu.fr[15] = d; // fmov fr7,fr15

    // 8c0a8d5e..66 — memset(base+0x4c, 0, 0x200)
    cpu.r[6] = CLEAR;
    cpu.r[4] = base.wrapping_add(TABLE);
    cpu.r[0] = 0x8c010f88;
    cpu.r[5] = 0; // delay slot of `jsr`
    cpu.pr = 0x8c0a8d68;
    crate::registry::p_8c010f88::run(cpu, mem);

    // 8c0a8d68 — fr1 = max(fr4,0): `fcmp/gt fr1,fr12` is false for NaN and
    // for a<=0, leaving the fldi0 zero in place.
    let mut fr1 = 0.0f32;
    if a > fr1 {
        fr1 = a; // 8c0a8d72
    }
    // 8c0a8d74 — fr0 = min(128.0, fr1)
    let mut fr0 = 128.0f32; // mova 0x8c0a8e58
    if fr0 > fr1 {
        fr0 = fr1; // 8c0a8d80
    }
    // 8c0a8d82 — fr13 = max(fr5, fr0)
    let mut fr13 = b;
    if !(b > fr0) {
        fr13 = fr0; // 8c0a8d86
    }
    // 8c0a8d88 — fr1 = min(128.0, fr13)
    fr1 = 128.0; // mova 0x8c0a8e58
    if fr1 > fr13 {
        fr1 = fr13; // 8c0a8d94
    }

    // 8c0a8d96 — i0 = trunc(fr0), broadcast through fpul
    cpu.fpul = (fr0 as i32) as u32; // ftrc fr0,fpul
    let mut r3 = cpu.fpul; // sts fpul,r3
    cpu.fpul = r3; // lds r3,fpul
    let mut fr2 = (cpu.fpul as i32) as f32; // float fpul,fr2

    let mut r0 = cpu.r[0]; // memset leaves its destination in r0
    let mut r4 = cpu.r[4]; // still base+0x4c
    let mut r5 = cpu.r[5]; // 0 from the `jsr` delay slot
    let mut fr3 = cpu.fr[3];
    let mut fr4 = a;
    let mut fr5 = b;

    // 8c0a8d9e..dd4 — cubic ramp for every integer below fr1
    if fr1 > fr2 {
        r4 = r3 << 2; // mov r3,r4 ; shll2 r4
        fr4 = fr1; // fmov fr1,fr4
        fr5 = d; // fmov fr15,fr5
        fr4 = fr4 - fr0; // fsub fr0,fr4
        fr5 = fr5 - c; // fsub fr14,fr5
        let r1 = base.wrapping_add(TABLE);
        loop {
            cpu.fpul = r3; // lds r3,fpul
            r0 = r1.wrapping_add(r4); // mov r1,r0 ; add r4,r0
            fr2 = (cpu.fpul as i32) as f32; // float fpul,fr2
            fr2 = fr2 - fr0; // fsub fr0,fr2
            fr2 = fr2 / fr4; // fdiv fr4,fr2
            fr3 = fr2; // fmov fr2,fr3
            fr3 = fr3 * fr2; // fmul fr2,fr3
            fr3 = fr3 * fr2; // fmul fr2,fr3
            fr3 = fr3 * fr5; // fmul fr5,fr3
            r3 = r3.wrapping_add(1); // add #1,r3
            fr3 = fr3 + c; // fadd fr14,fr3
            cpu.fpul = r3; // lds r3,fpul
            let _ = mem.write_f32(r0, fr3); // fmov fr3,@r0
            fr2 = (cpu.fpul as i32) as f32; // float fpul,fr2
            r4 = r4.wrapping_add(4); // delay slot / fall-through of `bt.s`
            if !(fr1 > fr2) {
                break; // bt.s 0x8c0a8db2 not taken
            }
        }
    }

    // 8c0a8dd6 — pad from trunc(fr1) to 127 with 1.0
    cpu.fpul = (fr1 as i32) as u32; // ftrc fr1,fpul
    r3 = LEN; // mov.l 0x8c0a8e5c
    r4 = cpu.fpul; // sts fpul,r4
    if (r3 as i32) > (r4 as i32) {
        let r1 = base.wrapping_add(TABLE);
        r5 = r4 << 2;
        fr2 = 1.0; // fldi1 fr2
        loop {
            r0 = r1.wrapping_add(r5); // mov r1,r0 ; add r5,r0
            r4 = r4.wrapping_add(1); // add #1,r4
            let _ = mem.write_f32(r0, fr2); // fmov fr2,@r0
            r5 = r5.wrapping_add(4); // delay slot / fall-through of `bt.s`
            if !((r3 as i32) > (r4 as i32)) {
                break; // bt.s 0x8c0a8dea not taken
            }
        }
    }

    // 8c0a8df8..e20 — invert the whole table
    r4 = 0;
    r5 = 0;
    r0 = LEN;
    r3 = base.wrapping_add(TABLE);
    fr4 = 1.0; // fldi1 fr4
    loop {
        let r1 = r3.wrapping_add(r5); // mov r3,r1 ; add r5,r1
        fr3 = mem.read_f32(r1).unwrap_or(0.0); // fmov @r1,fr3
        fr2 = fr4; // fmov fr4,fr2
        r4 = r4.wrapping_add(2); // add #2,r4
        fr2 = fr2 - fr3; // fsub fr3,fr2
        let _ = mem.write_f32(r1, fr2); // fmov fr2,@r1
        let next = r1.wrapping_add(4); // add #4,r1
        fr3 = mem.read_f32(next).unwrap_or(0.0); // fmov @r1,fr3
        fr2 = fr4; // fmov fr4,fr2
        r5 = r5.wrapping_add(8); // add #8,r5
        fr2 = fr2 - fr3; // fsub fr3,fr2
        // The `bt.s` slot stores the second entry even when the branch is not
        // taken, so the last pair is complemented as well.
        let _ = mem.write_f32(next, fr2); // fmov fr2,@r1 (delay slot)
        if !((r0 as i32) > (r4 as i32)) {
            break; // bt.s 0x8c0a8e04 not taken
        }
    }

    // 8c0a8e22..3e — latch the control points and the ready byte
    r0 = base.wrapping_add(0x38);
    r3 = base.wrapping_add(0x3c);
    let _ = mem.write_f32(r0, fr0); // fmov fr0,@r0
    let _ = mem.write_f32(r3, fr1); // fmov fr1,@r3
    let _ = mem.write_f32(base.wrapping_add(0x40), c); // fmov fr14,@r1
    let _ = mem.write_f32(base.wrapping_add(0x44), d); // fmov fr15,@r0
    let _ = mem.write_u8(base.wrapping_add(0x24C), 1); // mov.b r1,@(r0,r14)

    // Epilogue pops, `rts`, and the delay-slot fr15 pop.
    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0); // lds.l @r15+,pr
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[14] = mem.read_u32(cpu.r[15]).unwrap_or(0); // mov.l @r15+,r14
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.fr[12] = mem.read_f32(cpu.r[15]).unwrap_or(0.0); // fmov @r15+,fr12
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.fr[13] = mem.read_f32(cpu.r[15]).unwrap_or(0.0); // fmov @r15+,fr13
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.fr[14] = mem.read_f32(cpu.r[15]).unwrap_or(0.0); // fmov @r15+,fr14
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.fr[15] = mem.read_f32(cpu.r[15]).unwrap_or(0.0); // delay slot of `rts`
    cpu.r[15] = cpu.r[15].wrapping_add(4);

    // Register file on return (the tail's address arithmetic is what sticks).
    cpu.r[0] = 0x24C; // mov.l 0x8c0a8e60
    cpu.r[1] = 1; // mov #1,r1
    cpu.r[3] = base.wrapping_add(0x3C);
    cpu.r[4] = LEN;
    cpu.r[5] = LEN * 4; // 0x200: 64 complement iterations of `add #8,r5`
    cpu.r[6] = CLEAR;
    cpu.r[7] = CLEAR; // memset's remaining count
    cpu.fr[0] = fr0;
    cpu.fr[1] = fr1;
    cpu.fr[2] = fr2;
    cpu.fr[3] = fr3;
    cpu.fr[4] = fr4;
    cpu.fr[5] = fr5;
    // fr6/fr7 (the level arguments) are only read, never written.
    cpu.fpul = (fr1 as i32) as u32; // ftrc fr1,fpul at 8c0a8dd6
    cpu.set_t(false); // last `cmp/gt r4,r0` (128 > 128) is false on exit
}
