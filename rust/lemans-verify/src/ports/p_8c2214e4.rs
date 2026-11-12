//! 0x8c2214e4 — boolean wrapper: -1 when 0x8c221da8 returns non-zero, else 0.
//!
//! `jsr` the callee, `tst r0,r0`; the non-zero arm returns `mov #-1,r0`, the
//! zero arm `mov #0,r0`. `pr` is saved and restored on both arms; T is the
//! `tst` result on entry to either exit (the `mov #imm` does not change it).

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[3] = mem.read_u32(0x8c2215e8).unwrap_or(0x8c221da8); // mov.l 0x8c2215e8,r3
    cpu.pr = 0x8c2214ec;
    crate::registry::p_8c221da8::run(cpu, mem); // jsr @r3 ; nop
    cpu.set_t(cpu.r[0] == 0); // tst r0,r0
    cpu.r[0] = if cpu.r[0] == 0 {
        0 // mov #0,r0
    } else {
        u32::MAX // mov #-1,r0
    };
}
