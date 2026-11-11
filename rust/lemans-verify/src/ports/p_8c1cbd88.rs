//! 0x8c1cbd88 — r5==1 → r0 = *(r4+44); else r0 = *(r4+48) (rts delay slots).

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.set_t(cpu.r[5] == 0); // tst r5,r5
    if cpu.r[5] != 0 {
        cpu.r[0] = cpu.r[5];
        cpu.set_t(cpu.r[0] == 1); // cmp/eq #1,r0
        if cpu.r[0] == 1 {
            cpu.r[0] = mem.read_u32(cpu.r[4] + 44).unwrap_or(0); // rts delay
            return;
        }
        cpu.set_t(cpu.r[0] == 2);
    }
    cpu.r[0] = mem.read_u32(cpu.r[4] + 48).unwrap_or(0); // rts delay (default/2/0)
}
