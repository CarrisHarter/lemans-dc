//! 0x8c19d39c — init_object.
//!
//! Sets `[r4]=0x8c1f1574`, `[r4+4]=r5`, `byte[r4+8]=0`, `[r4+16]=0.0`,
//! `[r4+12]=fr4`, and returns `r4`.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[0] = mem.read_u32(0x8c19d3b8).unwrap_or(0);
    cpu.r[1] = cpu.r[4];
    cpu.r[1] = cpu.r[1].wrapping_add(16);
    let _ = mem.write_u32(cpu.r[4], cpu.r[0]);
    cpu.fr[0] = 0.0;
    let _ = mem.write_u32(cpu.r[4] + 4, cpu.r[5]);
    cpu.r[0] = 0;
    let _ = mem.write_u8(cpu.r[4] + 8, cpu.r[0] as u8);
    cpu.r[0] = cpu.r[4];
    let _ = mem.write_f32(cpu.r[1], cpu.fr[0]);
    cpu.r[0] = cpu.r[0].wrapping_add(12);
    let _ = mem.write_f32(cpu.r[0], cpu.fr[4]);
    cpu.r[0] = cpu.r[4]; // delay slot of `rts`
}
