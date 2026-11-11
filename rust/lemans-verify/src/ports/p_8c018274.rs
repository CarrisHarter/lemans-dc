//! 0x8c018274 — `*(u32 *)(*(u32 *)0x8c1f2f98 + 8) = 0;`
//!
//! Side effects the C hides: r2 = 0x8c1f2f98 (the pool constant) and r3 = the
//! dereferenced pointer both survive; r1 = 0.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let p = mem.read_u32(0x8c018280).unwrap_or(0);
    cpu.r[2] = p;
    let inner = mem.read_u32(p).unwrap_or(0);
    cpu.r[3] = inner;
    cpu.r[1] = 0;
    let _ = mem.write_u32(inner + 8, 0);
}
