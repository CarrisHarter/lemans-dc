//! 0x8c0b4828 — scale_float_at_32.
//!
//! `fr0=[r4+32] ; fr0*=fr4 ; [r4+32]=fr0 ; r0=0 ; rts ; [r4]=0` — scales the
//! float at +32 and clears the byte at `[r4]`.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let base = cpu.r[4];
    cpu.r[1] = base.wrapping_add(32);
    cpu.fr[0] = mem.read_f32(cpu.r[1]).unwrap_or(0.0);
    cpu.r[0] = 0;
    cpu.fr[0] = cpu.fr[0] * cpu.fr[4];
    let _ = mem.write_f32(cpu.r[1], cpu.fr[0]);
    let _ = mem.write_u8(base, 0); // delay slot of `rts`
}
