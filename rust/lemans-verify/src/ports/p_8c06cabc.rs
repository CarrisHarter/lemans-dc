//! 0x8c06cabc — advance_arg_8_store_float.
//!
//! `add #8,r4 ; rts ; fmov fr4,@r4` — r4 is advanced before the delay-slot store.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[4] = cpu.r[4].wrapping_add(8);
    let _ = mem.write_f32(cpu.r[4], cpu.fr[4]); // delay slot of `rts`
}
