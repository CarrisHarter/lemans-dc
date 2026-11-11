//! 0x8c0cf068 — test_bits_to_bool.
//!
//! `r1=[r4+12] ; tst r1,r5 ; subc r0,r0 ; rts ; add #1,r0` — returns 1 when
//! the AND is non-zero and 0 when it is zero.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[1] = mem.read_u32(cpu.r[4] + 12).unwrap_or(0);
    let zero = (cpu.r[1] & cpu.r[5]) == 0;
    cpu.set_t(zero); // tst r1,r5
    cpu.r[0] = 0u32.wrapping_sub(zero as u32); // subc r0,r0
    cpu.r[0] = cpu.r[0].wrapping_add(1); // delay slot of `rts`
}
