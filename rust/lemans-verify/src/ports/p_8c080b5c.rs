//! 0x8c080b5c — init struct at r4: [0] = *0x8c1f8d90 (double deref), [4..24] = 0.
//!
//! Original side effects: r15 net unchanged, r1 = 0, r3 = r4+4 then last zero
//! slot, r0 = r4, r4 = pool value.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let s = cpu.r[4];
    cpu.r[0] = s; // via r15 round-trip
    cpu.r[4] = mem.read_u32(0x8c1f8d90).unwrap_or(0); // mov.l @r3,r4 (double deref)
    cpu.r[1] = 0;
    cpu.r[3] = cpu.r[0];
    let _ = mem.write_u32(cpu.r[0], cpu.r[4]);
    cpu.r[3] = cpu.r[3].wrapping_add(4);
    cpu.r[3] = mem.read_u32(0).unwrap_or(0); // [r15+4] reload — synthetic stack, use stored r0+4
    cpu.r[3] = cpu.r[0].wrapping_add(4);
    for off in [4u32, 8, 12, 16, 20, 24] {
        let _ = mem.write_u32(cpu.r[0] + off, cpu.r[1]);
    }
}
