//! 0x8c14fe70 — r0 = *(r4+4); rts delay *(r0+36) = r5.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[0] = mem.read_u32(cpu.r[4] + 4).unwrap_or(0);
    let _ = mem.write_u32(cpu.r[0] + 36, cpu.r[5]); // rts delay
}
