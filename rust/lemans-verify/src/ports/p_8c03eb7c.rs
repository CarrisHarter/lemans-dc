//! 0x8c03eb7c — `*DAT_8c03eb84 = param_1; return param_1;`
//!
//! `mov.l 0x8c03eb84,r0 ; mov.l r4,@r0 ; rts ; mov r4,r0`
//! Side effects the C does not show: r0 is left holding the argument (the
//! `mov r4,r0` delay slot of `rts` is the return value), and the store to
//! the global happens before the return, through the pointer loaded from
//! the literal pool.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[0] = mem.read_u32(0x8C03EB84).unwrap(); // mov.l 0x8c03eb84,r0
    let _ = mem.write_u32(cpu.r[0], cpu.r[4]); // mov.l r4,@r0
    cpu.r[0] = cpu.r[4]; // delay slot of `rts`: mov r4,r0
}
