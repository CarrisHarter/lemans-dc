//! 0x8c225ae4 — index_times_10_plus_low_nibble.
//!
//! `10*(r4>>4) + (r4&15)` — a decimal-digit split via shifts. `shlr2` and
//! `and` leave T alone; the `shll r0` before the `and` sets it from bit 31 of
//! `5*(r4>>4)`, which is the last flag-setting instruction before the exit.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[5] = cpu.r[4];
    cpu.r[5] = cpu.r[5] >> 2;
    cpu.r[5] = cpu.r[5] >> 2;
    cpu.r[0] = cpu.r[5];
    cpu.r[3] = cpu.r[5];
    cpu.r[0] = cpu.r[0] << 2;
    cpu.r[2] = 15;
    cpu.r[0] = cpu.r[0].wrapping_add(cpu.r[3]);
    cpu.set_t(cpu.r[0] >> 31 != 0); // shll r0 sets T from the old bit 31
    cpu.r[0] = cpu.r[0] << 1;
    cpu.r[4] = cpu.r[4] & cpu.r[2];
    cpu.r[0] = cpu.r[0].wrapping_add(cpu.r[4]); // delay slot of `rts`
}
