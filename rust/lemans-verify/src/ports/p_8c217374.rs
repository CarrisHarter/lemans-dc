//! 0x8c217374 — *r4=0; *(r4+4)=r5; *(r4+8)=0; rts delay r0=r4.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[6] = 0;
    let _ = mem.write_u32(cpu.r[4], cpu.r[6]);
    let _ = mem.write_u32(cpu.r[4] + 4, cpu.r[5]);
    let _ = mem.write_u32(cpu.r[4] + 8, cpu.r[6]);
    cpu.r[0] = cpu.r[4]; // rts delay
}
