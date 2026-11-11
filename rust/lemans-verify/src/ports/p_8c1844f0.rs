//! 0x8c1844f0 — copy 6 words from r4 to r5.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    for i in 0..6u32 {
        let v = mem.read_u32(cpu.r[4] + i * 4).unwrap_or(0);
        if i % 2 == 0 {
            cpu.r[1] = v;
        } else {
            cpu.r[0] = v;
        }
        let _ = mem.write_u32(cpu.r[5] + i * 4, v);
    }
}
