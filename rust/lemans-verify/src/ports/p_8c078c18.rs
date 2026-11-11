//! 0x8c078c18 — r4 += 4; store new r4 at *r15 (rts delay slot).

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[0] = mem.read_u32(cpu.r[15]).unwrap_or(0); // mov.l @r15,r0
    cpu.r[4] = cpu.r[4].wrapping_add(4);
    let _ = mem.write_u32(cpu.r[0], cpu.r[4]); // rts delay slot
}
