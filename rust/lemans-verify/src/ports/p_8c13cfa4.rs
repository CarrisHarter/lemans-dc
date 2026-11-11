//! 0x8c13cfa4 — r4 += 116; r5 *= 8; *r6 = *(*(r4)+r5); rts delay *(r6+4) = *(*(r4)+r5+4).

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[4] = cpu.r[4].wrapping_add(116);
    cpu.r[5] = cpu.r[5] << 2; // shll2
    cpu.r[5] = cpu.r[5] << 1; // shll  => r5*8
    cpu.r[1] = mem.read_u32(cpu.r[4]).unwrap_or(0);
    cpu.r[0] = cpu.r[5];
    cpu.r[0] = mem.read_u32(cpu.r[0].wrapping_add(cpu.r[1])).unwrap_or(0);
    let _ = mem.write_u32(cpu.r[6], cpu.r[0]);
    cpu.r[0] = mem.read_u32(cpu.r[4]).unwrap_or(0);
    cpu.r[0] = cpu.r[0].wrapping_add(cpu.r[5]);
    cpu.r[0] = mem.read_u32(cpu.r[0] + 4).unwrap_or(0);
    let _ = mem.write_u32(cpu.r[6] + 4, cpu.r[0]); // rts delay slot
}
