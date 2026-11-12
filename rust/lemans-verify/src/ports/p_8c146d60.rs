//! 0x8c146d60 - call 0x8c14ae28 with r14 pre-set to 1 and add 1 to the result.
//!
//! `mov #0,r14` then the `jsr` delay slot `add #1,r14` means the callee sees
//! r14 = 1. r14 and pr are pushed and restored, so both end at entry values.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[14]);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr);
    cpu.r[0] = mem.read_u32(0x8c146d74).unwrap_or(0x8c14ae28); // mov.l 0x8c146d74,r0
    cpu.r[14] = 0; // mov #0,r14
    cpu.r[14] = cpu.r[14].wrapping_add(1); // jsr delay: add #1,r14
    cpu.pr = 0x8c146d6c;
    crate::registry::p_8c14ae28::run(cpu, mem);
    cpu.r[0] = cpu.r[0].wrapping_add(cpu.r[14]); // add r14,r0
    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0); // lds.l @r15+,pr
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[14] = mem.read_u32(cpu.r[15]).unwrap_or(0); // rts delay: mov.l @r15+,r14
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}
