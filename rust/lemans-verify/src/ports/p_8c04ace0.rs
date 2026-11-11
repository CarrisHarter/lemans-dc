//! 0x8c04ace0 — `return *(int*)(*(int*)(*(int*)(*p2*4 + *p1) + *p2*4) + 0x10) + p3*4;`
//!
//! ```text
//! mov.l @r5,r0
//! shll2 r6
//! mov.l @r4,r1
//! shll2 r0
//! mov.l @(r0,r1),r1
//! mov.l @(r0,r1),r0
//! mov.l @(16,r0),r0
//! rts
//! add   r6,r0        ; delay slot
//! ```
//! Hidden side effects: r6 is shifted in place (the C only uses `param_3 * 4`),
//! the intermediate pointer stays in r1, and the delay slot adds the *shifted*
//! r6 to the loaded value.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let mut r0 = mem.read_u32(cpu.r[5]).unwrap();
    cpu.r[6] <<= 2; // shll2 r6
    let mut r1 = mem.read_u32(cpu.r[4]).unwrap();
    r0 <<= 2; // shll2 r0
    r1 = mem.read_u32(r1.wrapping_add(r0)).unwrap();
    cpu.r[1] = r1;
    r0 = mem.read_u32(r1.wrapping_add(r0)).unwrap();
    r0 = mem.read_u32(r0.wrapping_add(0x10)).unwrap();
    cpu.r[0] = r0.wrapping_add(cpu.r[6]); // delay slot of `rts`
}
