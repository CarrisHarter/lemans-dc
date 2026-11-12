//! 0x8c231eca - code from the size at `[r4+12]` and bit 31 of `[r4+24]`.
//!
//! 8 -> 1/2, 16 -> 3/4, 32 -> 5/6, 64 -> 7/8, anything else -> 5. The high
//! code of each pair is the bit-set arm. `r5` is the mask 0x80000000.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[0] = mem.read_u32(cpu.r[4].wrapping_add(12)).unwrap_or(0);
    cpu.r[5] = mem.read_u32(0x8c231f00).unwrap_or(0x8000_0000);
    let size = cpu.r[0];
    cpu.set_t(size == 8);
    if cpu.t() {
        cpu.r[3] = mem.read_u32(cpu.r[4].wrapping_add(24)).unwrap_or(0);
        cpu.set_t((cpu.r[3] & cpu.r[5]) == 0);
        cpu.r[0] = if cpu.t() { 1 } else { 2 };
        return;
    }
    cpu.set_t(size == 16);
    if cpu.t() {
        cpu.r[3] = mem.read_u32(cpu.r[4].wrapping_add(24)).unwrap_or(0);
        cpu.set_t((cpu.r[3] & cpu.r[5]) == 0);
        cpu.r[0] = if cpu.t() { 3 } else { 4 };
        return;
    }
    cpu.set_t(size == 32);
    if cpu.t() {
        cpu.r[3] = mem.read_u32(cpu.r[4].wrapping_add(24)).unwrap_or(0);
        cpu.set_t((cpu.r[3] & cpu.r[5]) == 0);
        cpu.r[0] = if cpu.t() { 5 } else { 6 };
        return;
    }
    cpu.set_t(size == 64);
    if cpu.t() {
        cpu.r[3] = mem.read_u32(cpu.r[4].wrapping_add(24)).unwrap_or(0);
        cpu.set_t((cpu.r[3] & cpu.r[5]) == 0);
        cpu.r[0] = if cpu.t() { 7 } else { 8 };
        return;
    }
    cpu.r[0] = 5;
}
