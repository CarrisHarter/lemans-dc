//! 0x8c232f60 — r5 = ((r4 & -8) >> 1) + (r4&3); +0x400000 if r4&4; +0x400000 if r4>=0x800000; rts delay r0=r5.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, _mem: &mut Memory) {
    cpu.r[5] = cpu.r[4] & 0xfffffff8;
    cpu.r[6] = 0x400000;
    cpu.r[2] = 3;
    cpu.r[5] >>= 1; // shlr
    cpu.r[2] &= cpu.r[4];
    cpu.r[3] = 4;
    cpu.r[5] = cpu.r[5].wrapping_add(cpu.r[2]);
    if cpu.r[4] & cpu.r[3] != 0 {
        cpu.r[5] = cpu.r[5].wrapping_add(cpu.r[6]);
    }
    cpu.r[3] = 0x800000;
    if cpu.r[4] >= cpu.r[3] {
        cpu.r[5] = cpu.r[5].wrapping_add(cpu.r[6]);
    }
    cpu.r[0] = cpu.r[5]; // rts delay
}
