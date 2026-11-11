//! 0x8c1b7d7c — *r4 = 0x8c1f1b44 (overwrites 0x8c1f0318), [4]=0, [8]=0, [12]=1, rts delay [24]=0; r0=r4.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[0] = 0x8c1f0318;
    cpu.r[3] = 0;
    cpu.r[1] = 1;
    let _ = mem.write_u32(cpu.r[4], cpu.r[0]);
    cpu.r[0] = 0x8c1f1b44;
    let _ = mem.write_u32(cpu.r[4], cpu.r[0]);
    let _ = mem.write_u32(cpu.r[4] + 8, cpu.r[3]);
    cpu.r[0] = cpu.r[4];
    let _ = mem.write_u32(cpu.r[4] + 4, cpu.r[3]);
    let _ = mem.write_u32(cpu.r[4] + 12, cpu.r[1]);
    let _ = mem.write_u32(cpu.r[4] + 24, cpu.r[3]); // rts delay slot
}
