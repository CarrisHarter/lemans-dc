//! 0x8c018cf0 — free a block only when the pointer is non-null and the
//! signed word count is positive.
//!
//! ```text
//! mov.l r14,@-r15 ; sts.l pr,@-r15 ; mov r4,r14
//! tst r14,r14 ; bt 0x8c018d06
//! exts.w r5,r0 ; cmp/pl r0 ; bf 0x8c018d06
//! mov.l 0x8c018d10,r0 ; jsr @r0 ; nop
//! 0x8c018d06: mov r14,r0 ; lds.l @r15+,pr ; rts ; mov.l @r15+,r14
//! ```
//!
//! Two independent guards decide the call. The null test uses the full dword
//! pointer in `r4`, so `tst` leaves T set only on the null path. The count is
//! the *low half* of `r5`: `exts.w` sign-extends it into `r0`, and `cmp/pl`
//! branches on the sign of that 32-bit result, so a negative int32 `r5` whose
//! low word is 1 still frees. The call target is read from the literal pool at
//! 0x8c018d10 (0x8c01032c, the thunk to `free` at 0x8c159fc0) with `r4` and
//! `r5` untouched; the callee's register residue, T and stores pass through.
//! On both early exits the caller's T comes from its own last compare.
//!
//! The return value is always the original `r4`: `r0` is reloaded from the
//! live `r14` (still the `mov r4,r14` copy) before the epilogue restores the
//! caller's `r14` from the stack, so the callee's `r0` never survives.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[15] = cpu.r[15].wrapping_sub(4); // mov.l r14,@-r15
    let _ = mem.write_u32(cpu.r[15], cpu.r[14]);
    cpu.r[15] = cpu.r[15].wrapping_sub(4); // sts.l pr,@-r15
    let _ = mem.write_u32(cpu.r[15], cpu.pr);

    cpu.r[14] = cpu.r[4]; // mov r4,r14
    cpu.set_t(cpu.r[14] == 0); // tst r14,r14
    if !cpu.t() {
        // bt 0x8c018d06 was not taken.
        cpu.r[0] = (cpu.r[5] as u16 as i16) as i32 as u32; // exts.w r5,r0
        cpu.set_t((cpu.r[0] as i32) > 0); // cmp/pl r0
        if cpu.t() {
            // bf 0x8c018d06 was not taken.
            cpu.r[0] = mem.read_u32(0x8c018d10).unwrap_or(0x8c01032c); // mov.l @(0x8c018d10,pc),r0
            cpu.pr = 0x8c018d06; // jsr @r0 returns past its delay slot
            // delay slot 0x8c018d04: nop
            crate::registry::p_8c01032c::run(cpu, mem);
        }
    }

    // 0x8c018d06
    cpu.r[0] = cpu.r[14]; // mov r14,r0
    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0); // lds.l @r15+,pr
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[14] = mem.read_u32(cpu.r[15]).unwrap_or(0); // rts delay: mov.l @r15+,r14
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}
