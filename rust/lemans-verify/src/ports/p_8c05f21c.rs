//! 0x8c05f21c — copy 13 dwords (52 bytes) from [r5] to r4+12.
//!
//! Original: `add #12,r4` then interleaved `mov.l @(disp,r5),rX` /
//! `mov.l rX,@(disp,r4)` pairs for disp 0..48, ending in `rts` with delay
//! slot `mov.l r0,@(48,r4)`. Leftovers the C does not show: r4 itself is
//! advanced by 12, r1 = [r5+40] and r0 = [r5+48] survive.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let src = cpu.r[5];
    let dst = cpu.r[4].wrapping_add(12); // add #12,r4
    let mut r1 = 0u32;
    let mut r0 = 0u32;
    // pairs for disp 0..44 (k = 0..5); final r1 = [r5+40], then disp 48
    for k in 0..6u32 {
        r1 = mem.read_u32(src.wrapping_add(k * 8)).unwrap();
        r0 = mem.read_u32(src.wrapping_add(k * 8 + 4)).unwrap();
        let _ = mem.write_u32(dst.wrapping_add(k * 8), r1);
        let _ = mem.write_u32(dst.wrapping_add(k * 8 + 4), r0);
    }
    r0 = mem.read_u32(src.wrapping_add(48)).unwrap(); // mov.l @(48,r5),r0
    cpu.r[4] = dst;
    cpu.r[1] = r1;
    cpu.r[0] = r0;
    let _ = mem.write_u32(dst.wrapping_add(48), r0); // rts delay slot
}
