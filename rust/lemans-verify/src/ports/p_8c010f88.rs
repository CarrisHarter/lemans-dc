//! 0x8c010f88 — memset: fill n bytes at r4 with the low byte of r5.
//!
//! Side effects the C hides: r3 is zeroed (loop scratch seed), r7 ends as
//! the loop counter (n, or 0 when n==0), r0 ends as r4 because the `rts`
//! delay slot reloads it, and T is always set on exit.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let dst = cpu.r[4];
    let val = cpu.r[5] as u8;
    let n = cpu.r[6];
    cpu.r[3] = 0; // mov r7,r3 after r7 = 0
    cpu.r[7] = 0;
    if n > 0 {
        for i in 0..n {
            let _ = mem.write_u8(dst.wrapping_add(i), val);
        }
        cpu.r[7] = n; // counter after the last `cmp/hs r6,r7`
    }
    cpu.set_t(true); // 0 >=u n (n==0 path) or r7 >=u n (loop exit)
    cpu.r[0] = dst;  // delay slot of `rts`
}
