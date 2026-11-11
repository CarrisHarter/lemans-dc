//! 0x8c06d244 — same shape as 0x8c06d1e8 with globals 0x8c2400f8..0x8c240128.
//!
//! Only the all-clear path is pinnable (globals outside the image).
//! Side effects: r1 = last mask, r0 = pool or 0x04000000, r4 &= 0x04000000,
//! T = (r4 == 0).

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    for (mask, pool) in [
        (0x0700_0000u32, 0x8c24_00f8u32),
        (0x0100_0000, 0x8c24_0108),
        (0x0200_0000, 0x8c24_0118),
    ] {
        cpu.r[1] = mask;
        cpu.r[0] = cpu.r[4] & mask;
        cpu.set_t(cpu.r[0] == 0);
        if cpu.r[0] != 0 {
            cpu.r[0] = pool;
            let _ = mem.write_u32(cpu.r[0], cpu.fr[4].to_bits());
        }
    }
    cpu.r[0] = 0x0400_0000;
    cpu.r[4] &= cpu.r[0];
    cpu.set_t(cpu.r[4] == 0);
    if cpu.r[4] != 0 {
        cpu.r[0] = 0x8c24_0128;
        let _ = mem.write_u32(cpu.r[0], cpu.fr[4].to_bits());
    }
}
