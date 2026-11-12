//! 0x8c21cad4 — r5 = r4 >> 10; when bit 1 of r4>>8 is clear, r5 |= 0xf0; when
//! r4 & 0x10000 is clear, r5 |= 15; rts delay r0=r5.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, _mem: &mut Memory) {
    cpu.r[3] = 0x20000;
    cpu.r[5] = cpu.r[4];
    cpu.r[2] = 0x3fff00;
    cpu.r[5] >>= 8; // shlr8 r5
    let _high = cpu.r[4] & cpu.r[3] != 0; // tst r4,r3 - overwritten by shlr2 below
    // The bt.s at 0x8c21cae0 tests the `shlr2 r5` carry (bit 1 of r4>>8), not
    // the tst: shlr2 sits between the tst and the branch.
    cpu.set_t(cpu.r[5] >> 1 & 1 != 0); // shlr2 r5 sets T from bit 1
    cpu.r[5] >>= 2; // shlr2 r5
    cpu.r[5] &= cpu.r[2]; // delay slot of bt.s: always executes
    if !cpu.t() {
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
