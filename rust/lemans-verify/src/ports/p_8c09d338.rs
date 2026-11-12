//! 0x8c09d338 — free(ptr) if `ptr != NULL` and `(i16)count > 0`; return `ptr`.
//!
//! ```text
//! mov.l r14,@-r15     ; push r14
//! sts.l pr,@-r15      ; push pr
//! mov   r4,r14        ; r14 = ptr
//! tst   r14,r14
//! bt    0x8c09d34e    ; ptr == NULL: no free call
//! exts.w r5,r0        ; r0 = sext16(low half of count)
//! cmp/pl r0
//! bf    0x8c09d34e    ; signed 16-bit count <= 0: no free call
//! mov.l 0x8c09d358,r0 ; r0 = 0x8c01032c (free thunk)
//! jsr   @r0 ; nop
//! 0x8c09d34e: mov r14,r0
//! lds.l @r15+,pr
//! rts
//! mov.l @r15+,r14     ; delay slot
//! ```
//!
//! Only the *low 16 bits* of r5 matter: the original tests `exts.w r5,r0`, so
//! e.g. r5=0xffff0001 is positive and calls free, while r5=0x00010000 is zero
//! and does not. The jsr sets pr to 0x8c09d34e and its delay slot is a nop;
//! the callee (the 0x8c01032c thunk, which tail-calls free at 0x8c159fc0)
//! returns through pr. The epilogue then restores the caller's pr and r14 from
//! the frame, so both leave the routine with their seed values, and r0 ends as
//! the argument on every path. T is the callee's T after a call, `tst`'s set
//! flag on the NULL path and `cmp/pl`'s clear flag on the non-positive path.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    // 0x8c09d338: mov.l r14,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[14]);
    // 0x8c09d33a: sts.l pr,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr);
    // 0x8c09d33c: mov r4,r14
    cpu.r[14] = cpu.r[4];
    // 0x8c09d33e: tst r14,r14
    cpu.set_t(cpu.r[14] == 0);
    // 0x8c09d340: bt 0x8c09d34e
    if cpu.r[14] != 0 {
        // 0x8c09d342: exts.w r5,r0 — sign-extend the low 16 bits only
        cpu.r[0] = cpu.r[5] as i16 as i32 as u32;
        // 0x8c09d344: cmp/pl r0
        cpu.set_t((cpu.r[0] as i32) > 0);
        // 0x8c09d346: bf 0x8c09d34e
        if (cpu.r[0] as i32) > 0 {
            // 0x8c09d348: mov.l 0x8c09d358,r0
            cpu.r[0] = mem.read_u32(0x8c09d358).unwrap_or(0x8c01032c);
            // 0x8c09d34a: jsr @r0 ; 0x8c09d34c: nop (delay slot)
            cpu.pr = 0x8c09d34e;
            crate::registry::p_8c01032c::run(cpu, mem);
        }
    }
    // 0x8c09d34e: mov r14,r0
    cpu.r[0] = cpu.r[14];
    // 0x8c09d350: lds.l @r15+,pr
    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    // 0x8c09d352: rts ; 0x8c09d354: mov.l @r15+,r14 (delay slot)
    cpu.r[14] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}
