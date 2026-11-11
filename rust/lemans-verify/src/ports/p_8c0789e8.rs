//! 0x8c0789e8 — *r4 = r5; return r4 (rts delay slot).

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let _ = mem.write_u32(cpu.r[4], cpu.r[5]);
    cpu.r[0] = cpu.r[4]; // rts delay slot: mov r4,r0
}
