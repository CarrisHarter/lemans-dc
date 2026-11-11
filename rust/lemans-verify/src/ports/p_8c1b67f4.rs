//! 0x8c1b67f4 — rts; delay byte[r4] = 0.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[0] = 0;
    let _ = mem.write_u8(cpu.r[4], 0); // rts delay
}
