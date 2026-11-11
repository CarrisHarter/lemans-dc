//! 0x8c172e10 — invert_table_byte_if_enabled.
//!
//! If the global at `0x8c2b5414` is 1, returns `byte[0x8c20d454 + r4] ^ 0xff`;
//! otherwise returns `r4` unchanged.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let r3 = mem.read_u32(0x8c172ef8).unwrap_or(0);
    let r0 = mem.read_u32(r3).unwrap_or(0);
    cpu.r[3] = r3;
    cpu.set_t(r0 == 1);
    if r0 == 1 {
        cpu.r[0] = mem.read_u32(0x8c172efc).unwrap_or(0);
        cpu.r[4] = cpu.r[4] & 0xff;
        cpu.r[1] = mem.read_u16(0x8c172ef4).unwrap_or(0) as i16 as i32 as u32;
        let mut r2 = mem.read_u8(cpu.r[0].wrapping_add(cpu.r[4])).unwrap_or(0) as u32;
        r2 = r2 & 0xff;
        r2 ^= cpu.r[1];
        cpu.r[2] = r2;
        cpu.r[0] = r2; // delay slot of `rts`
    } else {
        cpu.r[0] = cpu.r[4];
    }
}
