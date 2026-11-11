//! 0x8c23a4e0 — store_by_index.
//!
//! Stores `r5` at `[0x8c2343a0]+16` for `r4 == 0`, at `+20` for `r4 == 1`, and
//! returns 0; any other index returns `0xeeee`.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[0] = cpu.r[4];
    cpu.r[6] = mem.read_u32(0x8c23a520).unwrap_or(0);
    let eq0 = cpu.r[0] == 0;
    cpu.set_t(eq0);
    if eq0 {
        let _ = mem.write_u32(cpu.r[6] + 16, cpu.r[5]);
        cpu.r[0] = 0;
        return;
    }
    let eq1 = cpu.r[0] == 1;
    cpu.set_t(eq1);
    if eq1 {
        let _ = mem.write_u32(cpu.r[6] + 20, cpu.r[5]);
        cpu.r[0] = 0;
        return;
    }
    cpu.r[0] = mem.read_u32(0x8c23a524).unwrap_or(0);
}
