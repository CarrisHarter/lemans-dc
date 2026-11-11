//! 0x8c1ade6c — same zero-init as 0x8c1ade18.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[1] = 0;
    let _ = mem.write_u32(cpu.r[4], cpu.r[1]);
    cpu.r[0] = cpu.r[4];
    let _ = mem.write_u32(cpu.r[4] + 4, cpu.r[1]);
    let _ = mem.write_u32(cpu.r[4] + 8, cpu.r[1]); // rts delay
}
