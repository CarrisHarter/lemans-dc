//! 0x8c21aff2 — index_times_8_plus_field.
//!
//! `r0 = [0x8c2ba634] + 4 + 8*r4` (the index is scaled by two shifts).
//! `shll2` leaves T alone; `shll r4` sets it from bit 31 of r4<<2 and is the
//! last flag-setting instruction before the `rts`.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[3] = mem.read_u32(0x8c21b018).unwrap_or(0);
    cpu.r[4] = cpu.r[4] << 2;
    cpu.set_t(cpu.r[4] >> 31 != 0); // shll r4 sets T from the old bit 31
    cpu.r[4] = cpu.r[4] << 1;
    cpu.r[0] = mem.read_u32(cpu.r[3]).unwrap_or(0);
    cpu.r[0] = cpu.r[0].wrapping_add(4);
    cpu.r[0] = cpu.r[0].wrapping_add(cpu.r[4]); // delay slot of `rts`
}
