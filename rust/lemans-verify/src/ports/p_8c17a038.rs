//! 0x8c17a038 — index_times_28.
//!
//! `r0=r5 ; shll2 ; shll ; r0-=r5 ; shll2 ; rts ; r0+=r4` — computes
//! `28*r5 + r4`. `shll2` leaves T alone; the `shll r0` sets it from bit 31
//! of r5<<2, which is the last flag-setting instruction before the exit.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[0] = cpu.r[5];
    cpu.r[0] = cpu.r[0] << 2;
    cpu.set_t(cpu.r[0] >> 31 != 0); // shll r0 sets T from the old bit 31
    cpu.r[0] = cpu.r[0] << 1;
    cpu.r[0] = cpu.r[0].wrapping_sub(cpu.r[5]);
    cpu.r[0] = cpu.r[0] << 2;
    cpu.r[0] = cpu.r[0].wrapping_add(cpu.r[4]); // delay slot of `rts`
}
