//! 0x8c081fd0 — set_mode_byte.
//!
//! Stores the low byte of `r5` at `r4+12`; when it is 1, sets bits 0-1 of the
//! `r4+16` word and then clears bit 2.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let base = cpu.r[4];
    cpu.r[0] = cpu.r[5];
    let _ = mem.write_u8(base + 12, cpu.r[0] as u8); // mov.b r0,@(12,r4)
    cpu.r[0] = cpu.r[5] & 0xFF; // extu.b r5,r0
    let is_one = cpu.r[0] == 1;
    cpu.set_t(is_one); // cmp/eq #1,r0
    if !is_one {
        return;
    }
    cpu.r[0] = mem.read_u32(base + 16).unwrap_or(0);
    cpu.r[1] = (-5i32) as u32;
    cpu.r[0] |= 3; // or #3,r0
    let _ = mem.write_u32(base + 16, cpu.r[0]);
    cpu.r[0] = mem.read_u32(base + 16).unwrap_or(0);
    cpu.r[0] &= cpu.r[1]; // and r1,r0
    let _ = mem.write_u32(base + 16, cpu.r[0]);
}
