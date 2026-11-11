//! 0x8c07bcd8 — pick the larger of two counters and clamp it to INT_MAX.
//!
//! ```text
//! r3 = [0x8c07bd08] = 0x8c1f85f0 ; r4 = [0x8c07bd0c] = 0x8c1f85f8
//! r1 = [r3] ; r0 = [r4] ; cmp/hi r1,r0 ; bf.s pick_first ; r15 -= 8
//! bra clamp ; nop
//! pick_first: r4 = r3
//! clamp: r1 = [r4] ; r0 = 0x7fffffff
//!        if (r1 > 0x7fffffff) r0 = 0x7fffffff else r0 = r1
//!        rts ; r15 += 8
//! ```
//!
//! Hidden side effects: `r4` ends as whichever pointer was selected, `r3` keeps
//! the first address, `r1` keeps the selected value, and T is the `cmp/hi`
//! result of the final clamp test.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let first = mem.read_u32(0x8C07_BD08).unwrap_or(0);
    let second = mem.read_u32(0x8C07_BD0C).unwrap_or(0);
    cpu.r[3] = first;
    cpu.r[4] = second;

    let a = mem.read_u32(first).unwrap_or(0);
    let b = mem.read_u32(second).unwrap_or(0);
    cpu.r[1] = a;
    cpu.r[0] = b;
    let second_bigger = b > a;
    cpu.set_t(second_bigger); // cmp/hi r1,r0
    if !second_bigger {
        cpu.r[4] = first; // bf.s target
    }

    cpu.r[1] = mem.read_u32(cpu.r[4]).unwrap_or(0);
    cpu.r[0] = 0x7FFF_FFFF;
    let over = cpu.r[1] > 0x7FFF_FFFF;
    cpu.set_t(over); // cmp/hi r0,r1
    if !over {
        cpu.r[0] = cpu.r[1];
    }
}
