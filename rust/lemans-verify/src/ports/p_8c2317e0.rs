//! 0x8c2317e0 — r0=0x808, r1=0x8c2c6600; writes r4,r6,r5,r7 at r0+r1 with r0 += 8,-4,+8; rts delay r0=0.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[0] = 0x808;
    cpu.r[1] = 0x8c2c6600;
    let _ = mem.write_u32(cpu.r[0] + cpu.r[1], cpu.r[4]);
    cpu.r[0] = cpu.r[0].wrapping_add(8);
    let _ = mem.write_u32(cpu.r[0] + cpu.r[1], cpu.r[6]);
    cpu.r[0] = cpu.r[0].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[0] + cpu.r[1], cpu.r[5]);
    cpu.r[0] = cpu.r[0].wrapping_add(8);
    let _ = mem.write_u32(cpu.r[0] + cpu.r[1], cpu.r[7]);
    cpu.r[0] = 0; // rts delay
}
