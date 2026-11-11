//! 0x8c079c9c — copy 3 words from r5 to r4.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[0] = cpu.r[4]; // mov r4,r0
    for i in 0..3u32 {
        cpu.r[1] = mem.read_u32(cpu.r[5] + i * 4).unwrap_or(0);
        let _ = mem.write_u32(cpu.r[4] + i * 4, cpu.r[1]);
    }
}
