//! 0x8c0274f0 — `return *(u32 *)(param_2 * 4 + param_1 + DAT_8c0274fc);`
//!
//! ```text
//! mov.l 0x8c0274fc,r1   ; r1 = 0x144 (table base, left in r1)
//! shll2 r5              ; r5 *= 4 (argument scaled in place, no T change)
//! mov   r5,r0
//! add   r1,r4           ; r4 itself is mutated: r4 += 0x144
//! rts
//! mov.l @(r0,r4),r0     ; delay slot: final load uses the *mutated* r4
//! ```
//! Hidden side effects: r1/r5 leftovers and r4 advanced by the table base.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[1] = mem.read_u32(0x8C0274FC).unwrap(); // literal pool: 0x144
    cpu.r[5] = cpu.r[5] << 2; // shll2 r5 (does not touch T)
    cpu.r[0] = cpu.r[5];
    cpu.r[4] = cpu.r[4].wrapping_add(cpu.r[1]); // add r1,r4
    cpu.r[0] = mem.read_u32(cpu.r[0].wrapping_add(cpu.r[4])).unwrap(); // delay slot
}
