//! 0x8c0e2884 — return 7 if byte r5 != 0, else 2.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, _mem: &mut Memory) {
    cpu.r[0] = cpu.r[5] & 0xff; // extu.b r5,r0
    cpu.set_t(cpu.r[0] == 0); // tst r0,r0
    if cpu.r[0] != 0 {
        cpu.r[0] = 7; // rts delay slot
        return;
    }
    cpu.r[0] = 2;
}
