//! 0x8c07fd48 — guarded `free`: free a block only when the pointer is non-null
//! and the `short` second argument is positive.
//!
//! ```text
//! mov.l r14,@-r15
//! sts.l pr,@-r15
//! mov   r4,r14          ; save the pointer argument
//! tst   r14,r14
//! bt    0x8c07fd5e      ; null -> return it untouched
//! exts.w r5,r0          ; only the low 16 bits of arg2 are tested
//! cmp/pl r0
//! bf    0x8c07fd5e      ; arg2 <= 0 -> skip the call
//! mov.l 0x8c07fd68,r0   ; r0 = 0x8c01032c (free thunk)
//! jsr   @r0
//! nop
//! 0x8c07fd5e:
//! mov   r14,r0
//! lds.l @r15+,pr
//! rts
//! mov.l @r15+,r14
//! ```
//!
//! The return value is always the `r4` argument (`r14`), whether or not the
//! call happened. `exts.w` sign-extends only the low half of `r5` before the
//! signed compare, so e.g. `r5=0xffff0001` still frees. The call delegates to
//! the ported thunk, which tail-calls `free`; the callee's whole register
//! residue and T flag survive, except `r0`, which is overwritten by `r14`.
//! `r14`/`pr` are callee-saved and restored from the stack.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    // mov.l r14,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[14]);
    // sts.l pr,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr);
    // mov r4,r14
    cpu.r[14] = cpu.r[4];
    // tst r14,r14
    cpu.set_t(cpu.r[14] == 0);
    if !cpu.t() {
        // exts.w r5,r0
        cpu.r[0] = (cpu.r[5] as u16 as i16) as i32 as u32;
        // cmp/pl r0
        cpu.set_t((cpu.r[0] as i32) > 0);
        if cpu.t() {
            // mov.l 0x8c07fd68,r0
            cpu.r[0] = mem.read_u32(0x8c07fd68).unwrap_or(0x8c01032c);
            // jsr @r0 sets pr to the instruction after the delay slot
            cpu.pr = 0x8c07fd5e;
            // jsr @r0 ; nop
            crate::registry::p_8c01032c::run(cpu, mem);
        }
    }
    // mov r14,r0
    cpu.r[0] = cpu.r[14];
    // lds.l @r15+,pr
    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    // rts ; delay slot: mov.l @r15+,r14
    cpu.r[14] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}
