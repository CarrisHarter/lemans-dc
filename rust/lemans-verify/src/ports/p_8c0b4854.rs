//! 0x8c0b4854 — store_float_at_32.
//!
//! `[r4+32]=fr4 ; r0=0 ; rts ; [r4]=0` — `r0` ends as 0 and the byte at `[r4]`
//! is cleared.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let base = cpu.r[4];
    cpu.r[0] = base.wrapping_add(32);
    let _ = mem.write_f32(cpu.r[0], cpu.fr[4]);
    cpu.r[0] = 0;
    let _ = mem.write_u8(base, 0); // delay slot of `rts`
}
