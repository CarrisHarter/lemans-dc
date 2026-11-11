//! 0x8c154ec4 — round_up_to_12_boundary.
//!
//! Rounds `r1` up by twelves until it reaches `12*r0 + r1`, then zeroes
//! `[r4+4]`. `r3` keeps the target and `r1` the rounded value; `T` is the
//! last `cmp/hi`.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let r0 = mem.read_u32(cpu.r[4] + 4).unwrap_or(0);
    let mut r1 = mem.read_u32(cpu.r[4] + 8).unwrap_or(0);
    let mut r3 = r0;
    r3 = r3 << 1;
    r3 = r3.wrapping_add(r0);
    r3 = r3 << 2;
    r3 = r3.wrapping_add(r1);
    let mut t = r3 > r1;
    cpu.set_t(t);
    if t {
        loop {
            r1 = r1.wrapping_add(12);
            t = r3 > r1;
            cpu.set_t(t);
            if !t {
                break;
            }
        }
    }
    cpu.r[1] = r1;
    cpu.r[3] = r3;
    cpu.r[0] = 0;
    let _ = mem.write_u32(cpu.r[4] + 4, 0); // delay slot of `rts`
}
