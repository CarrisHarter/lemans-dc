//! 0x8c22c3e0 — call the three-list sum and restore pr.
//!
//! r4 and r5 pass straight through. The only residue this thunk adds is r3
//! loaded from the pool before the call; the callee overwrites it.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    // 0x8c22c3e0 sts.l pr,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr);
    // 0x8c22c3e2 mov.l 0x8c22c400,r3
    cpu.r[3] = mem.read_u32(0x8c22c400).unwrap_or(0);
    // 0x8c22c3e4 jsr @r3 / 0x8c22c3e6 nop
    cpu.pr = 0x8c22c3e8;
    crate::registry::p_8c237440::run(cpu, mem);
    // 0x8c22c3e8 lds.l @r15+,pr / 0x8c22c3ea rts / 0x8c22c3ec nop
    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}
