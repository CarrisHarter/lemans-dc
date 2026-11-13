//! 0x8c23a740 - store r6 into an entry slot selected by r5.
//!
//! The search at 0x8c237780 must hit, and `r5` must be strictly below the
//! limit at `[r4+12]`. The slot is `[r4+72+r5*8]`. When `r5+1` equals the
//! limit, bit 28 of r6 is cleared; otherwise it is set. A miss or an index
//! past the limit returns 0xeeee.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[14]);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr);
    cpu.r[15] = cpu.r[15].wrapping_sub(8);
    cpu.r[3] = mem.read_u32(0x8c23a7a0).unwrap_or(0x8c237780);
    cpu.r[14] = cpu.r[5];
    let _ = mem.write_u32(cpu.r[15], cpu.r[4]);
    let _ = mem.write_u32(cpu.r[15].wrapping_add(4), cpu.r[6]);
    cpu.r[4] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.pr = 0x8c23a752;
    crate::registry::p_8c237780::run(cpu, mem);
    cpu.set_t(cpu.r[0] == 0);
    let mut fail = cpu.t();
    if !fail {
        cpu.r[4] = mem.read_u32(cpu.r[15]).unwrap_or(0);
        cpu.r[3] = mem.read_u32(cpu.r[4].wrapping_add(12)).unwrap_or(0);
        cpu.set_t(cpu.r[14] >= cpu.r[3]);
        fail = cpu.t();
    }
    if fail {
        cpu.r[0] = mem.read_u32(0x8c23a7ac).unwrap_or(0xeeee);
    } else {
        cpu.r[2] = mem.read_u32(cpu.r[4].wrapping_add(12)).unwrap_or(0);
        cpu.r[3] = cpu.r[14].wrapping_add(1);
        cpu.set_t(cpu.r[2] == cpu.r[3]);
        cpu.r[5] = mem.read_u32(cpu.r[15].wrapping_add(4)).unwrap_or(0); // bt.s delay
        if cpu.t() {
            cpu.r[2] = mem.read_u32(0x8c23a7a8).unwrap_or(0xefffffff);
            cpu.r[5] &= cpu.r[2];
        } else {
            cpu.r[1] = mem.read_u32(0x8c23a7a4).unwrap_or(0x10000000);
            cpu.r[5] |= cpu.r[1];
        }
        cpu.r[3] = cpu.r[4];
        cpu.r[14] = cpu.r[14].wrapping_shl(2); // shll2, T untouched
        cpu.r[3] = cpu.r[3].wrapping_add(72);
        cpu.set_t((cpu.r[14] >> 31) & 1 == 1); // shll
        cpu.r[14] = cpu.r[14].wrapping_shl(1);
        cpu.r[14] = cpu.r[3].wrapping_add(cpu.r[14]);
        let _ = mem.write_u32(cpu.r[14], cpu.r[5]);
        cpu.r[0] = 0;
    }
    cpu.r[15] = cpu.r[15].wrapping_add(8);
    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[14] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}
