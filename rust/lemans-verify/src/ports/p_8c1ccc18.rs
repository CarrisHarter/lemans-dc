//! 0x8c1ccc18 — init_record_a.
//!
//! Initialises a record: flag 1, zeros at +4/+12/+16, 1.0 at +8, 1 at +20, and
//! zero at `+0x178c`.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[3] = 1;
    let _ = mem.write_u8(cpu.r[4], cpu.r[3] as u8);
    cpu.r[1] = 0;
    cpu.r[0] = cpu.r[4];
    let _ = mem.write_u32(cpu.r[4] + 4, cpu.r[1]);
    cpu.r[0] = cpu.r[0].wrapping_add(8);
    cpu.fr[0] = 1.0;
    let _ = mem.write_f32(cpu.r[0], cpu.fr[0]);
    let _ = mem.write_u32(cpu.r[4] + 12, cpu.r[1]);
    cpu.r[0] = 20;
    let _ = mem.write_u32(cpu.r[4] + 16, cpu.r[1]);
    let _ = mem.write_u8(cpu.r[4].wrapping_add(cpu.r[0]), cpu.r[3] as u8);
    cpu.r[0] = mem.read_u32(0x8c1ccc38).unwrap_or(0);
    let _ = mem.write_u32(cpu.r[4].wrapping_add(cpu.r[0]), cpu.r[1]); // delay slot of `rts`
}
