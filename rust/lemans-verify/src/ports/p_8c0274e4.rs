//! 0x8c0274e4 — `return **(u32 **)(DAT_8c0274ec + param_1);`
//!
//! `mov.l 0x8c0274ec,r0 ; mov.l @(r0,r4),r0 ; rts ; mov.l @r0,r0`
//! Same double indirection as 0x8c0274d8, different literal slot (0x178);
//! the final value is the `rts` delay-slot load.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[0] = mem.read_u32(0x8C0274EC).unwrap(); // literal pool: 0x178
    cpu.r[0] = mem.read_u32(cpu.r[0].wrapping_add(cpu.r[4])).unwrap();
    cpu.r[0] = mem.read_u32(cpu.r[0]).unwrap(); // delay slot of `rts`
}
