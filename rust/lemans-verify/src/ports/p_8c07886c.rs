//! 0x8c07886c — r4 += 12; return it in r0 (rts delay slot).

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, _mem: &mut Memory) {
    cpu.r[4] = cpu.r[4].wrapping_add(12);
    cpu.r[0] = cpu.r[4]; // rts delay slot: mov r4,r0
}
