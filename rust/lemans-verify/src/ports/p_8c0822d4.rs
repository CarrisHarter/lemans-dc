//! 0x8c0822d4 — copy 9 words from r5 to r4+20.
//!
//! Original side effects: r4 += 20, r1 = *(r5+24), r0 = *(r5+32).

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[4] = cpu.r[4].wrapping_add(20);
    for i in 0..9u32 {
        let v = mem.read_u32(cpu.r[5] + i * 4).unwrap_or(0);
        if i == 8 || i % 2 == 1 {
            cpu.r[0] = v;
        } else {
            cpu.r[1] = v;
        }
        let _ = mem.write_u32(cpu.r[4] + i * 4, v);
    }
}
