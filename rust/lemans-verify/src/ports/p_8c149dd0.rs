//! 0x8c149dd0 — r0 = (unsigned)*r5 > *r4 ? r5 : r4 (pointer max).

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[1] = mem.read_u32(cpu.r[4]).unwrap_or(0);
    cpu.r[0] = mem.read_u32(cpu.r[5]).unwrap_or(0);
    cpu.set_t(cpu.r[0] > cpu.r[1]); // cmp/hi r1,r0: T=(r0>r1)
    if !(cpu.r[0] > cpu.r[1]) {
        cpu.r[5] = cpu.r[4];
    }
    cpu.r[0] = cpu.r[5]; // rts delay slot
}
