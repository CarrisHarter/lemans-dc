//! 0x8c13d8b4 — free the array at `[r4+8]` after walking `count * 8` bytes.
//!
//! `end = base + (count << 3)` (`shll2` then `shll`; `shll` sets T, then
//! `cmp/hi` overwrites it). The cursor steps by 8 until it is not below `end`.
//! A null base exits with T=1 and no call. The free path leaves the callee's T.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let base_slot = cpu.r[4];
    cpu.r[5] = mem.read_u32(base_slot.wrapping_add(8)).unwrap_or(0);
    cpu.set_t(cpu.r[5] == 0);
    if cpu.r[5] != 0 {
        let count = mem.read_u32(base_slot.wrapping_add(4)).unwrap_or(0);
        cpu.r[1] = cpu.r[5];
        cpu.r[0] = count.wrapping_shl(2);
        cpu.set_t(cpu.r[0] >> 31 != 0);
        cpu.r[0] = cpu.r[0].wrapping_shl(1);
        cpu.r[1] = cpu.r[1].wrapping_add(cpu.r[0]);
        cpu.set_t(cpu.r[1] > cpu.r[5]);
        cpu.r[3] = cpu.r[5];
        if cpu.t() {
            loop {
                cpu.r[3] = cpu.r[3].wrapping_add(8);
                cpu.set_t(cpu.r[1] > cpu.r[3]);
                if !cpu.t() {
                    break;
                }
            }
        }
        cpu.r[0] = mem.read_u32(0x8c13d8e0).unwrap_or(0x8c01032c);
        cpu.r[4] = cpu.r[5];
        cpu.pr = 0x8c13d8d8;
        crate::registry::p_8c01032c::run(cpu, mem);
    }
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}
