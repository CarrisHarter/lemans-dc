//! 0x8c12b3f8 — linear search for r5 in the array at (*r4)+32; r0 = 1 if found else 0.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[0] = mem.read_u32(cpu.r[4]).unwrap_or(0);
    cpu.r[0] = cpu.r[0].wrapping_add(32);
    cpu.r[1] = mem.read_u32(cpu.r[0] + 8).unwrap_or(0);
    cpu.r[0] = mem.read_u32(cpu.r[0] + 4).unwrap_or(0);
    cpu.r[3] = cpu.r[1];
    cpu.r[0] = cpu.r[0] << 2;
    cpu.r[3] = cpu.r[3].wrapping_add(cpu.r[0]); // bra delay slot
    loop {
        cpu.set_t(cpu.r[3] == cpu.r[1]); // cmp/eq r3,r1 (loop head)
        if cpu.r[3] == cpu.r[1] {
            cpu.r[0] = 0; // rts delay slot
            return;
        }
        cpu.r[0] = mem.read_u32(cpu.r[1]).unwrap_or(0);
        cpu.set_t(cpu.r[0] == cpu.r[5]); // cmp/eq r5,r0
        if cpu.r[0] == cpu.r[5] {
            cpu.r[0] = 1; // rts delay slot
            return;
        }
        cpu.r[1] = cpu.r[1].wrapping_add(4);
    }
}
