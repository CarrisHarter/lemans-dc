//! 0x8c084328 — insert a 32-byte record into an object's indexed table and
//! raise the object's tracked maximum.
//!
//! Arguments: r4 = object, r5 = source record (8 words), r6 = table index
//! (only the low 16 bits are used).
//!
//! CFG (single conditional branch; everything else is straight line):
//! ```text
//! table = *(u32*)(r4+24)
//! slot  = table + (r6 & 0xffff)*32
//! src[0..8] -> slot                         (word by word, ascending)
//! *(u8*)(table + (r6 & 0xffff)*32) = 1      (table re-read after the copy)
//! d = |src.f[16] - src.f[8]|                (call 0x8c0b2780)
//! if d > *(f32*)(r4+44)                     (fcmp/gt ; bf 0x8c084374)
//!     *(f32*)(r4+44) = d
//! ```
//!
//! Both the prologue and the frame scratch that feeds `fr4`/`fr0` are real
//! stores, so they are reproduced: the frame keeps src[4..16] at
//! sp+0..sp+12 and the two floats are loaded back from sp+4 (src[16]) and
//! sp+12 (src[8]).
//!
//! Side effects the decompiled C hides:
//! - The frame is 32 bytes (four pushes plus `add #-16,r15`) and every push
//!   and pop is a stack store/load; r15 returns to its entry value.
//! - `0x8c084364 jsr @r0` calls 0x8c0b2780 with the return address
//!   0x8c084368 in pr and its delay slot already applied: `fsub fr0,fr4`
//!   leaves the difference in fr4, and the callee returns |fr4| in fr0.
//! - `fcmp/gt fr1,fr0` selects the store (T = fr0 > fr1), but the later
//!   `shll r0` rewrites T with its shifted-out bit. r0 is at most
//!   0xffff<<4 there, so its bit 31 is clear and T returns 0 on both paths;
//!   only the step count witnesses which side of `bf` ran.
//! - fr1 keeps the old r4+44 float, fr4 the signed difference
//!   src[16]-src[8], and fr0 its absolute value from the callee.
//! - The copy reloads `*(u32*)(r4+24)` for the validity byte, so a table
//!   pointer that the copy itself overwrote redirects that byte.
//! - r0 ends as (r6 & 0xffff)*32, r1 as 1, r2 as entry-sp-20, r3 as the
//!   reloaded table pointer, r4 as src[24] and r5 as table+index*32; r12,
//!   r13 and r14 are restored from the frame to r4, r5 and r6.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    // 0x8c084328 prologue: push r14,r13,r12,pr, open a 16-byte scratch area,
    // and keep the three arguments in r14/r13/r12.
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
    cpu.r[15] = cpu.r[15].wrapping_sub(16);
    cpu.r[12] = cpu.r[4];

    // 0x8c084338..0x8c08435a: copy src[4],src[8] to sp+8/sp+12 and
    // src[12],src[16] to sp+0/sp+4.
    cpu.r[0] = cpu.r[13].wrapping_add(4);
    cpu.r[3] = mem.read_u32(cpu.r[0]).unwrap_or(0);
    cpu.r[5] = cpu.r[15];
    cpu.r[1] = cpu.r[13];
    cpu.r[0] = mem.read_u32(cpu.r[0].wrapping_add(4)).unwrap_or(0);
    cpu.r[5] = cpu.r[5].wrapping_add(8);
    cpu.r[2] = 4;
    let _ = mem.write_u32(cpu.r[5], cpu.r[3]);
    cpu.r[1] = cpu.r[1].wrapping_add(12);
    let _ = mem.write_u32(cpu.r[5].wrapping_add(4), cpu.r[0]);
    cpu.r[4] = cpu.r[15];
    cpu.r[3] = mem.read_u32(cpu.r[1]).unwrap_or(0);
    cpu.r[2] = cpu.r[2].wrapping_add(cpu.r[15]);
    cpu.r[0] = 0x8c0b2780;
    cpu.r[1] = mem.read_u32(cpu.r[1].wrapping_add(4)).unwrap_or(0);
    let _ = mem.write_u32(cpu.r[4], cpu.r[3]);
    let _ = mem.write_u32(cpu.r[4].wrapping_add(4), cpu.r[1]);

    // 0x8c08435c..0x8c084362: fr4 = src[16], fr0 = src[8] through the frame.
    cpu.fr[4] = mem.read_f32(cpu.r[2]).unwrap_or(0.0);
    cpu.r[2] = 12;
    cpu.r[2] = cpu.r[2].wrapping_add(cpu.r[15]);
    cpu.fr[0] = mem.read_f32(cpu.r[2]).unwrap_or(0.0);

    // 0x8c084364 jsr @r0 (0x8c0b2780) ; 0x8c084366 fsub fr0,fr4
    cpu.pr = 0x8c084368;
    cpu.fr[4] = cpu.fr[4] - cpu.fr[0];
    crate::registry::p_8c0b2780::run(cpu, mem);

    // 0x8c084368 mov r12,r0 ; add #44,r0 ; fmov @r0,fr1 ; fcmp/gt ; bf
    cpu.r[0] = cpu.r[12].wrapping_add(44);
    cpu.fr[1] = mem.read_f32(cpu.r[0]).unwrap_or(0.0);
    cpu.set_t(cpu.fr[0] > cpu.fr[1]);
    if cpu.t() {
        // 0x8c084372 fmov fr0,@r0 — keep the larger magnitude.
        let _ = mem.write_f32(cpu.r[0], cpu.fr[0]);
    }

    // 0x8c084374..0x8c084380: slot = *(u32*)(r4+24) + (r6 & 0xffff)*32.
    // `shll` (unlike the two `shll2`s) rewrites T with the shifted-out bit,
    // so the final flag is this shift's, not the fcmp's: r0 is at most
    // 0xffff<<4 here, hence T ends 0 on both paths.
    cpu.r[0] = cpu.r[14] & 0xffff;
    cpu.r[0] = cpu.r[0] << 2;
    cpu.r[0] = cpu.r[0] << 2;
    cpu.r[5] = mem.read_u32(cpu.r[12].wrapping_add(24)).unwrap_or(0);
    let out = cpu.r[0] >> 31 != 0;
    cpu.r[0] = cpu.r[0] << 1;
    cpu.set_t(out);
    cpu.r[4] = mem.read_u32(cpu.r[13]).unwrap_or(0);
    cpu.r[5] = cpu.r[5].wrapping_add(cpu.r[0]);
    cpu.r[3] = mem.read_u32(cpu.r[13].wrapping_add(4)).unwrap_or(0);
    cpu.r[1] = 1;
    let _ = mem.write_u32(cpu.r[5], cpu.r[4]);
    let _ = mem.write_u32(cpu.r[5].wrapping_add(4), cpu.r[3]);

    // 0x8c08438a..0x8c0843a0: the rest of the 32-byte record.
    cpu.r[4] = mem.read_u32(cpu.r[13].wrapping_add(8)).unwrap_or(0);
    cpu.r[3] = mem.read_u32(cpu.r[13].wrapping_add(12)).unwrap_or(0);
    let _ = mem.write_u32(cpu.r[5].wrapping_add(8), cpu.r[4]);
    let _ = mem.write_u32(cpu.r[5].wrapping_add(12), cpu.r[3]);
    cpu.r[4] = mem.read_u32(cpu.r[13].wrapping_add(16)).unwrap_or(0);
    cpu.r[3] = mem.read_u32(cpu.r[13].wrapping_add(20)).unwrap_or(0);
    let _ = mem.write_u32(cpu.r[5].wrapping_add(16), cpu.r[4]);
    let _ = mem.write_u32(cpu.r[5].wrapping_add(20), cpu.r[3]);
    cpu.r[4] = mem.read_u32(cpu.r[13].wrapping_add(24)).unwrap_or(0);
    cpu.r[3] = mem.read_u32(cpu.r[13].wrapping_add(28)).unwrap_or(0);
    let _ = mem.write_u32(cpu.r[5].wrapping_add(24), cpu.r[4]);
    let _ = mem.write_u32(cpu.r[5].wrapping_add(28), cpu.r[3]);

    // 0x8c0843a2 mov.l @(24,r12),r3 ; 0x8c0843a4 mov.b r1,@(r0,r3)
    cpu.r[3] = mem.read_u32(cpu.r[12].wrapping_add(24)).unwrap_or(0);
    let _ = mem.write_u8(cpu.r[0].wrapping_add(cpu.r[3]), cpu.r[1] as u8);

    // 0x8c0843a6..0x8c0843b0: close the frame and return. The `rts` at
    // 0x8c0843ae takes its delay slot at 0x8c0843b0, which pops r14.
    cpu.r[15] = cpu.r[15].wrapping_add(16);
    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[12] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[13] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[14] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}
