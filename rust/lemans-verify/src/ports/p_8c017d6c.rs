//! 0x8c017d6c — stores the two literal-pool pointers at [r4+12] and [r4+16],
//! returns 1.
//!
//! `mov.l 0x8c017d80,r0` / `mov.l 0x8c017d84,r0` load the pool values
//! (0x8c1f2e88) into r0; only the final r0 survives — the `rts` delay slot
//! `mov #1,r0` overwrites it. r14/r15 are pushed and popped (net zero).

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let base = cpu.r[4];
    let v1 = mem.read_u32(0x8c017d80).unwrap_or(0);
    let _ = mem.write_u32(base + 12, v1);
    let v2 = mem.read_u32(0x8c017d84).unwrap_or(0);
    let _ = mem.write_u32(base + 16, v2);
    cpu.r[0] = 1; // delay slot of `rts`
}
