//! 0x8c05dbfc — `*param_1 = *param_2; return param_1;`
//!
//! ```text
//! mov.l @r5,r1          ; r1 = [r5]
//! mov   r4,r0           ; r0 = r4 (return value)
//! rts
//! mov.l r1,@r4          ; delay slot: [r4] = r1
//! ```
//! Hidden side effects: r1 ends holding the loaded source value, r0 ends as
//! the *original* r4 (the C's `return param_1`); the store happens in the rts
//! delay slot. T is untouched.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let src = mem.read_u32(cpu.r[5]).unwrap_or(0); // mov.l @r5,r1
    cpu.r[1] = src;
    cpu.r[0] = cpu.r[4]; // mov r4,r0
    let _ = mem.write_u32(cpu.r[4], src); // rts delay slot
}
