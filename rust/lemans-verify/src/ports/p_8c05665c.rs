//! 0x8c05665c — `rts ; mov r4,r0`: return param_1.
//!
//! The only effect is the delay slot: r0 = r4. T and everything else stay
//! as seeded (the spec seeds sr.T = 1 to prove it).

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, _mem: &mut Memory) {
    cpu.r[0] = cpu.r[4]; // delay slot of `rts`
}
