//! 0x8c182c34 — *r4 = *r5; r0 = r4; r1 = *r6; rts delay *(r4+4) = r1.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[1] = mem.read_u32(cpu.r[5]).unwrap_or(0);
    cpu.r[0] = cpu.r[4];
    let _ = mem.write_u32(cpu.r[4], cpu.r[1]);
    cpu.r[1] = mem.read_u32(cpu.r[6]).unwrap_or(0);
    let _ = mem.write_u32(cpu.r[4] + 4, cpu.r[1]); // rts delay slot
}
