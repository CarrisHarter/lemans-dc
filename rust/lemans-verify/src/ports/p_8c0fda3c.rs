//! 0x8c0fda3c — remap the arguments and forward to 0x8c0e1f18 when r6 is set.
//!
//! `r4 = r5` always; `tst r6,r6` decides whether 0x8c0e1f18 runs with
//! `r5 = r6`. The `rts` delay slot is a nop and pr is restored, so the observable
//! state is the callee's (or the incoming registers on the skip path).

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[4] = cpu.r[5]; // mov r5,r4
    cpu.set_t(cpu.r[6] == 0); // tst r6,r6
    if cpu.r[6] != 0 {
        cpu.r[0] = mem.read_u32(0x8c0fda50).unwrap_or(0x8c0e1f18); // mov.l 0x8c0fda50,r0
        cpu.r[5] = cpu.r[6]; // delay slot: mov r6,r5
        cpu.pr = 0x8c0fda4a;
        crate::registry::p_8c0e1f18::run(cpu, mem); // jsr @r0
    }
}
