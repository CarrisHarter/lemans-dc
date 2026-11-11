//! 0x8c19d33c — clear_float_and_byte.
//!
//! `[r4+16]=0.0 ; mov #0,r0 ; rts ; byte[r4+8]=r0`.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[0] = cpu.r[4].wrapping_add(16);
    cpu.fr[0] = 0.0;
    let _ = mem.write_f32(cpu.r[0], cpu.fr[0]);
    cpu.r[0] = 0;
    let _ = mem.write_u8(cpu.r[4] + 8, cpu.r[0] as u8); // delay slot of `rts`
}
