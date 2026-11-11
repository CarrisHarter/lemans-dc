//! 0x8c067f08 — `*(u8 *)(0x9c + r4) = 1`.
//!
//! Original: `mov.l 0x8c067f10,r0 ; mov #1,r1 ; rts ; mov.b r1,@(r0,r4)`
//! Side effects the C hides: r0 = 0x9c and r1 = 1 both stay live after the
//! byte store in the `rts` delay slot.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[0] = mem.read_u32(0x8c067f10).unwrap(); // 0x9c
    cpu.r[1] = 1; // mov #1,r1
    let addr = cpu.r[0].wrapping_add(cpu.r[4]);
    let _ = mem.write_u8(addr, cpu.r[1] as u8); // delay slot of rts
}
