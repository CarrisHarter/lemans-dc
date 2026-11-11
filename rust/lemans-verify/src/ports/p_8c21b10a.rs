//! 0x8c21b10a — store_two_fields_10e0.
//!
//! Stores `r4` at `+0x10e0` and `+0x10e4`; the second store is the `rts` delay
//! slot and reloads the object pointer in between.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[5] = mem.read_u32(0x8c21b170).unwrap_or(0);
    cpu.r[0] = 0x10e0;
    cpu.r[3] = mem.read_u32(cpu.r[5]).unwrap_or(0);
    let _ = mem.write_u32(cpu.r[0].wrapping_add(cpu.r[3]), cpu.r[4]);
    cpu.r[0] = cpu.r[0].wrapping_add(4);
    cpu.r[3] = mem.read_u32(cpu.r[5]).unwrap_or(0);
    let _ = mem.write_u32(cpu.r[0].wrapping_add(cpu.r[3]), cpu.r[4]); // delay slot of `rts`

}
