//! 0x8c017f24 — returns 0 when the three u32s at r4 are all zero, else 1.
//!
//! T is the result of the last tst executed: (w0 == 0), (w1 == 0) or
//! (w2 == 0) depending on where the bf chain bails.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let r4 = cpu.r[4];
    let w0 = mem.read_u32(r4).unwrap_or(0);
    cpu.set_t(w0 == 0);
    if w0 != 0 {
        cpu.r[0] = 1;
        return;
    }
    let w1 = mem.read_u32(r4 + 4).unwrap_or(0);
    cpu.set_t(w1 == 0);
    if w1 != 0 {
        cpu.r[0] = 1;
        return;
    }
    let w2 = mem.read_u32(r4 + 8).unwrap_or(0);
    cpu.set_t(w2 == 0);
    if w2 != 0 {
        cpu.r[0] = 1;
        return;
    }
    cpu.r[0] = 0; // bra delay slot: mov #0,r0
}
