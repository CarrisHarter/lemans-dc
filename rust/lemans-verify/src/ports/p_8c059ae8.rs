//! 0x8c059ae8 — `*DAT_8c059af0 = param_1;` i.e. store to the global.
//!
//! `mov.l 0x8c059af0,r0 ; rts ; mov.l r4,@r0` — the literal at 0x8c059af0
//! holds 0x8c23de00; the store to that global is the `rts` delay slot and
//! r0 keeps the pointer afterwards.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[0] = mem.read_u32(0x8C059AF0).unwrap(); // mov.l 0x8c059af0,r0
    let _ = mem.write_u32(cpu.r[0], cpu.r[4]); // delay slot of `rts`: mov.l r4,@r0
}
