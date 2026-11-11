//! 0x8c173a20 — ocbp/ocbi cache-hint loop over a 32-byte-aligned range.
//!
//! r1 = r4 & ~31, r5 = (r4 + r5 - 1) & ~31. ocbp @r1, then while r1 <u r5:
//! ocbp @r5 once, r5 -= 32, and loop { ocbi @r1; ocbi @r1 } with r1 += 32
//! per bt.s/bf.s delay slot. The cache ops have no architectural effect, so
//! only r1 (final = first value >= r5-32 plus the exit delay's +32), r5,
//! r2=0xffffffe0 and T=1 leak. rts delay slot is a nop.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, _mem: &mut Memory) {
    let mask = 0xFFFF_FFE0u32;
    let mut r1 = cpu.r[4] & mask;
    let mut r5 = cpu.r[4].wrapping_add(cpu.r[5]).wrapping_sub(1) & mask;

    // 8c173a2c: ocbp @r1 (no-op); cmp/hs r5,r1 ; bt.s exit (delay add #32,r1)
    let mut t = r1 >= r5;
    cpu.set_t(t);
    r1 = r1.wrapping_add(32);
    if !t {
        // 8c173a34: ocbp @r5 (no-op); r5 -= 32
        r5 = r5.wrapping_sub(32);
        loop {
            // 8c173a38: ocbi @r1 (no-op); cmp/hs r5,r1 ; bt.s exit (delay +32)
            t = r1 >= r5;
            cpu.set_t(t);
            r1 = r1.wrapping_add(32);
            if t {
                break;
            }
            // 8c173a40: ocbi @r1 (no-op); cmp/hs r5,r1 ; bf.s loop (delay +32)
            t = r1 >= r5;
            cpu.set_t(t);
            r1 = r1.wrapping_add(32);
            if !t {
                continue;
            }
            break;
        }
    }
    cpu.r[1] = r1;
    cpu.r[2] = mask;
    cpu.r[5] = r5;
}
