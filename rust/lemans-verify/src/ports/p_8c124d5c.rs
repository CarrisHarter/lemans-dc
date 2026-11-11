//! 0x8c124d5c — `*param_2 = **param_1;` (double dereference).
//!
//! ```text
//! mov.l @r15,r1   ; r1 = param_2
//! mov.l @r4,r0    ; r0 = *param_1
//! mov.l @r0,r0    ; r0 = **param_1
//! rts
//! mov.l r0,@r1    ; delay slot: store
//! ```
//! Hidden side effect: r0 ends as the doubly-dereferenced value; the C shows
//! only the store.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let r1 = mem.read_u32(cpu.r[15]).unwrap(); // mov.l @r15,r1
    let r0 = mem.read_u32(cpu.r[4]).unwrap(); // mov.l @r4,r0
    let r0 = mem.read_u32(r0).unwrap(); // mov.l @r0,r0
    let _ = mem.write_u32(r1, r0); // delay slot of rts
    cpu.r[1] = r1;
    cpu.r[0] = r0;
}
