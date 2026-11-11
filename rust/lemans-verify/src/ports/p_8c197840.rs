//! 0x8c197840 — rts; delay *(r4+56) = r5.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let _ = mem.write_u32(cpu.r[4] + 56, cpu.r[5]); // rts delay
}
