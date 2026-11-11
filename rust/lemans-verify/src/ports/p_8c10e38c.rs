//! 0x8c10e38c — load_byte_via_field_80.
//!
//! `r0=[r4] ; r0+=80 ; rts ; r0=[r0]` — a byte through the pointer at `[r4]`.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[0] = mem.read_u32(cpu.r[4]).unwrap_or(0);
    cpu.r[0] = cpu.r[0].wrapping_add(80);
    cpu.r[0] = mem.read_u8(cpu.r[0]).unwrap_or(0) as i8 as i32 as u32; // delay slot
}
