//! 0x8c1959c8 — flag_to_code.
//!
//! Returns 6 when the byte at `0x8c2b79e8` is non-zero, else 5.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[0] = mem.read_u32(0x8c1959e0).unwrap_or(0);
    cpu.r[0] = mem.read_u8(cpu.r[0]).unwrap_or(0) as u32;
    cpu.r[0] = cpu.r[0] & 0xff;
    cpu.r[0] = cpu.r[0] & 0xff;
    cpu.set_t(cpu.r[0] == 0);
    if cpu.r[0] == 0 {
        cpu.r[0] = 5;
    } else {
        cpu.r[0] = 6; // delay slot of `rts`
    }
}
