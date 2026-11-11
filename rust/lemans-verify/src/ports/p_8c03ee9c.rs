//! 0x8c03ee9c — `*(u8 *)(DAT_8c03eea4 + param_1) = 1;`
//!
//! `mov.l 0x8c03eea4,r0 ; mov #1,r1 ; rts ; mov.b r1,@(r0,r4)`
//! Side effects the C does not show: r0 is left holding the literal-pool
//! value 0x3bc, r1 is left 1, and the byte store happens in the `rts`
//! delay slot at [r0 + r4] (a single byte, not a word).

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[0] = mem.read_u32(0x8C03EEA4).unwrap(); // mov.l 0x8c03eea4,r0
    cpu.r[1] = 1; // mov #1,r1
    let addr = cpu.r[0].wrapping_add(cpu.r[4]);
    let _ = mem.write_u8(addr, cpu.r[1] as u8); // delay slot of `rts`: mov.b r1,@(r0,r4)
}
