//! 0x8c1745ca — clear flag word at 0x8c2b562c, then tail-call memset
//! (0x8c010f88) to zero 0x100 bytes at 0x8c2b552c.
//!
//! Linear setup ending in `jmp @r3` with delay slot `mov r2,r5` (r5 = 0).
//! The tail call is modelled by delegating to the memset port.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[1] = 0x8c2b562c;
    cpu.r[2] = 0;
    cpu.r[6] = 0x100; // mov.w sign-extends 0x0100
    cpu.r[4] = 0x8c2b552c;
    cpu.r[3] = 0x8c010f88;
    let _ = mem.write_u32(0x8c2b562c, 0);
    cpu.r[5] = 0; // jmp delay slot
    crate::registry::p_8c010f88::run(cpu, mem); // jmp @r3
}
