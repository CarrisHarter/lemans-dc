//! 0x8c0f8f10 — release a handle, then free it when the count's low half is positive.
//!
//! Null r4 exits after the prologue (`tst` leaves T=1). Otherwise 0x8c018bd8
//! runs with `r5 = -1`, then `exts.w r5 > 0` gates a final free through
//! 0x8c01032c. `r0 = r13` at exit; r13/r14 are pushed and end at entry values.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let handle = cpu.r[4]; // mov r4,r13
    let count_reg = cpu.r[5]; // mov r5,r14
    cpu.set_t(handle == 0); // tst r13,r13
    if handle != 0 {
        cpu.r[0] = mem.read_u32(0x8c0f8f3c).unwrap_or(0x8c018bd8); // mov.l 0x8c0f8f3c,r0
        cpu.r[5] = u32::MAX; // delay slot: mov #-1,r5
        cpu.pr = 0x8c0f8f24;
        crate::registry::p_8c018bd8::run(cpu, mem); // jsr @r0

        let count = (count_reg as u16 as i16) as i32; // exts.w r14,r0
        cpu.r[0] = count as u32;
        cpu.set_t(count > 0); // cmp/pl r0
        if count > 0 {
            cpu.r[0] = mem.read_u32(0x8c0f8f40).unwrap_or(0x8c01032c);
            cpu.r[4] = handle; // delay slot: mov r13,r4
            cpu.pr = 0x8c0f8f30;
            crate::registry::p_8c01032c::run(cpu, mem);
        }
    }
    cpu.r[0] = handle; // mov r13,r0
}
