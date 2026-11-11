//! 0x8c010160 — `*(u32 *)(DAT_8c010168 + param_1 * 4) = param_2;`
//!
//! `mov.l 0x8c010168,r0 ; shll2 r4 ; rts ; mov.l r5,@(r0,r4)`
//! Side effects the C does not show: r4 is left shifted left by 2, and r0
//! is left holding the table base from the `mov.l` (the delay-slot store
//! does not touch r0).

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[0] = mem.read_u32(0x8C010168).unwrap();
    cpu.r[4] = cpu.r[4] << 2; // shll2 r4 (no T bit)
    let addr = cpu.r[0].wrapping_add(cpu.r[4]);
    let _ = mem.write_u32(addr, cpu.r[5]); // delay slot of `rts`
}
