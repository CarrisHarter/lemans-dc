//! 0x8c13ec6c — free(ptr) when `ptr != NULL` and `(i16)count > 0`; return `ptr`.
//!
//! ```text
//! 8c13ec6c  mov.l r14,@-r15     push r14
//! 8c13ec6e  sts.l pr,@-r15      push pr
//! 8c13ec70  mov   r4,r14        r14 = ptr
//! 8c13ec72  tst   r14,r14
//! 8c13ec74  bt    0x8c13ec82    ptr == NULL: no free call
//! 8c13ec76  exts.w r5,r0        r0 = sext16(low half of count)
//! 8c13ec78  cmp/pl r0
//! 8c13ec7a  bf    0x8c13ec82    signed 16-bit count <= 0: no free call
//! 8c13ec7c  mov.l 0x8c13ec8c,r0 ; r0 = 0x8c01032c (free thunk)
//! 8c13ec7e  jsr   @r0 ; 8c13ec80: nop (delay slot)
//! 8c13ec82  mov   r14,r0        return ptr
//! 8c13ec84  lds.l @r15+,pr
//! 8c13ec86  rts
//! 8c13ec88  mov.l @r15+,r14     delay slot
//! ```
//!
//! Only the *low 16 bits* of r5 matter: the original tests `exts.w r5,r0`, so
//! e.g. r5 = 0xffff0001 is positive and calls free, while r5 = 0x00010000 is
//! zero and does not. The jsr sets pr to 0x8c13ec82 and its delay slot is a
//! nop; the callee (the 0x8c01032c thunk, which tail-calls free at 0x8c159fc0)
//! returns through pr. The epilogue then restores the caller's pr and r14 from
//! the frame, so both leave the routine with their seed values, and r0 ends as
//! the argument on every path. T is the callee's T after a call, `tst`'s set
//! flag on the NULL path and `cmp/pl`'s clear flag on the non-positive path.

use sh4_oracle::{Cpu, Memory};

/// 0x8c13ec8c holds 0x8c01032c, the free thunk.
const FREE_THUNK_PTR: u32 = 0x8c13ec8c;
const FREE_THUNK: u32 = 0x8c01032c;

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    // 0x8c13ec6c: mov.l r14,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[14]);
    // 0x8c13ec6e: sts.l pr,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr);
    // 0x8c13ec70: mov r4,r14
    cpu.r[14] = cpu.r[4];
    // 0x8c13ec72: tst r14,r14
    cpu.set_t(cpu.r[14] == 0);
    // 0x8c13ec74: bt 0x8c13ec82
    if cpu.r[14] != 0 {
        // 0x8c13ec76: exts.w r5,r0 — sign-extend the low 16 bits only
        cpu.r[0] = cpu.r[5] as i16 as i32 as u32;
        // 0x8c13ec78: cmp/pl r0
        cpu.set_t((cpu.r[0] as i32) > 0);
        // 0x8c13ec7a: bf 0x8c13ec82
        if (cpu.r[0] as i32) > 0 {
            // 0x8c13ec7c: mov.l 0x8c13ec8c,r0
            cpu.r[0] = mem.read_u32(FREE_THUNK_PTR).unwrap_or(FREE_THUNK);
            // 0x8c13ec7e: jsr @r0 ; 0x8c13ec80: nop (delay slot)
            cpu.pr = 0x8c13ec82;
            crate::registry::p_8c01032c::run(cpu, mem);
        }
    }
    // 0x8c13ec82: mov r14,r0
    cpu.r[0] = cpu.r[14];
    // 0x8c13ec84: lds.l @r15+,pr
    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    // 0x8c13ec86: rts ; 0x8c13ec88: mov.l @r15+,r14 (delay slot)
    cpu.r[14] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}
