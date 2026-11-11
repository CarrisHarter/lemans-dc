//! 0x8c017efc — compares three u32s at r4 with three at r5; returns 0 when all
//! equal, 1 otherwise.
//!
//! Side effects the C hides: r1 keeps the last loaded left-hand word
//! (mem32[r4], [r4+4] or [r4+8] depending on where it bails), and T is the
//! result of the last cmp/eq executed.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let r4 = cpu.r[4];
    let r5 = cpu.r[5];
    let a0 = mem.read_u32(r4).unwrap_or(0);
    let b0 = mem.read_u32(r5).unwrap_or(0);
    cpu.r[1] = a0;
    cpu.set_t(a0 == b0);
    if a0 != b0 {
        cpu.r[0] = 1;
        return;
    }
    let a1 = mem.read_u32(r4 + 4).unwrap_or(0);
    let b1 = mem.read_u32(r5 + 4).unwrap_or(0);
    cpu.r[1] = a1;
    cpu.set_t(a1 == b1);
    if a1 != b1 {
        cpu.r[0] = 1;
        return;
    }
    let a2 = mem.read_u32(r4 + 8).unwrap_or(0);
    let b2 = mem.read_u32(r5 + 8).unwrap_or(0);
    cpu.r[1] = a2;
    cpu.set_t(a2 == b2);
    if a2 != b2 {
        cpu.r[0] = 1;
        return;
    }
    cpu.r[0] = 0; // bra delay slot: mov #0,r0
}
