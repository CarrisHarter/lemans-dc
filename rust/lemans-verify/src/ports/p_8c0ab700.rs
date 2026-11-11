//! 0x8c0ab700 — store_two_floats_at_f4_f8.
//!
//! `[r4+0xf4]=fr4 ; [r4+0xf8]=fr5` — the second store is the `rts` delay slot and
//! `r4` is advanced by 0xf8.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let base = cpu.r[4];
    let off1 = mem.read_u32(0x8C0A_B710).unwrap_or(0xf4);
    let off2 = mem.read_u32(0x8C0A_B714).unwrap_or(0xf8);
    cpu.r[0] = off1;
    cpu.r[1] = base.wrapping_add(off1);
    cpu.r[0] = off2;
    let _ = mem.write_f32(cpu.r[1], cpu.fr[4]);
    cpu.r[4] = base.wrapping_add(off2);
    let _ = mem.write_f32(cpu.r[4], cpu.fr[5]); // delay slot of `rts`
}
