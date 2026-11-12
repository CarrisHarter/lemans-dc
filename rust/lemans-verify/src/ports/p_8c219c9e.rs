//! 0x8c219c9e — find lowest set bit of *r4 (vs mask 0x80000000 >> i); r0=r7 (addr+4) if r5==0x80000000, else r0=0 after 32.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let saved_r12 = cpu.r[12];
    cpu.r[7] = cpu.r[4];
    cpu.r[6] = 0x80000000;
    cpu.r[0] = 32;
    cpu.r[12] = mem.read_u32(cpu.r[4]).unwrap_or(0); cpu.r[4] = cpu.r[4].wrapping_add(4);
    cpu.r[7] = cpu.r[7].wrapping_add(4);
    cpu.r[1] = 0;
    loop {
        cpu.r[3] = cpu.r[6];
        if cpu.r[12] & cpu.r[3] != 0 {
            if cpu.r[6] == cpu.r[5] {
                cpu.r[0] = cpu.r[7];
                cpu.set_t(true); // cmp/eq r6,r5 true here
                cpu.r[12] = saved_r12; // rts delay: mov.l @r15+,r12
                return;
            }
            cpu.r[4] = cpu.r[4].wrapping_add(4);
            cpu.r[7] = cpu.r[4];
        }
        cpu.r[1] = cpu.r[1].wrapping_add(1);
        cpu.r[6] >>= 1; // shlr
        if cpu.r[1] >= cpu.r[0] { break; } // cmp/ge r0,r1: T = r1>=r0; bf loops while T=0
    }
    cpu.r[0] = 0; cpu.r[12] = saved_r12; cpu.sr |= 1;
}
