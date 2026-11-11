//! 0x8c1b69e8 — rts; delay *(r4+16) = r5.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let _ = mem.write_u32(cpu.r[4] + 16, cpu.r[5]); // rts delay
}
