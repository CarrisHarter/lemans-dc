//! 0x8c1ccc68 — init_record_c.
//!
//! Same as 0x8c1ccc3c but returns `r4` instead of the last pool word.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[0] = 1;
    let _ = mem.write_u8(cpu.r[4], cpu.r[0] as u8);
    cpu.r[1] = 0;
    cpu.r[0] = cpu.r[4];
    let _ = mem.write_u32(cpu.r[4] + 4, cpu.r[1]);
    cpu.r[0] = cpu.r[0].wrapping_add(8);
    cpu.fr[0] = 1.0;
    let _ = mem.write_f32(cpu.r[0], cpu.fr[0]);
    let _ = mem.write_u32(cpu.r[4] + 12, cpu.r[1]);
    cpu.r[0] = 20;
    let _ = mem.write_u32(cpu.r[4] + 16, cpu.r[1]);
    let _ = mem.write_u8(cpu.r[4].wrapping_add(cpu.r[0]), cpu.r[1] as u8);
    cpu.r[0] = mem.read_u32(0x8c1ccc8c).unwrap_or(0);
    let _ = mem.write_u32(cpu.r[4].wrapping_add(cpu.r[0]), cpu.r[1]);
    cpu.r[0] = mem.read_u32(0x8c1ccc90).unwrap_or(0);
    let _ = mem.write_u32(cpu.r[4].wrapping_add(cpu.r[0]), cpu.r[1]);
    cpu.r[0] = cpu.r[4]; // delay slot of `rts`
}
