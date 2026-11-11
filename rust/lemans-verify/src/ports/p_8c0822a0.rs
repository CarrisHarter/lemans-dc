//! 0x8c0822a0 — copy 12 words from r5 to r4+56.
//!
//! Original side effects: r4 += 56, r1 = *(r5+40), r0 = *(r5+44).

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[4] = cpu.r[4].wrapping_add(56);
    for i in 0..12u32 {
        let v = mem.read_u32(cpu.r[5] + i * 4).unwrap_or(0);
        if i % 2 == 0 {
            cpu.r[1] = v;
        } else {
            cpu.r[0] = v;
        }
        let _ = mem.write_u32(cpu.r[4] + i * 4, v);
    }
}
