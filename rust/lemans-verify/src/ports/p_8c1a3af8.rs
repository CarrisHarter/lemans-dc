//! 0x8c1a3af8 — r5*=8; r1 = *(*[r4+12]+4+r5) - *(*[r4+12]+r5); r0 = (r1 >= 0) ? r1 : 0 (rts delay).

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[3] = mem.read_u32(cpu.r[4] + 12).unwrap_or(0);
    cpu.r[5] = (cpu.r[5] << 2) << 1;
    cpu.r[1] = cpu.r[3].wrapping_add(4);
    cpu.r[0] = cpu.r[5];
    cpu.r[1] = mem.read_u32(cpu.r[0].wrapping_add(cpu.r[1])).unwrap_or(0);
    cpu.r[0] = cpu.r[5];
    cpu.r[0] = mem.read_u32(cpu.r[0].wrapping_add(cpu.r[3])).unwrap_or(0);
    cpu.r[1] = cpu.r[1].wrapping_sub(cpu.r[0]); // sub r0,r1
    cpu.set_t((cpu.r[1] as i32) >= 0); // cmp/pz r1
    if (cpu.r[1] as i32) < 0 {
        cpu.r[1] = 0;
    }
    cpu.r[0] = cpu.r[1]; // rts delay slot
}
