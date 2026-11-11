//! 0x8c104a64 — copy 3 floats from r5 to r4; r5 += 12.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    for i in 0..3u32 {
        let v = f32::from_bits(mem.read_u32(cpu.r[5] + i * 4).unwrap_or(0));
        let _ = mem.write_u32(cpu.r[4] + i * 4, v.to_bits());
    }
    cpu.r[5] = cpu.r[5].wrapping_add(12);
}
