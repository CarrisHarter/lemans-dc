//! 0x8c0c01ec — copy_fields_0_8_12.
//!
//! Copies `[r5]`, `[r5+8]` and `[r5+12]` to the same offsets in `r4`; the last
//! store is the delay slot and the `+4` field is left alone.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let dst = cpu.r[4];
    let src = cpu.r[5];
    cpu.r[0] = mem.read_u32(src).unwrap_or(0);
    let _ = mem.write_u32(dst, cpu.r[0]);
    cpu.r[0] = mem.read_u32(src + 8).unwrap_or(0);
    let _ = mem.write_u32(dst + 8, cpu.r[0]);
    cpu.r[0] = mem.read_u32(src + 12).unwrap_or(0);
    let _ = mem.write_u32(dst + 12, cpu.r[0]); // delay slot of `rts`
}
