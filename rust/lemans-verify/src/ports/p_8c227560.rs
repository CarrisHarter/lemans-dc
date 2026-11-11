//! 0x8c227560 — set_bit_from_flag.
//!
//! Returns 10 when bit 0x38000 of `[r4+4]` is set; otherwise clears bits 1-2 of
//! `[r4]`, inserts `r5 & 1` at bit 1, and returns 0.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[2] = mem.read_u32(cpu.r[4] + 4).unwrap_or(0);
    cpu.r[3] = mem.read_u32(0x8c2275a0).unwrap_or(0);
    let t = (cpu.r[3] & cpu.r[2]) == 0;
    cpu.set_t(t);
    if t {
        cpu.r[6] = cpu.r[4];
        cpu.r[1] = mem.read_u32(cpu.r[6]).unwrap_or(0);
        cpu.r[3] = 1;
        cpu.r[2] = (-3i32) as u32;
        cpu.r[5] = cpu.r[5] & cpu.r[3];
        cpu.r[1] = cpu.r[1] & cpu.r[2];
        cpu.set_t((cpu.r[5] >> 31) & 1 != 0); // `shll` sets T from the bit shifted out
        cpu.r[5] = cpu.r[5] << 1;
        cpu.r[5] = cpu.r[5] | cpu.r[1];
        cpu.r[0] = 0;
        let _ = mem.write_u32(cpu.r[6], cpu.r[5]);
    } else {
        cpu.r[0] = 10; // delay slot of `rts`
    }
}
