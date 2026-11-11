//! 0x8c0b838c — init_three_vtables_2.
//!
//! Stores three pool words at `[r4]` (the last wins), sets the byte at
//! `r4+4` to 1, and returns `r4` from the delay slot.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let base = cpu.r[4];
    for pool in [0x8C0B_8360u32, 0x8C0B_8364, 0x8C0B_83A8] {
        cpu.r[0] = mem.read_u32(pool).unwrap_or(0);
        let _ = mem.write_u32(base, cpu.r[0]);
    }
    cpu.r[0] = 1;
    let _ = mem.write_u8(base + 4, 1);
    cpu.r[0] = base; // delay slot of `rts`
}
