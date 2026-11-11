//! 0x8c019670 — `return (u8)param[0x70];`
//!
//! `add #112,r4 ; mov.b @r4,r0 ; rts ; extu.b r0,r0`
//! Hidden side effect: r4 itself is advanced by 0x70; the C reads
//! `*(param + 0x70)` without showing r4 modified. The delay slot zero-extends.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[4] = cpu.r[4].wrapping_add(0x70);
    let v = mem.read_u8(cpu.r[4]).unwrap_or(0);
    cpu.r[0] = v as u32; // `extu.b` in the rts delay slot
}
