//! 0x8c05d60c — `*param_2 = *param_1;` where param_2 is *pointed to* by r15.
//!
//! ```text
//! mov.l @r15,r1         ; r1 = [r15] = destination pointer
//! mov.l @r4,r0          ; r0 = [r4] = value
//! rts
//! mov.l r0,@r1          ; delay slot: [r1] = r0
//! ```
//! Hidden side effects: r1 ends holding the loaded destination pointer (the C
//! shows only the store), r0 keeps the source value; the store happens in the
//! rts delay slot. T is untouched.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let dst = mem.read_u32(cpu.r[15]).unwrap_or(0); // mov.l @r15,r1
    cpu.r[1] = dst;
    let val = mem.read_u32(cpu.r[4]).unwrap_or(0); // mov.l @r4,r0
    cpu.r[0] = val;
    let _ = mem.write_u32(dst, val); // rts delay slot
}
