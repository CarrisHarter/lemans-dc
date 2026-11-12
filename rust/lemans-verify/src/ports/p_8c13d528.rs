//! 0x8c13d528 — load `[r4+92]` and look up an element, or return 0 if it is null.
//!
//! The loaded pointer replaces r4. A null leaves `r0 = 0` and T=1 (`tst`).
//! Otherwise 0x8c0bde70 runs with that pointer and the caller's r5; its `r0`
//! is the result and the `bra` delay slot is a nop. pr is restored.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    cpu.r[4] = cpu.r[4].wrapping_add(92);
    cpu.r[4] = mem.read_u32(cpu.r[4]).unwrap_or(0);
    cpu.set_t(cpu.r[4] == 0);
    if cpu.r[4] != 0 {
        cpu.r[0] = mem.read_u32(0x8c13d544).unwrap_or(0x8c0bde70);
        cpu.pr = 0x8c13d538;
        crate::registry::p_8c0bde70::run(cpu, mem);
    } else {
        cpu.r[0] = 0;
    }
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}
