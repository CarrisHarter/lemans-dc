//! 0x8c07a2a8 — dec_inc_wrap.
//!
//! Decrement `[r4+4]`, increment `[r4+8]`, then wrap it modulo `[r4]` when it
//! reaches the period. T is the `cmp/hs` result from before the wrap.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let base = cpu.r[4];
    let mut a = mem.read_u32(base + 4).unwrap_or(0);
    a = a.wrapping_sub(1);
    let _ = mem.write_u32(base + 4, a);

    let mut b = mem.read_u32(base + 8).unwrap_or(0);
    b = b.wrapping_add(1);
    let _ = mem.write_u32(base + 8, b);

    let period = mem.read_u32(base).unwrap_or(0);
    let reached = b >= period;
    cpu.set_t(reached); // cmp/hs r1,r0
    cpu.r[1] = period;
    cpu.r[0] = b;
    if reached {
        cpu.r[0] = b.wrapping_sub(period);
        let _ = mem.write_u32(base + 8, cpu.r[0]);
    }
}
