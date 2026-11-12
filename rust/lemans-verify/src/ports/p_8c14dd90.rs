//! 0x8c14dd90 — copy_words_until_limit.
//!
//! Copies words from `r4` to `r6` until `r4 == r5`; a null `r6` skips the
//! store but still advances the destination. `r0` ends as `r6`.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let lo = cpu.r[4];
    let hi = cpu.r[5];
    let mut src = lo;
    let mut dst = cpu.r[6];
    cpu.set_t(src == hi); // cmp/eq r5,r4
    if src != hi {
        loop {
            if dst != 0 {
                let v = mem.read_u32(src).unwrap_or(0);
                let _ = mem.write_u32(dst, v);
                cpu.r[0] = v;
            }
            src = src.wrapping_add(4);
            let done = src == hi;
            cpu.set_t(done); // cmp/eq r5,r4
            dst = dst.wrapping_add(4); // delay slot of `bf.s`
            if done {
                break;
            }
        }
    }
    cpu.r[4] = src;
    cpu.r[6] = dst;
    cpu.r[0] = dst; // delay slot of `rts`
}
