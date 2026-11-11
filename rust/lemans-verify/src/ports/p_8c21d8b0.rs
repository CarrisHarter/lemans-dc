//! 0x8c21d8b0 — set_global_or_minus_one.
//!
//! Returns -1 when `r4` is null; otherwise stores `r4` at `0x8c23d450` and
//! returns 0.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let t = cpu.r[4] == 0;
    cpu.set_t(t);
    if t {
        cpu.r[0] = u32::MAX; // delay slot of `rts`
        return;
    }
    cpu.r[2] = mem.read_u32(0x8c21da98).unwrap_or(0);
    cpu.r[0] = 0;
    let _ = mem.write_u32(cpu.r[2], cpu.r[4]);
}
