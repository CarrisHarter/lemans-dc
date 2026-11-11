//! 0x8c051f68 — `*(u32 *)(r4 + 0x194) = r5;`
//!
//! `mov.l 0x8c051f70,r0 ; rts ; mov.l r5,@(r0,r4)` — the store is the `rts`
//! delay slot. Hidden side effect the C does not show: r0 is left holding the
//! pool constant 0x194, not the stored value.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[0] = mem.read_u32(0x8C051F70).unwrap(); // 0x194
    let _ = mem.write_u32(cpu.r[0].wrapping_add(cpu.r[4]), cpu.r[5]); // rts delay slot
}
