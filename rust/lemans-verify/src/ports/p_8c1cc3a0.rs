//! 0x8c1cc3a0 — set_two_flags.
//!
//! Writes 1 to `byte[0x8c2b8758]` and `byte[0x8c215d88]`; the second is the
//! `rts` delay slot.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[3] = mem.read_u32(0x8c1cc3ac).unwrap_or(0);
    cpu.r[1] = 1;
    cpu.r[0] = mem.read_u32(0x8c1cc3b0).unwrap_or(0);
    let _ = mem.write_u8(cpu.r[3], cpu.r[1] as u8);
    let _ = mem.write_u8(cpu.r[0], cpu.r[1] as u8); // delay slot of `rts`
}
