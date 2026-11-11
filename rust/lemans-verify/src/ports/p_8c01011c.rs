//! 0x8c01011c — `*param = 0; return param;`
//!
//! `mov #0,r0 ; mov.l r0,@r4 ; rts ; mov r4,r0`
//! The delay slot copies the argument into r0, which is the return value the
//! C signature hides behind `return param_1`.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let base = cpu.r[4];
    cpu.r[0] = 0;
    let _ = mem.write_u32(base, 0);
    cpu.r[0] = base; // delay slot of `rts`
}
