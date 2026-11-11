//! 0x8c0b4f0c — 3 floats at r4 -= floats at r5 in place; r4 += 8.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let mut last = 0.0f32;
    let mut lastb = 0.0f32;
    for i in 0..3u32 {
        let a = f32::from_bits(mem.read_u32(cpu.r[4] + i * 4).unwrap_or(0));
        let b = f32::from_bits(mem.read_u32(cpu.r[5] + i * 4).unwrap_or(0));
        last = a - b;
        lastb = b;
        let _ = mem.write_u32(cpu.r[4] + i * 4, last.to_bits());
    }
    cpu.r[1] = cpu.r[4].wrapping_add(4);
    cpu.r[0] = cpu.r[5].wrapping_add(4);
    cpu.r[5] = cpu.r[5].wrapping_add(8);
    cpu.r[4] = cpu.r[4].wrapping_add(8);
    cpu.fr[0] = lastb;
    cpu.fr[1] = last;
}
