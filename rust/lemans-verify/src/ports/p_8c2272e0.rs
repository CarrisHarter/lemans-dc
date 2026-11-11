//! 0x8c2272e0 — r6=r4+4; r5 = (r5&3)<<27 | (*r6 & 0xe7ffffff); *r6=r5; rts delay r0=0.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[6] = cpu.r[4].wrapping_add(4);
    cpu.r[1] = 0xe7ffffff;
    cpu.r[2] = 3;
    cpu.r[0] = mem.read_u32(cpu.r[6]).unwrap_or(0);
    cpu.r[5] &= cpu.r[2];
    cpu.r[3] = 27;
    cpu.r[0] &= cpu.r[1];
    cpu.r[5] = cpu.r[5] << 27; // shad 27
    cpu.r[5] |= cpu.r[0];
    let _ = mem.write_u32(cpu.r[6], cpu.r[5]);
    cpu.r[0] = 0; // rts delay
}
