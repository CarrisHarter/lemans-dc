//! 0x8c239158 — walk list from *(r4+4); r7=1 if node with r4==r5 && *(r4+8)==r6; rts delay r0=r7.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[4] = mem.read_u32(cpu.r[4] + 4).unwrap_or(0);
    cpu.r[7] = 0;
    while cpu.r[4] != 0 {
        cpu.r[0] = 1;
        if cpu.r[5] == cpu.r[4] && cpu.r[4] != 0 {
            cpu.r[2] = mem.read_u32(cpu.r[4] + 8).unwrap_or(0);
            if cpu.r[6] == cpu.r[2] {
                cpu.r[7] = cpu.r[0];
            }
        }
        cpu.r[4] = mem.read_u32(cpu.r[4] + 4).unwrap_or(0);
    }
    cpu.sr |= 1; // final tst r4,r4 with r4==0 -> T=1
    cpu.r[0] = cpu.r[7]; // rts delay
}
