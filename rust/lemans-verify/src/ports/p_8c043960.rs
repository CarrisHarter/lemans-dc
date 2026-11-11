//! 0x8c043960 — `*DAT_8c043968 = param_1;`
//!
//! `mov.l 0x8c043968,r0 ; rts ; mov.l r4,@r0`
//! Side effects the C does not show: r0 is left holding the global's
//! address from the literal pool (NOT the argument), and the store of
//! param_1 happens in the `rts` delay slot.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[0] = mem.read_u32(0x8C043968).unwrap(); // mov.l 0x8c043968,r0
    let _ = mem.write_u32(cpu.r[0], cpu.r[4]); // delay slot of `rts`: mov.l r4,@r0
}
