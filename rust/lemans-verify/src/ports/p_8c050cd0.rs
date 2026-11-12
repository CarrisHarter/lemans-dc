//! 0x8c050cd0 — split a 3-vector delta into ground-plane components.
//!
//! `base = *(r4+8)`. 0x8c0b4f5c subtracts the vectors `*(base+4)` and `*(r5)`
//! into a 12-byte stack local; 0x8c0b2c00 then returns `len = |d.x,d.y|` as a
//! float. `d.x,d.y` are divided by `len`, or set to 0 with `d.z = 1` when
//! `len == 0`. The two plane coefficients at `base+20` and `base+24` are
//! combined with the unit delta into `cos`:
//!
//! * `cos > 0.9999` -> `*r6 = len`, fr0 = 0
//! * else `v = len*cos`, `*r6 = v`; if `v > len` -> `*r6 = len`, fr0 = 0;
//!   otherwise fr0 = `sqrt(len^2 - v^2)`.
//!
//! The `v > len` arm is unreachable in IEEE-754 single precision: with
//! `cos <= 0.9999`, `len*cos` can round to `len` but never above it, so it can
//! never satisfy `v > len`. It is transcribed anyway. The equal-length arm
//! leaves `*r6` written twice (`len*cos`, then `len`); the redundant stores are
//! effectful and are reproduced.
//!
//! Register residue: r0 = 0x8c050d8c (mova literal), r1 = base+20,
//! r2 = frame+12, r3 = base+24, r4 = frame+4, r5 = base+4, r6 = base+12,
//! r7 = r5_entry+8, r13/r14/r15/pr restored from the frame.

use sh4_oracle::{Cpu, Memory};

/// `[0x8c050d8c]` — 0.9999f, the guard compared against the plane dot product.
const DOT_GUARD: f32 = f32::from_bits(0x3f7f_f972);

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    // mov.l r14,@-r15 ; mov.l r13,@-r15 ; sts.l pr,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[14]);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[13]);
    cpu.r[14] = cpu.r[6]; // mov r6,r14
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr);
    cpu.r[13] = cpu.r[4]; // mov r4,r13
    cpu.r[15] = cpu.r[15].wrapping_sub(20); // add #-20,r15
    cpu.r[4] = cpu.r[5]; // mov r5,r4 — callee 1 gets the second vector as `a`
    cpu.r[0] = cpu.r[15].wrapping_add(8); // mov r15,r0 ; add #8,r0
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[0]); // mov.l r0,@-r15 — dst of the subtract
    cpu.r[5] = mem.read_u32(cpu.r[13].wrapping_add(8)).unwrap_or(0); // mov.l @(8,r13),r5
    cpu.pr = 0x8c050cec; // jsr @r0 -> 0x8c0b4f5c
    cpu.r[5] = cpu.r[5].wrapping_add(4); // delay slot: add #4,r5
    crate::registry::p_8c0b4f5c::run(cpu, mem);
    cpu.r[15] = cpu.r[15].wrapping_add(4); // add #4,r15

    cpu.r[0] = 8;
    cpu.fr[1] = mem.read_f32(cpu.r[15].wrapping_add(8)).unwrap_or(0.0); // fmov @(8,r15),fr1
    cpu.r[2] = 4;
    cpu.r[2] = cpu.r[2].wrapping_add(cpu.r[15]);
    cpu.r[0] = 12;
    cpu.fr[0] = mem.read_f32(cpu.r[15].wrapping_add(12)).unwrap_or(0.0); // fmov @(12,r15),fr0
    cpu.r[4] = cpu.r[15]; // mov r15,r4
    cpu.r[0] = 0;
    let _ = mem.write_f32(cpu.r[15], cpu.fr[1]); // fmov fr1,@(0,r15)
    cpu.pr = 0x8c050d06; // jsr @r0 -> 0x8c0b2c00
    let _ = mem.write_f32(cpu.r[2], cpu.fr[0]); // delay slot: fmov fr0,@r2
    crate::registry::p_8c0b2c00::run(cpu, mem);

    cpu.fr[2] = 0.0; // fldi0 fr2
    let len_zero = cpu.fr[0] == cpu.fr[2]; // fcmp/eq fr2,fr0
    cpu.set_t(len_zero);
    if len_zero {
        // bt 0x8c050d26 — degenerate x/y delta: zero both, force z to 1
        cpu.r[0] = 8;
        let _ = mem.write_f32(cpu.r[15].wrapping_add(8), cpu.fr[2]);
        cpu.fr[1] = 1.0; // fldi1 fr1
        cpu.r[0] = 12;
        let _ = mem.write_f32(cpu.r[15].wrapping_add(12), cpu.fr[2]);
        cpu.r[0] = 16;
        let _ = mem.write_f32(cpu.r[15].wrapping_add(16), cpu.fr[1]);
    } else {
        cpu.r[0] = 8;
        cpu.fr[1] = mem.read_f32(cpu.r[15].wrapping_add(8)).unwrap_or(0.0);
        cpu.fr[2] = 1.0; // fldi1 fr2
        cpu.fr[2] = cpu.fr[2] / cpu.fr[0]; // fdiv fr0,fr2
        cpu.fr[1] = cpu.fr[1] * cpu.fr[2]; // fmul fr2,fr1
        cpu.r[0] = 8;
        let _ = mem.write_f32(cpu.r[15].wrapping_add(8), cpu.fr[1]);
        cpu.r[0] = 12;
        cpu.fr[1] = mem.read_f32(cpu.r[15].wrapping_add(12)).unwrap_or(0.0);
        cpu.fr[1] = cpu.fr[1] * cpu.fr[2];
        cpu.r[0] = 12;
        let _ = mem.write_f32(cpu.r[15].wrapping_add(12), cpu.fr[1]); // bra delay slot
    }

    // 0x8c050d34 — join
    cpu.r[3] = mem.read_u32(cpu.r[13].wrapping_add(8)).unwrap_or(0);
    cpu.r[2] = 8;
    cpu.r[2] = cpu.r[2].wrapping_add(cpu.r[15]);
    cpu.r[1] = cpu.r[3];
    cpu.fr[1] = mem.read_f32(cpu.r[2]).unwrap_or(0.0); // fmov @r2,fr1
    cpu.r[3] = cpu.r[3].wrapping_add(24);
    cpu.r[1] = cpu.r[1].wrapping_add(20);
    cpu.fr[4] = mem.read_f32(cpu.r[1]).unwrap_or(0.0); // fmov @r1,fr4
    cpu.r[2] = 12;
    cpu.r[2] = cpu.r[2].wrapping_add(cpu.r[15]);
    cpu.fr[3] = mem.read_f32(cpu.r[3]).unwrap_or(0.0); // fmov @r3,fr3
    cpu.r[0] = 0x8c050d8c; // mova 0x8c050d8c,r0
    cpu.fr[2] = mem.read_f32(cpu.r[2]).unwrap_or(0.0); // fmov @r2,fr2
    cpu.fr[4] = cpu.fr[4] * cpu.fr[1]; // fmul fr1,fr4
    cpu.fr[1] = DOT_GUARD; // fmov @r0,fr1
    cpu.fr[3] = cpu.fr[3] * cpu.fr[2]; // fmul fr2,fr3
    cpu.fr[4] = cpu.fr[4] + cpu.fr[3]; // fadd fr3,fr4
    let over = cpu.fr[4] > cpu.fr[1]; // fcmp/gt fr1,fr4
    cpu.set_t(over);
    if over {
        // bf not taken
        let _ = mem.write_f32(cpu.r[14], cpu.fr[0]); // fmov fr0,@r14
        cpu.fr[0] = 0.0; // bra delay slot: fldi0 fr0
    } else {
        cpu.fr[1] = cpu.fr[0]; // fmov fr0,fr1
        cpu.fr[1] = cpu.fr[1] * cpu.fr[4]; // fmul fr4,fr1
        let _ = mem.write_f32(cpu.r[14], cpu.fr[1]); // fmov fr1,@r14
        cpu.fr[1] = mem.read_f32(cpu.r[14]).unwrap_or(0.0); // fmov @r14,fr1
        let clamped = cpu.fr[1] > cpu.fr[0]; // fcmp/gt fr0,fr1
        cpu.set_t(clamped);
        if clamped {
            let _ = mem.write_f32(cpu.r[14], cpu.fr[0]); // fmov fr0,@r14
            cpu.fr[0] = 0.0; // bra delay slot: fldi0 fr0
        } else {
            cpu.fr[0] = cpu.fr[0] * cpu.fr[0]; // fmul fr0,fr0
            cpu.fr[1] = cpu.fr[1] * cpu.fr[1]; // fmul fr1,fr1
            cpu.fr[0] = cpu.fr[0] - cpu.fr[1]; // fsub fr1,fr0
            cpu.fr[0] = cpu.fr[0].sqrt(); // fsqrt fr0
        }
    }

    // 0x8c050d7a — epilogue
    cpu.r[15] = cpu.r[15].wrapping_add(20); // add #20,r15
    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0); // lds.l @r15+,pr
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[13] = mem.read_u32(cpu.r[15]).unwrap_or(0); // mov.l @r15+,r13
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[14] = mem.read_u32(cpu.r[15]).unwrap_or(0); // rts delay slot: mov.l @r15+,r14
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}
