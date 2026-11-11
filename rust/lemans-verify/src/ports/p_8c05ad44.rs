//! 0x8c05ad44 — `return *DAT_8c05ad4c;` (double dereference of a global).
//!
//! ```text
//! mov.l 0x8c05ad4c,r0   ; r0 = [0x8c05ad4c] = 0x8c23de10 (literal pool)
//! rts
//! mov.l @r0,r0          ; delay slot: r0 = [0x8c23de10]
//! ```
//! Hidden side effect: the rts delay slot performs the actual global load;
//! r0 ends as `*0x8c23de10`, and the literal is read through memory (the
//! pool entry at 0x8c05ad4c), not hardcoded. T is untouched.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let global = mem.read_u32(0x8c05ad4c).unwrap_or(0);
    cpu.r[0] = mem.read_u32(global).unwrap_or(0); // rts delay slot
}
