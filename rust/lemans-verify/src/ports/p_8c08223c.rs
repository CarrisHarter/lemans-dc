//! 0x8c08223c — release a block when it is non-null and flagged positive.
//!
//! Saves `r14`/`pr`, then if `r4 != 0` and the sign-extended low half of `r5`
//! is positive, calls `free(r4)` through the 0x8c01032c thunk (r4/r5 unchanged,
//! delay slot `nop`); that thunk tail-jumps to 0x8c159fc0. Always returns
//! `r14` (= `r4`) in r0 and restores `r14`/`pr`/`r15`.
//!
//! Exit T: the null test's `bt` leaves T=1; the `cmp/pl` skip leaves T=0; on
//! the call path T is whatever the callee's last flag-setting instruction set.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[14]);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr);
    cpu.r[14] = cpu.r[4];
    let is_null = cpu.r[14] == 0;
    cpu.set_t(is_null); // tst r14,r14 ; bt
    if !is_null {
        cpu.r[0] = cpu.r[5] as i16 as i32 as u32; // exts.w r5,r0
        let positive = (cpu.r[0] as i32) > 0; // cmp/pl r0
        cpu.set_t(positive);
        if positive {
            cpu.r[0] = mem.read_u32(0x8c08225c).unwrap_or(0x8c01032c);
            cpu.pr = 0x8c082252; // jsr @r0 return address
            // delay slot of `jsr` is `nop`
            crate::registry::p_8c01032c::run(cpu, mem);
        }
    }
    cpu.r[0] = cpu.r[14];
    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[14] = mem.read_u32(cpu.r[15]).unwrap_or(0); // delay slot of `rts`
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}
