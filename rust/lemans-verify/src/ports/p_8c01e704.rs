//! 0x8c01e704 — `*dst = *src; return dst;`
//!
//! `mov.l @r5,r1 ; mov r4,r0 ; rts ; mov.l r1,@r4`
//! Hidden side effects: r1 keeps the loaded source word and the store happens
//! in the rts delay slot.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let v = mem.read_u32(cpu.r[5]).unwrap_or(0);
    cpu.r[1] = v;
    cpu.r[0] = cpu.r[4];
    // delay slot of `rts`
    let _ = mem.write_u32(cpu.r[4], v);
}
