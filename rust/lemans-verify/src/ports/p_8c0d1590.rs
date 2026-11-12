//! 0x8c0d1590 — stamp a vtable word, call 0x8c06cbc0, then optionally free.
//!
//! Null r4 exits after the prologue (`tst` leaves T=1). Otherwise `r1 =
//! [0x8c0d15c0]` (0x8c1ec088), `r5 = 0`, `jsr 0x8c06cbc0` with the vtable store
//! in the delay slot, then `exts.w r5 > 0` gates a final free. r13/r14 are
//! pushed, so both end at entry values; `r0 = r13` at exit.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let object = cpu.r[4]; // mov r4,r13
    let flag_reg = cpu.r[5]; // mov r5,r14
    cpu.set_t(object == 0); // tst r13,r13
    if object != 0 {
        cpu.r[1] = mem.read_u32(0x8c0d15c0).unwrap_or(0x8c1ec088); // mov.l 0x8c0d15c0,r1
        cpu.r[5] = 0; // mov #0,r5
        cpu.r[0] = mem.read_u32(0x8c0d15c4).unwrap_or(0x8c06cbc0); // mov.l 0x8c0d15c4,r0
        let _ = mem.write_u32(object, cpu.r[1]); // delay slot: mov.l r1,@r13
        cpu.pr = 0x8c0d15a8;
        crate::registry::p_8c06cbc0::run(cpu, mem); // jsr @r0

        let count = (flag_reg as u16 as i16) as i32; // exts.w r14,r0
        cpu.r[0] = count as u32;
        cpu.set_t(count > 0); // cmp/pl r0
        if count > 0 {
            cpu.r[0] = mem.read_u32(0x8c0d15c8).unwrap_or(0x8c01032c);
            cpu.r[4] = object; // delay slot: mov r13,r4
            cpu.pr = 0x8c0d15b4;
            crate::registry::p_8c01032c::run(cpu, mem);
        }
    }
    cpu.r[0] = object; // mov r13,r0
}
