//! 0x8c14aeb0 - init the sub-object at `r4+24`, then stamp -1 at `[r4+36]`.
//!
//! The `jsr` delay slot adds 24 to r4, so the callee gets `r4+24` while r14
//! keeps the original. `mov #-1,r0` runs after the call, so r0 = 0xffffffff
//! is the return value as well as the stored word.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[14]);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr);
    cpu.r[14] = cpu.r[4]; // mov r4,r14
    cpu.r[0] = mem.read_u32(0x8c14aec8).unwrap_or(0x8c14b698); // mov.l 0x8c14aec8,r0
    cpu.r[4] = cpu.r[4].wrapping_add(24); // jsr delay: add #24,r4
    cpu.pr = 0x8c14aebc;
    crate::registry::p_8c14b698::run(cpu, mem);
    cpu.r[0] = 0xffff_ffff; // mov #-1,r0
    let _ = mem.write_u32(cpu.r[14].wrapping_add(36), cpu.r[0]); // mov.l r0,@(36,r14)
    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0); // lds.l @r15+,pr
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[14] = mem.read_u32(cpu.r[15]).unwrap_or(0); // rts delay: mov.l @r15+,r14
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}
