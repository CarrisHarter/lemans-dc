//! 0x8c0ae798 — free an object's payload, then the object itself when asked.
//!
//! Null r4 exits after the prologue (`tst` leaves T=1). Otherwise 0x8c0102cc
//! frees `[r4+4]` (the argument load is the `jsr` delay slot), then
//! `exts.w r5 > 0` gates 0x8c01032c freeing r4 itself. `r0 = r13` at exit;
//! r13/r14 are pushed, so both end at their entry values.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let object = cpu.r[4]; // mov r4,r13
    let count_reg = cpu.r[5]; // mov r5,r14
    cpu.set_t(object == 0); // tst r13,r13
    if object != 0 {
        cpu.r[0] = mem.read_u32(0x8c0ae7c4).unwrap_or(0x8c0102cc);
        cpu.r[4] = mem.read_u32(object.wrapping_add(4)).unwrap_or(0); // delay: mov.l @(4,r13),r4
        cpu.pr = 0x8c0ae7ac;
        crate::registry::p_8c0102cc::run(cpu, mem);

        let count = (count_reg as u16 as i16) as i32; // exts.w r14,r0
        cpu.r[0] = count as u32;
        cpu.set_t(count > 0); // cmp/pl r0
        if count > 0 {
            cpu.r[0] = mem.read_u32(0x8c0ae7c8).unwrap_or(0x8c01032c);
            cpu.r[4] = object; // delay: mov r13,r4
            cpu.pr = 0x8c0ae7b8;
            crate::registry::p_8c01032c::run(cpu, mem);
        }
    }
    cpu.r[0] = object; // mov r13,r0
}
