//! 0x8c10efc4 — `*param_1 = 0; return param_1;`
//!
//! ```text
//! mov   #0,r0
//! mov.b r0,@r4           ; store a zero byte at param_1 (outside stack window)
//! rts
//! mov   r4,r0            ; delay slot: return param_1
//! ```
//! Hidden side effects: the zeroing byte store at r4, and r0 ends as r4
//! (not 0) because the rts delay slot overwrites it.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[0] = 0; // mov #0,r0
    let _ = mem.write_u8(cpu.r[4], 0); // mov.b r0,@r4
    cpu.r[0] = cpu.r[4]; // delay slot of `rts`
}
