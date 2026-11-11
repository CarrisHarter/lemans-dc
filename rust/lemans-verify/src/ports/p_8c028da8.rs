//! 0x8c028da8 — bit-test ladder on r5; returns 0x8c1f47c0 + string offset.
//!
//! Hidden side effects: r1 is left holding the chosen offset, r0 gets it
//! added in the `rts` delay slot, T is 0 whenever a bit matched (the last
//! `tst` saw a nonzero value) and 1 on the default path, and the final
//! `and r0,r5` test destroys r5 (r5 &= 0x80) on every path that reaches it.

use sh4_oracle::{Cpu, Memory};

const BASE: u32 = 0x8c1f47c0;

pub fn run(cpu: &mut Cpu, _mem: &mut Memory) {
    let x = cpu.r[5];
    let off: u32;
    if x & 0x01 != 0 {
        off = 0x92;
    } else if x & 0x02 != 0 {
        off = 0x97;
    } else if x & 0x04 != 0 {
        off = 0x9b;
    } else if x & 0x08 != 0 {
        off = 0xa2;
    } else if x & 0x10 != 0 {
        off = 0xa7;
    } else if x & 0x20 != 0 {
        off = 0xac;
    } else if x & 0x40 != 0 {
        off = 0xb1;
    } else if x & 0x200 != 0 {
        off = 0xb7;
    } else if x & 0x100 != 0 {
        off = 0xc3;
    } else {
        // Only this path reaches the final `and r0,r5` / `tst r5,r5` test.
        cpu.r[5] = x & 0x80;
        if x & 0x80 != 0 {
            off = 0xd0;
            cpu.set_t(false); // tst saw r5 = 0x80
        } else {
            off = 0xd4;
            cpu.set_t(true); // tst saw r5 = 0
        }
        cpu.r[0] = BASE.wrapping_add(off);
        cpu.r[1] = off;
        return;
    }
    // A bit matched: its `tst r0,r0` saw a nonzero value, so T = 0.
    cpu.set_t(false);
    cpu.r[0] = BASE.wrapping_add(off); // add r1,r0 in the rts delay slot
    cpu.r[1] = off;
}
