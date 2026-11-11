//! 0x8c0d46f4 — copy 4 words from r5 to r4+64.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[4] = cpu.r[4].wrapping_add(64);
    for i in 0..4u32 {
        let v = mem.read_u32(cpu.r[5] + i * 4).unwrap_or(0);
        if i % 2 == 0 {
            cpu.r[1] = v;
        } else {
            cpu.r[0] = v;
        }
        let _ = mem.write_u32(cpu.r[4] + i * 4, v);
    }
}
