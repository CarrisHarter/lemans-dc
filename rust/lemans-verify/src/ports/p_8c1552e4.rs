//! 0x8c1552e4 — write a value at `base+4` and through a pointer at `base+0x20`.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let base = cpu.r[4];
    let value = cpu.r[5];
    let _ = mem.write_u32(base + 4, value);
    let inner = mem.read_u32(base + 0x20).unwrap_or(0);
    cpu.r[0] = inner; // the original keeps the pointer in r0
    let _ = mem.write_u32(inner + 8, value);
}
