//! 0x8c1cb4bc — like 0x8c124d50: *r4 = **r4, *(*r15) = old *r4 (rts delay).

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[1] = mem.read_u32(cpu.r[4]).unwrap_or(0);
    cpu.r[0] = mem.read_u32(cpu.r[1]).unwrap_or(0);
    let _ = mem.write_u32(cpu.r[4], cpu.r[0]);
    cpu.r[0] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    let _ = mem.write_u32(cpu.r[0], cpu.r[1]); // rts delay slot
}
