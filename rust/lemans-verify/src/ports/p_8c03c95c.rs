//! 0x8c03c95c — `*(u32 *)(DAT_8c03c964 + param_1) = 0;`
//!
//! `mov.l 0x8c03c964,r0 ; mov #0,r1 ; rts ; mov.l r1,@(r0,r4)`
//! Same shape as 0x8c03c950 with the 0xf8 offset base: r0 is left holding
//! the literal-pool value, r1 is left 0, and the zero store happens in the
//! `rts` delay slot at [r0 + r4].

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[0] = mem.read_u32(0x8C03C964).unwrap(); // mov.l 0x8c03c964,r0
    cpu.r[1] = 0; // mov #0,r1
    let addr = cpu.r[0].wrapping_add(cpu.r[4]);
    let _ = mem.write_u32(addr, cpu.r[1]); // delay slot of `rts`: mov.l r1,@(r0,r4)
}
