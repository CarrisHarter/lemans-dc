//! 0x8c21b07a — load_field_10c0_high_half.
//!
//! Returns the high 16 bits of `[[0x8c2ba634]+0x10c0]`, zero-extended.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[3] = mem.read_u32(0x8c21b170).unwrap_or(0);
    cpu.r[1] = mem.read_u16(0x8c21b156).unwrap_or(0) as i16 as i32 as u32;
    cpu.r[0] = mem.read_u32(cpu.r[3]).unwrap_or(0);
    cpu.r[0] = mem.read_u32(cpu.r[0].wrapping_add(cpu.r[1])).unwrap_or(0);
    cpu.set_t(cpu.r[0] >> 16 & 1 != 0); // shlr16 r0 sets T from bit 16
    cpu.r[0] = cpu.r[0] >> 16;
    cpu.r[0] = cpu.r[0] & 0xffff; // exts.w then extu.w in the delay slot
}
