//! 0x8c197a98 — r0 = *r4; *(r4+4) = r0; rts delay *(r4+8) = r0.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[0] = mem.read_u32(cpu.r[4]).unwrap_or(0);
    let _ = mem.write_u32(cpu.r[4] + 4, cpu.r[0]);
    let _ = mem.write_u32(cpu.r[4] + 8, cpu.r[0]); // rts delay
}
