//! 0x8c010000 — fixture: `r0` counts down `r1` to zero.
//!
//! Original: `mov r1,r4 ; mov #0,r0 ; add #1,r0 ; dt r1 ; bf …`
//! T ends set because `dt` leaves zero.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, _mem: &mut Memory) {
    let mut counter = cpu.r[1];
    cpu.r[4] = counter; // the original's `mov r1,r4` is an observable side effect
    let mut acc: u32 = 0;
    loop {
        acc = acc.wrapping_add(1);
        counter = counter.wrapping_sub(1);
        if counter != 0 {
            continue;
        }
        break;
    }
    cpu.r[0] = acc;
    cpu.r[1] = counter;
    cpu.set_t(true);
}
