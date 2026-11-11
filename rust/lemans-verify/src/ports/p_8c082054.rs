//! 0x8c082054 — init_struct.
//!
//! Stores two pool words at `[r4]` (the second wins), zeroes `+4`, `+8`, the byte
//! at `+12` and `+20`, and puts 1 at `+16`. `r0` ends as `r4`.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let base = cpu.r[4];
    let first = mem.read_u32(0x8C08_2070).unwrap_or(0);
    let second = mem.read_u32(0x8C08_2074).unwrap_or(0);
    cpu.r[0] = first;
    let _ = mem.write_u32(base, first);
    cpu.r[0] = second;
    let _ = mem.write_u32(base, second);
    cpu.r[0] = 0;
    cpu.r[1] = 1;
    let _ = mem.write_u32(base + 4, 0);
    let _ = mem.write_u32(base + 8, 0);
    let _ = mem.write_u8(base + 12, 0);
    let _ = mem.write_u32(base + 16, 1);
    let _ = mem.write_u32(base + 20, 0);
    cpu.r[0] = base; // delay slot of `rts`
}
