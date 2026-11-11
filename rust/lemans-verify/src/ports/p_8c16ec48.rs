//! 0x8c16ec48 — store_flag_and_float.
//!
//! `add #92,r4 ; r3=[r4] ; byte[r3+0x80]=r5 ; [r3+0x84]=fr4` — the two
//! offsets come from the literal pool, not immediates.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[4] = cpu.r[4].wrapping_add(92);
    cpu.r[3] = mem.read_u32(cpu.r[4]).unwrap_or(0);
    cpu.r[0] = mem.read_u32(0x8c16ec5c).unwrap_or(0);
    cpu.r[1] = mem.read_u32(0x8c16ec60).unwrap_or(0);
    let _ = mem.write_u8(cpu.r[0].wrapping_add(cpu.r[3]), cpu.r[5] as u8);
    cpu.r[0] = mem.read_u32(cpu.r[4]).unwrap_or(0);
    cpu.r[0] = cpu.r[0].wrapping_add(cpu.r[1]);
    let _ = mem.write_f32(cpu.r[0], cpu.fr[4]); // delay slot of `rts`
}
