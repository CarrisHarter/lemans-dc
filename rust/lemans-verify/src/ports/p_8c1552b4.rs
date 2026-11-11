//! 0x8c1552b4 — copy 6 words from r5 to *([r4+8]).

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[3] = mem.read_u32(cpu.r[4] + 8).unwrap_or(0);
    for i in 0..6u32 {
        let v = mem.read_u32(cpu.r[5] + i * 4).unwrap_or(0);
        if i % 2 == 0 {
            cpu.r[1] = v;
        } else {
            cpu.r[0] = v;
        }
        let _ = mem.write_u32(cpu.r[3] + i * 4, v);
    }
}
