//! 0x8c1cc338 — conditional_global_byte.
//!
//! If the byte at `0x8c2b8738` is non-zero returns the byte behind the second
//! pool pointer, else 0.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[0] = mem.read_u32(0x8c1cc354).unwrap_or(0);
    cpu.r[0] = mem.read_u8(cpu.r[0]).unwrap_or(0) as u32;
    cpu.r[0] = cpu.r[0] & 0xff;
    cpu.r[0] = cpu.r[0] & 0xff;
    let t = cpu.r[0] == 0;
    cpu.set_t(t);
    if t {
        cpu.r[0] = 0;
    } else {
        cpu.r[0] = mem.read_u32(0x8c1cc358).unwrap_or(0);
        cpu.r[0] = mem.read_u8(cpu.r[0]).unwrap_or(0) as u32;
        cpu.r[0] = cpu.r[0] & 0xff; // delay slot of `rts`
    }
}
