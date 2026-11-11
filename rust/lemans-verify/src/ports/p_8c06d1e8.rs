//! 0x8c06d1e8 — for each set flag bit in r4 (0x07000000, 0x01000000,
//! 0x02000000, 0x04000000) store fr4 at the corresponding global pointer.
//!
//! The globals (0x8c2400f0..0x8c240120) lie outside the mapped image, so only
//! the all-clear path is pinnable. Side effects: r1 = last mask, r0 = pool or
//! 0x04000000, r4 &= 0x04000000 (the last and is in-place), T = (r4 == 0).

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    for (mask, pool) in [
        (0x0700_0000u32, 0x8c24_00f0u32),
        (0x0100_0000, 0x8c24_0100),
        (0x0200_0000, 0x8c24_0110),
    ] {
        cpu.r[1] = mask;
        cpu.r[0] = cpu.r[4] & mask;
        cpu.set_t(cpu.r[0] == 0); // tst r0,r0
        if cpu.r[0] != 0 {
            cpu.r[0] = pool;
            let _ = mem.write_u32(cpu.r[0], cpu.fr[4].to_bits()); // fmov fr4,@r0
        }
    }
    cpu.r[0] = 0x0400_0000;
    cpu.r[4] &= cpu.r[0]; // and r0,r4 (in-place)
    cpu.set_t(cpu.r[4] == 0); // tst r4,r4
    if cpu.r[4] != 0 {
        cpu.r[0] = 0x8c24_0120;
        let _ = mem.write_u32(cpu.r[0], cpu.fr[4].to_bits());
    }
}
