//! 0x8c085700 — free a container's payload, clear a global, then free the container.
//!
//! `tst` on r4 gives the null exit (T=1, nothing runs). Non-null: r13 = r5 is
//! the count, the first `jsr` frees `*r4` (argument load in the delay slot), the
//! `bf.s` delay slot stores 0 through the global at 0x8c26c238 on **both**
//! outcomes, and only a positive sign-extended low half of r5 reaches the second
//! `jsr`, which frees the container itself. `r0 = r14` afterwards, then pr/r13/
//! r14 are popped in the epilogue.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let object = cpu.r[4]; // mov r4,r14
    let count_reg = cpu.r[5]; // mov r5,r13
    cpu.set_t(object == 0); // tst r14,r14
    if object != 0 {
        cpu.r[0] = mem.read_u32(0x8c085730).unwrap_or(0x8c01032c);
        cpu.r[4] = mem.read_u32(object).unwrap_or(0); // delay slot: mov.l @r14,r4
        cpu.pr = 0x8c085714;
        crate::registry::p_8c01032c::run(cpu, mem); // free(*object)

        cpu.r[3] = mem.read_u32(0x8c085734).unwrap_or(0x8c26c238);
        cpu.r[1] = 0; // mov #0,r1
        let count = (count_reg as u16 as i16) as i32; // exts.w r13,r0
        cpu.r[0] = count as u32;
        cpu.set_t(count > 0); // cmp/pl r0
        let _ = mem.write_u32(cpu.r[3], cpu.r[1]); // bf.s delay slot: both outcomes
        if count > 0 {
            cpu.r[0] = mem.read_u32(0x8c085730).unwrap_or(0x8c01032c);
            cpu.r[4] = object; // delay slot: mov r14,r4
            cpu.pr = 0x8c085726;
            crate::registry::p_8c01032c::run(cpu, mem); // free(object)
        }
    }
    cpu.r[0] = object; // mov r14,r0
    // r13 and r14 are pushed in the prologue and popped in the epilogue, so both
    // end at their entry values; only r0 (and the callee residue) changes.
}
