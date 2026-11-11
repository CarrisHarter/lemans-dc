//! 0x8c110b7c — *r4 = 0, *(r4+4) = 0, *(r4+8) = 0 (rts delay); r1 = 0, r0 = r4.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[1] = 0;
    let _ = mem.write_u32(cpu.r[4], cpu.r[1]);
    cpu.r[0] = cpu.r[4];
    let _ = mem.write_u32(cpu.r[4] + 4, cpu.r[1]);
    let _ = mem.write_u32(cpu.r[4] + 8, cpu.r[1]); // rts delay slot
}
