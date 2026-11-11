//! 0x8c19ae78 — init_two_pointers.
//!
//! Stores the pool words `0x8c1e8fec` then `0x8c1f13c0` at `[r4]` (the second
//! wins), then zeroes the three words at `r4+4`.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[4]);
    cpu.r[0] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[3] = mem.read_u32(0x8c19ae98).unwrap_or(0);
    cpu.r[1] = mem.read_u32(0x8c19ae9c).unwrap_or(0);
    let _ = mem.write_u32(cpu.r[0], cpu.r[3]);
    let _ = mem.write_u32(cpu.r[0], cpu.r[1]);
    cpu.r[3] = cpu.r[0];
    cpu.r[3] = cpu.r[3].wrapping_add(4);
    cpu.r[1] = 0;
    let _ = mem.write_u32(cpu.r[3], cpu.r[1]);
    let _ = mem.write_u32(cpu.r[3] + 4, cpu.r[1]);
    let _ = mem.write_u32(cpu.r[3] + 8, cpu.r[1]);
    cpu.r[15] = cpu.r[15].wrapping_add(4); // delay slot of `rts`
}
