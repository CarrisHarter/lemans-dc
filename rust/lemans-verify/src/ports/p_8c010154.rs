//! 0x8c010154 — `return *(u32 *)(DAT_8c01015c + param_1 * 4);`
//!
//! `mov.l 0x8c01015c,r0 ; shll2 r4 ; rts ; mov.l @(r0,r4),r0`
//! The C hides that `param_1 * 4` is a real `shll2 r4`: r4 is left shifted
//! by 2 on return, and the table base is left in r0 before the delay-slot
//! load overwrites it.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[0] = mem.read_u32(0x8C01015C).unwrap();
    cpu.r[4] = cpu.r[4] << 2; // shll2 r4 (no T bit)
    let addr = cpu.r[0].wrapping_add(cpu.r[4]);
    cpu.r[0] = mem.read_u32(addr).unwrap(); // delay slot of `rts`
}
