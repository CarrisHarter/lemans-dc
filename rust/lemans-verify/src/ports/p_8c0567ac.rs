//! 0x8c0567ac — walk r1 up to r1 + count*0x38 in 0x38 steps, then zero [r4+4].
//!
//! ```text
//! mov.l @(4,r4),r0 ; mov.l @(8,r4),r1
//! r3 = r0*0x38 + r1          (shll2/shll/sub/shll2/shll/add)
//! cmp/hi r1,r3 ; bf done
//! loop: add #0x38,r1 ; cmp/hi r1,r3 ; bt loop
//! done: mov #0,r0 ; rts ; mov.l r0,@(4,r4)
//! ```
//! Leftovers the C does not show: r3 keeps the end sentinel, r1 ends as the
//! first multiple-of-0x38 step >= r3, r0 = 0, T = 0 (every exit is a
//! non-hi compare), and the store [r4+4] = 0 is the `rts` delay slot.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let base = cpu.r[4];
    let count = mem.read_u32(base.wrapping_add(4)).unwrap_or(0);
    let mut r1 = mem.read_u32(base.wrapping_add(8)).unwrap_or(0);
    let r3 = r1.wrapping_add(count.wrapping_mul(0x38));
    cpu.set_t(r3 > r1); // cmp/hi r1,r3
    while r3 > r1 {
        r1 = r1.wrapping_add(0x38); // add #56,r1
        cpu.set_t(r3 > r1); // cmp/hi r1,r3
    }
    cpu.r[0] = 0;
    cpu.r[1] = r1;
    cpu.r[3] = r3;
    let _ = mem.write_u32(base.wrapping_add(4), cpu.r[0]); // rts delay slot
}
