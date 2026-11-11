//! 0x8c16c97c — *(r4+4)=r5; r0=1; rts delay byte[r4+8]=1.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let _ = mem.write_u32(cpu.r[4] + 4, cpu.r[5]);
    cpu.r[0] = 1;
    let _ = mem.write_u8(cpu.r[4] + 8, 1); // rts delay
}
