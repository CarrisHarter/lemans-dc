//! 0x8c0b5ec0 — reset_three_globals.
//!
//! Loads three pool pointers, zeroes the first and moves the third's value into
//! the second; the second store is the delay slot.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let p3 = mem.read_u32(0x8C0B_5ED0).unwrap_or(0);
    cpu.r[0] = p3;
    cpu.r[3] = 0;
    cpu.r[1] = mem.read_u32(0x8C0B_5ED4).unwrap_or(0);
    cpu.r[4] = mem.read_u32(0x8C0B_5ED8).unwrap_or(0);
    cpu.r[0] = mem.read_u32(cpu.r[0]).unwrap_or(0);
    let _ = mem.write_u32(cpu.r[4], 0);
    let _ = mem.write_u32(cpu.r[1], cpu.r[0]); // delay slot of `rts`
}
