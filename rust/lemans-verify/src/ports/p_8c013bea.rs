//! 0x8c013bea — if *r4 == 1, zero the byte at *(r4+4).
//!
//! Original side effects: r0 = *r4, T = (r0 == 1); on the taken branch
//! r2 = *(r4+4), r3 = 0, byte store of 0 at r2.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[0] = mem.read_u32(cpu.r[4]).unwrap_or(0); // mov.l @r4,r0
    cpu.set_t(cpu.r[0] == 1); // cmp/eq #1,r0
    if cpu.r[0] == 1 {
        cpu.r[2] = mem.read_u32(cpu.r[4] + 4).unwrap_or(0); // mov.l @(4,r4),r2
        cpu.r[3] = 0;
        let _ = mem.write_u8(cpu.r[2], 0); // mov.b r3,@r2
    }
}
