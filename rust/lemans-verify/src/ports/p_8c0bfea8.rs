//! 0x8c0bfea8 — free a block when it is non-null and `r5`'s low half is positive.
//!
//! ```text
//! mov.l r14,@-r15                     ; save r14
//! sts.l pr,@-r15                      ; save pr
//! mov   r4,r14                        ; r14 = block
//! tst   r14,r14   / bt   0x8c0bfebe   ; null block exits before the sign check
//! exts.w r5,r0    / cmp/pl r0
//! bf    0x8c0bfebe                    ; only the sign-extended low 16 bits count
//! mov.l 0x8c0bfec8,r0 / jsr @r0 / nop ; free(block) through the 0x8c01032c thunk
//! mov   r14,r0                        ; return the block argument
//! lds.l @r15+,pr  / rts
//! mov.l @r15+,r14                     ; rts delay slot
//! ```
//!
//! All three exits converge on the same epilogue: `r0` is the entry `r4` (kept
//! in `r14`), and the saved `r14`/`pr` come back from the stack, so both end at
//! their entry values. On the call path the callee's `r0`..`r6` residue and T
//! survive except that `mov r14,r0` overwrites `r0`; on both early exits no
//! callee runs, so only `r0` changes and T is the `tst`/`cmp/pl` result.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[14]);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr);
    cpu.r[14] = cpu.r[4];
    let null = cpu.r[14] == 0;
    cpu.set_t(null);
    if !null {
        cpu.r[0] = cpu.r[5] as i16 as i32 as u32; // exts.w
        let positive = (cpu.r[0] as i32) > 0;
        cpu.set_t(positive);
        if positive {
            cpu.r[0] = mem.read_u32(0x8c0bfec8).unwrap_or(0);
            cpu.pr = 0x8c0bfebe; // jsr: pr = return address, delay slot is a nop
            crate::registry::p_8c01032c::run(cpu, mem);
        }
    }
    cpu.r[0] = cpu.r[14];
    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[14] = mem.read_u32(cpu.r[15]).unwrap_or(0); // delay slot of `rts`
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}
