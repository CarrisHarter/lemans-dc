//! 0x8c10e3c4 — `return *param_1 + 12;`
//!
//! ```text
//! mov.l @r4,r0
//! rts
//! add   #12,r0     ; delay slot: the add IS the return value adjustment
//! ```
//! The `add #12` sits in the `rts` delay slot, so it runs unconditionally
//! after the load; no T-bit change (`add #imm` leaves T alone).

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let v = mem.read_u32(cpu.r[4]).unwrap();
    cpu.r[0] = v.wrapping_add(12); // delay slot of `rts`
}
