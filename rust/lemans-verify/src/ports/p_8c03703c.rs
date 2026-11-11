//! 0x8c03703c — `*param_2 = **param_1;`
//!
//! ```text
//! mov.l @r4,r0
//! mov.l @r15,r1               ; r1 = return slot (stack)
//! mov.l @r0,r0
//! rts
//! mov.l r0,@r1                ; delay slot: store through the stack slot
//! ```
//! Hidden side effects: r0 is loaded twice (pointer, then value) and ends as
//! the value; r1 ends as the `[r15]` slot pointer; the delay-slot store lands
//! in the synthetic stack window, so it is not part of the diff.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[0] = mem.read_u32(cpu.r[4]).unwrap();
    cpu.r[1] = mem.read_u32(cpu.r[15]).unwrap();
    cpu.r[0] = mem.read_u32(cpu.r[0]).unwrap();
    let _ = mem.write_u32(cpu.r[1], cpu.r[0]); // delay slot of `rts`
}
