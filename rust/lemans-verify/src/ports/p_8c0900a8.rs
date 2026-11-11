//! 0x8c0900a8 — zero 9 words at r4 (offsets 0..32); r1 = 0, r0 = r4 (rts delay).

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[1] = 0;
    for i in 0..9u32 {
        let _ = mem.write_u32(cpu.r[4] + i * 4, cpu.r[1]);
    }
    cpu.r[0] = cpu.r[4]; // rts delay slot region also stores, then mov r4,r0
}
