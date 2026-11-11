//! 0x8c12b504 — `return **param_1;`
//!
//! `mov.l @r4,r0 ; rts ; mov.l @r0,r0`
//! The second dereference lives in the `rts` delay slot, so it runs even
//! though the C presents it as the plain return expression.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let p = mem.read_u32(cpu.r[4]).unwrap();
    cpu.r[0] = mem.read_u32(p).unwrap(); // rts delay slot: mov.l @r0,r0
}
