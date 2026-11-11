//! 0x8c08579c — `*r4 = r5; return r4;`
//!
//! `mov.l r5,@r4 ; rts ; mov r4,r0`
//! The C hides the return value; the `rts` delay slot leaves r0 = r4.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let _ = mem.write_u32(cpu.r[4], cpu.r[5]); // mov.l r5,@r4
    cpu.r[0] = cpu.r[4]; // rts delay slot: mov r4,r0
}
