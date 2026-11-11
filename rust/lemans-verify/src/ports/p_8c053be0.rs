//! 0x8c053be0 — `return param_1`: `rts` with delay slot `mov r4,r0`, so the
//! only effect is r0 = r4. T untouched.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, _mem: &mut Memory) {
    cpu.r[0] = cpu.r[4]; // rts delay slot
}
