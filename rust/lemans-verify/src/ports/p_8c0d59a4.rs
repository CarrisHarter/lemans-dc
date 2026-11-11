//! 0x8c0d59a4 — per-axis slab test over a 12-float array at r4; returns a
//! bitmask in r0.
//!
//! CFG: three straight-line blocks (k = 0,1,2), base byte offsets 0,4,8.
//! Block k reads a=[p+4k], b=[p+4k+12], c=[p+4k+24], d=[p+4k+36], computes
//! sum = |a|+|b|+|c|+1.0 (fabs/fadd/fldi1), t = d - sum (fsub), then
//! `fcmp/gt fr0(0),fr1(t)` -> T = (t > 0). The `mov #0,r0` sits in the bf.s
//! delay slot of block 0, so it runs on BOTH paths: r0 is always zeroed
//! there. If t > 0: `or #(1<<2k),r0` (block 2 via an early rts + delay
//! slot). Otherwise `fadd fr3,fr2` (fr2 = d+sum) and `fcmp/gt fr2,fr0`
//! -> T = (0 > fr2); if fr2 < 0: `or #(2<<2k),r0`.
//!
//! Final registers: r0 = mask, r1 = p+20, r3 = p+8, r4 = p+44, r5 = p+32,
//! fr0 = 0.0 (fldi0), fr1 = block2's t, fr2 = [p+44] (t>0 path) or
//! [p+44]+sum, fr3 = block2's sum. T = 1 when block2 has t>0, else
//! (block2's fr2 < 0).

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let p = cpu.r[4];
    let ld = |a: u32| f32::from_bits(mem.read_u32(a).unwrap());
    let mut r0 = 0u32; // mov #0,r0: bf.s delay slot of block 0, runs either way
    let mut fr1 = 0.0f32;
    let mut fr2 = 0.0f32;
    let mut fr3 = 0.0f32;
    let mut t_flag = true;
    for k in 0..3u32 {
        let a = p + [0u32, 4, 8][k as usize];
        let d = ld(a + 36);
        let mut sum = ld(a).abs();
        sum += ld(a + 12).abs();
        sum += ld(a + 24).abs();
        sum += 1.0; // fldi1 fr0; fadd fr0,fr3
        let t = d - sum; // fmov fr2,fr1; fsub fr3,fr1
        if t > 0.0 {
            // fcmp/gt fr0,fr1: T = fr1 > fr0
            r0 |= 1 << (2 * k);
            if k == 2 {
                fr1 = t;
                fr2 = d; // no fadd on this path
                fr3 = sum;
                t_flag = true;
            }
        } else {
            let f2 = d + sum; // fadd fr3,fr2
            if 0.0 > f2 {
                // fcmp/gt fr2,fr0: T = fr0 > fr2
                r0 |= 2 << (2 * k);
            }
            if k == 2 {
                fr1 = t;
                fr2 = f2;
                fr3 = sum;
                t_flag = f2 < 0.0;
            }
        }
    }
    cpu.r[0] = r0;
    cpu.r[1] = p + 20;
    cpu.r[3] = p + 8;
    cpu.r[4] = p + 44;
    cpu.r[5] = p + 32;
    cpu.fr[0] = 0.0; // fldi0 fr0
    cpu.fr[1] = fr1;
    cpu.fr[2] = fr2;
    cpu.fr[3] = fr3;
    cpu.set_t(t_flag);
}
