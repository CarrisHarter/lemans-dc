//! 0x8c1b7840 — r5==0/1/2 → r0=7, else r0=0 (via bra delay).

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, _mem: &mut Memory) {
    cpu.set_t(cpu.r[5] == 0);
    if cpu.r[5] == 0 {
        cpu.r[0] = 7;
        return;
    }
    cpu.r[0] = cpu.r[5];
    cpu.set_t(cpu.r[0] == 1);
    if cpu.r[0] == 1 {
        cpu.r[0] = 7;
        return;
    }
    cpu.set_t(cpu.r[0] == 2);
    if cpu.r[0] == 2 {
        cpu.r[0] = 3;
        return;
    }
    cpu.r[0] = 0; // bra delay slot
}
