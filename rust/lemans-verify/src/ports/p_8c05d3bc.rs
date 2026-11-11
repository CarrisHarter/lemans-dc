//! 0x8c05d3bc — `return *(undefined4 *)PTR_DAT_8c05d3c4;` (global fetch).
//!
//! ```text
//! mov.l 0x8c05d3c4,r0   ; r0 = [0x8c05d3c4] = 0x8c1f72a0 (literal pool)
//! rts
//! mov.l @r0,r0          ; delay slot: r0 = [0x8c1f72a0]
//! ```
//! Hidden side effect: the rts delay slot performs the global load; r0 ends
//! as `*0x8c1f72a0`, with the pointer itself read through the literal pool.
//! T is untouched.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let global = mem.read_u32(0x8c05d3c4).unwrap_or(0);
    cpu.r[0] = mem.read_u32(global).unwrap_or(0); // rts delay slot
}
