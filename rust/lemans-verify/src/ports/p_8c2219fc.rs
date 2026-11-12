//! 0x8c2219fc — zero 0x400 bytes at 0x8c2bde98, return 0.
//!
//! `r3 = memset (0x8c010f88)`, `r4 = 0x8c2bde98`, `r6 = 0x400`, `r5 = 0` in the
//! `jsr` delay slot; the `rts` delay slot reloads `r0 = 0`, so memset's return
//! value never reaches the caller. `pr` is restored from the stack.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[3] = mem.read_u32(0x8c221a7c).unwrap_or(0x8c010f88); // mov.l 0x8c221a7c,r3
    cpu.r[4] = mem.read_u32(0x8c221a78).unwrap_or(0x8c2bde98); // mov.l 0x8c221a78,r4
    cpu.r[6] = 0x400; // mov.w 0x8c221a32,r6 ! 400
    cpu.r[5] = 0; // delay slot: mov #0,r5
    cpu.pr = 0x8c221a08;
    crate::registry::p_8c010f88::run(cpu, mem); // jsr @r3
    cpu.r[0] = 0; // rts delay: mov #0,r0
}
