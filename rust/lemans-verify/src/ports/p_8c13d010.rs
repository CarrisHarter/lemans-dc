//! 0x8c13d010 — r4 += 116; r5 *= 8; r0 = *(*r4 + r5); r0 = 0 if -1 else 2.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[4] = cpu.r[4].wrapping_add(116);
    cpu.r[5] = (cpu.r[5] << 2) << 1;
    cpu.r[1] = mem.read_u32(cpu.r[4]).unwrap_or(0);
    cpu.r[0] = mem.read_u32(cpu.r[5].wrapping_add(cpu.r[1])).unwrap_or(0);
    cpu.set_t(cpu.r[0] == (-1i32) as u32); // cmp/eq #-1,r0
    if cpu.r[0] == (-1i32) as u32 {
        cpu.r[0] = 0; // rts delay slot
        return;
    }
    cpu.r[0] = 2;
}
