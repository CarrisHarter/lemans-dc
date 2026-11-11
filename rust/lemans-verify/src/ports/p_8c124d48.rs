//! 0x8c124d48 — `*param_2 = *param_1;` with param_2 fetched from the stack.
//!
//! ```text
//! mov.l @r15,r1   ; r1 = param_2 (top of stack)
//! mov.l @r4,r0    ; r0 = *param_1
//! rts
//! mov.l r0,@r1    ; delay slot: the actual store
//! ```
//! Hidden side effects: r1 and r0 are both left clobbered, and the store lives
//! in the `rts` delay slot.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let r1 = mem.read_u32(cpu.r[15]).unwrap(); // mov.l @r15,r1
    let r0 = mem.read_u32(cpu.r[4]).unwrap(); // mov.l @r4,r0
    let _ = mem.write_u32(r1, r0); // delay slot of rts: mov.l r0,@r1
    cpu.r[1] = r1;
    cpu.r[0] = r0;
}
