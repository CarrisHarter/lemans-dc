//! 0x8c04d284 — `return *(undefined4 *)(param_2 * 4 + *(int *)(DAT_8c04d290 + param_1));`
//!
//! ```text
//! mov.l 0x8c04d290,r0   ; r0 = 0xe8 (literal pool)
//! shll2 r5
//! mov.l @(r0,r4),r1
//! mov   r5,r0
//! rts
//! mov.l @(r0,r1),r0     ; delay slot
//! ```
//! Hidden side effects: the pool constant 0xe8 is consumed by the indexed
//! load (not left visible), r5 is shifted in place, the table base stays in
//! r1, and the element load is in the rts delay slot.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let pool = mem.read_u32(0x8C04D290).unwrap();
    cpu.r[5] <<= 2; // shll2 r5
    cpu.r[1] = mem.read_u32(pool.wrapping_add(cpu.r[4])).unwrap();
    cpu.r[0] = cpu.r[5];
    let addr = cpu.r[0].wrapping_add(cpu.r[1]);
    cpu.r[0] = mem.read_u32(addr).unwrap(); // delay slot of `rts`
}
