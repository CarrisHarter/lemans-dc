//! 0x8c1730fe — increment_global_r4_times.
//!
//! Tests `r4` first, then loops through the shared body at `0x8c1730f6` which
//! increments the counter at `0x8c2b5520` (pool word at `0x8c17315c`). The
//! `add #-1,r4` is the `bf.s` delay slot, so it runs even on the fall-through:
//! exit `r4` is always `0xffffffff` and `T=1`. Entry `N>0` runs `N` iterations;
//! `N=0` touches neither `r1`, `r2` nor the counter.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    loop {
        let t = cpu.r[4] == 0;
        cpu.set_t(t);
        cpu.r[4] = cpu.r[4].wrapping_sub(1); // delay slot of `bf.s`
        if t {
            break;
        }
        // Loop body at 0x8c1730f6.
        let r1 = mem.read_u32(0x8c17315c).unwrap_or(0);
        let r2 = mem.read_u32(r1).unwrap_or(0).wrapping_add(1);
        let _ = mem.write_u32(r1, r2);
        cpu.r[1] = r1;
        cpu.r[2] = r2;
    }
}
