//! 0x8c198b3c — reset_and_set_state_2.
//!
//! Zeroes `byte[r4+61]`, stores 1.0 at `[r4+80]`, then sets `[r4+56]=2`
//! in the `rts` delay slot.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[0] = 61;
    cpu.r[3] = 0;
    cpu.r[1] = cpu.r[4];
    let _ = mem.write_u8(cpu.r[4].wrapping_add(cpu.r[0]), cpu.r[3] as u8);
    cpu.r[1] = cpu.r[1].wrapping_add(80);
    cpu.fr[0] = 1.0;
    let _ = mem.write_f32(cpu.r[1], cpu.fr[0]);
    cpu.r[0] = 2;
    let _ = mem.write_u32(cpu.r[4] + 56, cpu.r[0]); // delay slot of `rts`
}
