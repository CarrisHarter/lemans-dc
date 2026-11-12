//! 0x8c18d06c — guarded free: `if (p && (i16)r5 > 0) free(p)`, returns p.
//!
//! `tst` decides the null path (T=1), `exts.w`+`cmp/pl` the count path (T=0);
//! only the low 16 bits of r5 are examined. The call goes to the free thunk at
//! 0x8c01032c (read from the pool) with `r4` unchanged and `pr = 0x8c18d082`.
//! `mov r14,r0` after the call overwrites the callee's r0 with the saved r4;
//! T is the callee's on that path. r14 and pr are restored from the frame.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let saved = cpu.r[4]; // mov r4,r14 (r14 is pushed, so it ends at its entry value)
    cpu.set_t(saved == 0); // tst r14,r14
    if saved != 0 {
        let count = (cpu.r[5] as u16 as i16) as i32; // exts.w r5,r0
        cpu.r[0] = count as u32;
        cpu.set_t(count > 0); // cmp/pl r0
        if count > 0 {
            cpu.r[0] = mem.read_u32(0x8c18d08c).unwrap_or(0x8c01032c);
            cpu.pr = 0x8c18d082;
            crate::registry::p_8c01032c::run(cpu, mem); // jsr @r0 ; nop
        }
    }
    cpu.r[0] = saved; // mov r14,r0
}
