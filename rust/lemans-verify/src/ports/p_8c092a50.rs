//! 0x8c092a50 — release a global object: two refcount drops, then free.
//!
//! `r14 = *0x8c26c368` (loaded through the pool); a null object exits at once
//! (`tst` leaves T=1). Otherwise two `jsr` calls to 0x8c018bd8 with `r4 =
//! object+4` and `r4 = object`, each with `mov #-1,r5` in the delay slot, then a
//! `jsr` to 0x8c01032c with `r4 = object` in its delay slot. No arguments are
//! read from the caller; r14 is pushed, so it ends at its entry value.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[0] = mem.read_u32(0x8c092a7c).unwrap_or(0x8c26c368);
    let object = mem.read_u32(cpu.r[0]).unwrap_or(0); // mov.l @r0,r14
    cpu.set_t(object == 0); // tst r14,r14
    if object != 0 {
        cpu.r[0] = mem.read_u32(0x8c092a80).unwrap_or(0x8c018bd8);
        cpu.r[4] = object.wrapping_add(4); // mov r14,r4 ; add #4,r4
        cpu.r[5] = u32::MAX; // delay slot: mov #-1,r5
        cpu.pr = 0x8c092a66;
        crate::registry::p_8c018bd8::run(cpu, mem);

        cpu.r[0] = mem.read_u32(0x8c092a80).unwrap_or(0x8c018bd8);
        cpu.r[4] = object; // mov r14,r4
        cpu.r[5] = u32::MAX; // delay slot: mov #-1,r5
        cpu.pr = 0x8c092a6e;
        crate::registry::p_8c018bd8::run(cpu, mem);

        cpu.r[0] = mem.read_u32(0x8c092a84).unwrap_or(0x8c01032c);
        cpu.r[4] = object; // delay slot: mov r14,r4
        cpu.pr = 0x8c092a74;
        crate::registry::p_8c01032c::run(cpu, mem);
    }
}
