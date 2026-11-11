//! 0x8c17fffc — float_at_64.
//!
//! `add #64,r4 ; rts ; fr0=[r4]` — the advance sticks and the load is
//! the delay slot.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[4] = cpu.r[4].wrapping_add(64);
    cpu.fr[0] = mem.read_f32(cpu.r[4]).unwrap_or(0.0); // delay slot of `rts`
}
