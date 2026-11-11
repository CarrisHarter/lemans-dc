//! 0x8c024c3c — `*(f32 *)(param_3 + 0x14) = fr4;` (param_3 arrives in r4)
//!
//! `add #20,r4 ; rts ; fmov fr4,@r4` — r4 is advanced BEFORE the delay-slot
//! store, so the original writes at r4+0x14 and leaves r4 advanced. T is
//! never touched.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[4] = cpu.r[4].wrapping_add(20);
    let _ = mem.write_u32(cpu.r[4], cpu.fr[4].to_bits());
}
