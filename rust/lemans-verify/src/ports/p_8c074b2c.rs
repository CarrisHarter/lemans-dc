//! 0x8c074b2c — table dispatch: v = *(r4 + (0xe8|0xf8 if *(r4+8)==3) + r5*4);
//! if v != 5 return *(*(r4+16+v*4)+12) else 0.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[3] = 0xe8;
    cpu.r[5] = cpu.r[5] << 2; // shll2 r5
    cpu.r[1] = cpu.r[4].wrapping_add(cpu.r[3]);
    cpu.r[0] = cpu.r[5];
    cpu.r[1] = mem.read_u32(cpu.r[0].wrapping_add(cpu.r[1])).unwrap_or(0);
    cpu.r[0] = mem.read_u32(cpu.r[4] + 8).unwrap_or(0);
    cpu.set_t(cpu.r[0] == 3); // cmp/eq #3,r0
    if cpu.r[0] == 3 {
        cpu.r[3] = 0xf8;
        cpu.r[1] = cpu.r[4].wrapping_add(cpu.r[3]);
        cpu.r[0] = cpu.r[5];
        cpu.r[1] = mem.read_u32(cpu.r[0].wrapping_add(cpu.r[1])).unwrap_or(0);
    }
    cpu.r[0] = cpu.r[1];
    cpu.set_t(cpu.r[0] == 5); // cmp/eq #5,r0
    if cpu.r[0] != 5 {
        cpu.r[1] = cpu.r[1] << 2; // shll2 r1
        cpu.r[0] = cpu.r[1];
        cpu.r[4] = cpu.r[4].wrapping_add(16);
        cpu.r[0] = mem.read_u32(cpu.r[0].wrapping_add(cpu.r[4])).unwrap_or(0);
        cpu.r[0] = mem.read_u32(cpu.r[0] + 12).unwrap_or(0); // rts delay slot
    } else {
        cpu.r[0] = 0; // rts delay slot at 0x8c074b5c
    }
}
