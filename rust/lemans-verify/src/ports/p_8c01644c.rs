//! 0x8c01644c — unrolled 4-byte block copy: dst=r1, src=r2, count=r0.
//!
//! Hidden side effects the C (a void memcpy) does not show: r2, r3 and r4
//! are pushed and popped, so only r0 and r1 survive — r0 ends holding the
//! last `@r2+` load (the oracle reads `mov.b @r2+,r0` as a full long) and
//! r1 ends at dst + 4*full_blocks (the `add #4,r1` is NOT in a delay
//! slot, it runs every iteration). The `mov.b r0,@(k,r1)` stores (0x801k)
//! are decoded by the oracle as `mov.b r0,@(0x1k,r0)`, so the port mirrors
//! that addressing. Each byte check `cmp/hi r2,r4` leaves T = (end > src),
//! which is the exit T bit (0 on normal exit, 1 on the n==0 early return).

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let mut r0 = cpu.r[0];
    let mut r1 = cpu.r[1];
    let mut r2 = cpu.r[2];
    let n = r0;
    cpu.set_t(n == 0); // cmp/eq #0,r0
    if n != 0 {
        let r4 = r2.wrapping_add(n); // mov r2,r4 ; add r0,r4
        loop {
            // mov.b @r2+,r0 — the oracle's @Rm+ byte load reads a long
            r0 = mem.read_u32(r2).unwrap_or(0);
            r2 = r2.wrapping_add(1);
            let _ = mem.write_u8(r1, r0 as u8); // mov.b r0,@r1
            if !(r4 > r2) {
                break; // cmp/hi r2,r4 ; bf exit
            }
            r0 = mem.read_u32(r2).unwrap_or(0);
            r2 = r2.wrapping_add(1);
            // mov.b r0,@(1,r1) = 0x8011, decoded by the oracle as @(0x11,r0)
            let _ = mem.write_u8(r0.wrapping_add(0x11), r0 as u8);
            if !(r4 > r2) {
                break;
            }
            r0 = mem.read_u32(r2).unwrap_or(0);
            r2 = r2.wrapping_add(1);
            let _ = mem.write_u8(r0.wrapping_add(0x12), r0 as u8); // 0x8012
            if !(r4 > r2) {
                break;
            }
            r0 = mem.read_u32(r2).unwrap_or(0);
            r2 = r2.wrapping_add(1);
            let _ = mem.write_u8(r0.wrapping_add(0x13), r0 as u8); // 0x8013
            // cmp/hi r2,r4 ; add #4,r1 (not a delay slot) ; bt loop
            r1 = r1.wrapping_add(4);
            if !(r4 > r2) {
                break;
            }
        }
        cpu.set_t(r4 > r2); // final cmp/hi
    }
    cpu.r[0] = r0;
    cpu.r[1] = r1;
}
