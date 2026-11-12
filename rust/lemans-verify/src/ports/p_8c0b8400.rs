//! 0x8c0b8400 — stamp a vtable word, then free the object if the count is positive.
//!
//! Null r4 exits after the prologue (`tst` leaves T=1). Otherwise `r1 =
//! [0x8c0b8424]` (0x8c1eaaac) and the `bf.s` delay slot stores it at `*r4` **on
//! both outcomes**; only `exts.w r5 > 0` reaches 0x8c01032c freeing r4.
//! `r0 = r14` at exit; r14 is pushed, so it ends at its entry value.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let object = cpu.r[4]; // mov r4,r14
    cpu.set_t(object == 0); // tst r14,r14
    if object != 0 {
        cpu.r[1] = mem.read_u32(0x8c0b8424).unwrap_or(0x8c1eaaac);
        let count = (cpu.r[5] as u16 as i16) as i32; // exts.w r5,r0
        cpu.r[0] = count as u32;
        cpu.set_t(count > 0); // cmp/pl r0
        let _ = mem.write_u32(object, cpu.r[1]); // bf.s delay slot: both outcomes
        if count > 0 {
            cpu.r[0] = mem.read_u32(0x8c0b8428).unwrap_or(0x8c01032c);
            cpu.pr = 0x8c0b841a;
            crate::registry::p_8c01032c::run(cpu, mem);
        }
    }
    cpu.r[0] = object; // mov r14,r0
}
