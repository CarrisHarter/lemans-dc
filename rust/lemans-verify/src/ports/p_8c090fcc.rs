//! 0x8c090fcc — fill loop over the r4+0xc0 region, then a fixed alternating tail.
//!
//! Loop: r1 = 46 each iteration; while 46 > r7 (r7 += 2, r0 += 8 per iteration)
//! store r5 at [r0+r6] and [r3+4] (r6 = r4+0xc0, r3 = r6+r0). Tail: r0 = 68,
//! then r1(46)/r5(0) alternately at offsets 8,0,20,12,32,24,44,36,56,48,68,
//! rts delay at 60.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[7] = 0;
    let mut r0 = 0u32;
    cpu.r[6] = cpu.r[4].wrapping_add(0xc0);
    cpu.r[5] = cpu.r[7];
    loop {
        cpu.r[1] = 46; // mov #46,r1 inside the loop
        cpu.r[3] = cpu.r[6];
        let _ = mem.write_u32(r0.wrapping_add(cpu.r[6]), cpu.r[5]);
        cpu.r[3] = cpu.r[3].wrapping_add(r0);
        cpu.r[7] = cpu.r[7].wrapping_add(2);
        let _ = mem.write_u32(cpu.r[3] + 4, cpu.r[5]);
        cpu.set_t(46 > (cpu.r[7] as i32)); // cmp/gt r7,r1: T=(r1>r7)
        r0 = r0.wrapping_add(8); // bt.s delay slot
        if 46 > (cpu.r[7] as i32) {
            continue;
        }
        break;
    }
    cpu.r[0] = 68;
    let r1 = cpu.r[1];
    let r5 = cpu.r[5];
    for (off, v) in [(8u32, r1), (0, r5), (20, r1), (12, r5), (32, r1), (24, r5), (44, r1), (36, r5), (56, r1), (48, r5), (68, r1)] {
        let _ = mem.write_u32(cpu.r[4] + off, v);
    }
    let _ = mem.write_u32(cpu.r[4] + 60, r5); // rts delay slot
}
