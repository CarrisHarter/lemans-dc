//! 0x8c0b4ef0 — multiply 3 floats at r4 by fr4 in place; r4 += 8; last store in rts delay.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    for i in 0..3u32 {
        let addr = cpu.r[4].wrapping_add(i * 4);
        let v = f32::from_bits(mem.read_u32(addr).unwrap_or(0));
        let _ = mem.write_u32(addr, (v * cpu.fr[4]).to_bits()); // fmul fr4,fr0
    }
    cpu.r[4] = cpu.r[4].wrapping_add(8);
}
