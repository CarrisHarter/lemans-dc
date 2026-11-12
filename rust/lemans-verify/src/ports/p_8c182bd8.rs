//! 0x8c182bd8 — free(ptr) if `ptr != NULL` and `(i16)count > 0`; return `ptr`.
//!
//! ```text
//! mov.l r14,@-r15     ; push r14
//! sts.l pr,@-r15      ; push pr
//! mov   r4,r14        ; r14 = ptr
//! tst   r14,r14
//! bt    0x8c182bee    ; ptr == NULL: no free call
//! exts.w r5,r0        ; r0 = sext16(low half of count)
//! cmp/pl r0
//! bf    0x8c182bee    ; signed 16-bit count <= 0: no free call
//! mov.l 0x8c182bf8,r0 ; r0 = 0x8c01032c (free thunk)
//! jsr   @r0 ; nop
//! 0x8c182bee: mov r14,r0
//! lds.l @r15+,pr
//! rts
//! mov.l @r15+,r14     ; delay slot
//! ```
//!
//! Twin of 0x8c182bb4: same semantics, own literal pool at 0x8c182bf8 and jsr
//! return pr 0x8c182bee. Only the *low 16 bits* of r5 matter (`exts.w r5,r0`).
//! The jsr's delay slot is a nop, so free receives r4 unchanged; the callee
//! (the 0x8c01032c thunk, which tail-calls free at 0x8c159fc0) returns through
//! pr. The epilogue restores the caller's pr and r14, so both leave with their
//! seed values, and r0 ends as the argument on every path. T is the callee's T
//! after a call, `tst`'s set flag on the NULL path and `cmp/pl`'s clear flag
//! on the non-positive path.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    // 0x8c182bd8: mov.l r14,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[14]);
    // 0x8c182bda: sts.l pr,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr);
    // 0x8c182bdc: mov r4,r14
    cpu.r[14] = cpu.r[4];
    // 0x8c182bde: tst r14,r14
    cpu.set_t(cpu.r[14] == 0);
    // 0x8c182be0: bt 0x8c182bee
    if cpu.r[14] != 0 {
        // 0x8c182be2: exts.w r5,r0 — sign-extend the low 16 bits only
        cpu.r[0] = cpu.r[5] as i16 as i32 as u32;
        // 0x8c182be4: cmp/pl r0
        cpu.set_t((cpu.r[0] as i32) > 0);
        // 0x8c182be6: bf 0x8c182bee
        if (cpu.r[0] as i32) > 0 {
            // 0x8c182be8: mov.l 0x8c182bf8,r0
            cpu.r[0] = mem.read_u32(0x8c182bf8).unwrap_or(0x8c01032c);
            // 0x8c182bea: jsr @r0 ; 0x8c182bec: nop (delay slot)
            cpu.pr = 0x8c182bee;
            crate::registry::p_8c01032c::run(cpu, mem);
        }
    }
    // 0x8c182bee: mov r14,r0
    cpu.r[0] = cpu.r[14];
    // 0x8c182bf0: lds.l @r15+,pr
    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    // 0x8c182bf2: rts ; 0x8c182bf4: mov.l @r15+,r14 (delay slot)
    cpu.r[14] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}
