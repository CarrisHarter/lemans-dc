//! 0x8c1b7824 — r5==0 → r0=7, r5==1 → r0=1, else r0=0.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, _mem: &mut Memory) {
    cpu.set_t(cpu.r[5] == 0); // tst r5,r5
    if cpu.r[5] == 0 {
        cpu.r[0] = 7; // rts delay slot
        return;
    }
    cpu.r[0] = cpu.r[5];
    cpu.set_t(cpu.r[0] == 1); // cmp/eq #1,r0
    if cpu.r[0] == 1 {
        cpu.r[0] = 1; // rts delay slot
        return;
    }
    cpu.r[0] = 0; // rts delay slot
}
