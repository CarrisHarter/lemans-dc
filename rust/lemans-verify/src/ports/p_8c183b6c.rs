//! 0x8c183b6c - zero five words, then reset the sub-object at `r4+20`.
//!
//! `[r14+0..+16]` are cleared, then 0x8c0b4ff0 runs on `r14+20` (the `add #20,r4`
//! happens before the call). Returns r14.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[14]);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr);
    cpu.r[14] = cpu.r[4]; // mov r4,r14
    cpu.r[1] = 0; // mov #0,r1
    let _ = mem.write_u32(cpu.r[14].wrapping_add(16), cpu.r[1]); // mov.l r1,@(16,r14)
    cpu.r[4] = cpu.r[4].wrapping_add(20); // add #20,r4
    let _ = mem.write_u32(cpu.r[14].wrapping_add(12), cpu.r[1]); // mov.l r1,@(12,r14)
    let _ = mem.write_u32(cpu.r[14].wrapping_add(8), cpu.r[1]); // mov.l r1,@(8,r14)
    cpu.r[0] = mem.read_u32(0x8c183b8c).unwrap_or(0x8c0b4ff0); // mov.l 0x8c183b8c,r0
    let _ = mem.write_u32(cpu.r[14].wrapping_add(4), cpu.r[1]); // mov.l r1,@(4,r14)
    let _ = mem.write_u32(cpu.r[14], cpu.r[1]); // jsr delay: mov.l r1,@r14
    cpu.pr = 0x8c183b84;
    crate::registry::p_8c0b4ff0::run(cpu, mem);
    cpu.r[0] = cpu.r[14]; // mov r14,r0
    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[14] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}
