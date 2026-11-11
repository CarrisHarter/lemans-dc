//! 0x8c10b6bc — switch(r5): 0 → r4+76, 1 → r4+0x94, 2 → r4+0xcc, 3 → r4+0x104, else 0.
//! r0 = result (rts delay slot per case).

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, _mem: &mut Memory) {
    cpu.set_t(cpu.r[5] == 0); // tst r5,r5
    if cpu.r[5] == 0 {
        cpu.r[4] = cpu.r[4].wrapping_add(76);
        cpu.r[0] = cpu.r[4]; // rts delay slot
        return;
    }
    cpu.r[0] = cpu.r[5];
    cpu.set_t(cpu.r[0] == 1);
    if cpu.r[0] == 1 {
        cpu.r[0] = 0x94;
        cpu.r[0] = cpu.r[0].wrapping_add(cpu.r[4]); // rts delay
        return;
    }
    cpu.set_t(cpu.r[0] == 2);
    if cpu.r[0] == 2 {
        cpu.r[0] = 0xcc;
        cpu.r[0] = cpu.r[0].wrapping_add(cpu.r[4]);
        return;
    }
    cpu.set_t(cpu.r[0] == 3);
    if cpu.r[0] == 3 {
        cpu.r[0] = 0x104;
        cpu.r[0] = cpu.r[0].wrapping_add(cpu.r[4]);
        return;
    }
    cpu.r[0] = 0; // rts delay slot
}
