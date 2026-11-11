//! 0x8c0192fc — `*DAT_8c01930c = param; param[0..2] = 0; return param;`
//!
//! `mov.l 0x8c01930c,r0 ; mov #0,r1 ; mov.l r4,@r0 ; mov.l r1,@r4 ; mov r4,r0
//!  ; mov.l r1,@(4,r4) ; rts ; mov.l r1,@(8,r4)`
//! Hidden side effects: r1 is zeroed, r0 ends as the argument, and the rts
//! delay slot performs the third store.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let arg = cpu.r[4];
    let global = mem.read_u32(0x8c01930c).unwrap_or(0);
    cpu.r[0] = global;
    cpu.r[1] = 0;
    let _ = mem.write_u32(global, arg);
    let _ = mem.write_u32(arg, 0);
    cpu.r[0] = arg;
    let _ = mem.write_u32(arg + 4, 0);
    // delay slot of `rts`
    let _ = mem.write_u32(arg + 8, 0);
}
