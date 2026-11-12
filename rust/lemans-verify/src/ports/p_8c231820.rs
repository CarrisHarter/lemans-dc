//! 0x8c231820 - set bit 6 of `[global+4]`, clear it in `[global]`, store r4 at +44.
//!
//! The store is the `bra` delay slot. `rts` delay zeroes r0. The loop at
//! 0x8c23182e is skipped.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[5] = mem.read_u32(0x8c231860).unwrap_or(0x8c2c65d0);
    cpu.r[6] = 64;
    cpu.r[3] = mem.read_u32(cpu.r[5]).unwrap_or(0);
    cpu.r[3] |= cpu.r[6];
    let _ = mem.write_u32(cpu.r[5], cpu.r[3]);
    let _ = mem.write_u32(cpu.r[5].wrapping_add(44), cpu.r[4]); // bra delay
    cpu.r[3] = mem.read_u32(cpu.r[5].wrapping_add(4)).unwrap_or(0);
    cpu.r[3] |= cpu.r[6];
    let _ = mem.write_u32(cpu.r[5].wrapping_add(4), cpu.r[3]);
    cpu.r[2] = mem.read_u32(cpu.r[5]).unwrap_or(0);
    cpu.r[3] = (-65i32) as u32;
    cpu.r[2] &= cpu.r[3];
    let _ = mem.write_u32(cpu.r[5], cpu.r[2]);
    cpu.r[0] = 0; // rts delay
}
