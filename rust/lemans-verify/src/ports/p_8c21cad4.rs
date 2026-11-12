//! 0x8c21cad4 — r5 = r4 >> 10; if r4 & 0x20000 { r5 = (r5 & 0x3fff00) | 0xf0 }; if r4 & 0x10000 { r5 |= 15 }; rts delay r0=r5.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, _mem: &mut Memory) {
    cpu.r[3] = 0x20000;
    cpu.r[5] = cpu.r[4];
    cpu.r[2] = 0x3fff00;
    cpu.r[5] >>= 8; // shlr8
    let high = cpu.r[4] & cpu.r[3] != 0; // tst r4,r3
    cpu.r[5] >>= 2; // shlr2 (T unchanged)
    cpu.r[5] &= cpu.r[2]; // delay slot of bt.s: always executes
    if high {
        cpu.r[0] = 0xf0;
        cpu.r[5] |= cpu.r[0];
    }
    cpu.r[3] = 0x10000;
    let low = cpu.r[4] & cpu.r[3] != 0;
    if low {
        cpu.r[1] = 15;
        cpu.r[5] |= cpu.r[1];
    }
    cpu.set_t(!low); // tst r3,r4 feeds bt at rts
    cpu.r[0] = cpu.r[5]; // rts delay
}
