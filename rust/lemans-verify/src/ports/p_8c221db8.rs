//! 0x8c221db8 — index_times_128_store.
//!
//! `[[0x8c2bde98] + 128*r4 + 8] = r5`.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[4] = cpu.r[4] << 2;
    cpu.r[3] = mem.read_u32(0x8c221e14).unwrap_or(0);
    cpu.r[4] = cpu.r[4] << 2;
    cpu.r[4] = cpu.r[4] << 2;
    cpu.set_t(cpu.r[4] >> 31 != 0); // shll r4
    cpu.r[4] = cpu.r[4] << 1;
    cpu.r[4] = cpu.r[4].wrapping_add(cpu.r[3]);
    let _ = mem.write_u32(cpu.r[4] + 8, cpu.r[5]); // delay slot of `rts`
}
