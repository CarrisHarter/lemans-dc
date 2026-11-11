//! 0x8c183198 — compare_tables_at_index.
//!
//! If `[r4+28]` is zero returns 0; else compares the `r5`-th word of the two
//! tables at `[r4+12]` and `[r4+16]` and returns 1 when equal. The result is
//! built with `subc r4,r4` then `add #1`.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[0] = mem.read_u32(cpu.r[4] + 28).unwrap_or(0);
    cpu.set_t(cpu.r[0] == 0);
    if cpu.r[0] == 0 {
        cpu.r[4] = 0; // bt
    } else {
        cpu.r[3] = mem.read_u32(cpu.r[4] + 12).unwrap_or(0);
        cpu.r[5] = cpu.r[5] << 2;
        cpu.r[0] = cpu.r[5];
        cpu.r[1] = mem.read_u32(cpu.r[4] + 16).unwrap_or(0);
        cpu.r[3] = mem.read_u32(cpu.r[0].wrapping_add(cpu.r[3])).unwrap_or(0);
        cpu.r[0] = mem.read_u32(cpu.r[0].wrapping_add(cpu.r[1])).unwrap_or(0);
        let t = cpu.r[0] == cpu.r[3];
        cpu.set_t(t);
        // subc r4,r4: r4 = r4 - r4 - T
        cpu.r[4] = 0u32.wrapping_sub(cpu.t() as u32);
        cpu.r[4] = cpu.r[4].wrapping_add(1); // delay slot of `bra`
    }
    cpu.r[0] = cpu.r[4]; // delay slot of `rts`
}
