//! 0x8c111c30 — 3-element sort network over pointer slots at r4/r5/r6.
//!
//! Comparator (shared shape, signed cmp/gt with movt in the bra delay slot):
//! equal u32 at +24 → compare u32 at +0, else compare u32 at +24.
//!
//! Hidden side effects the C does not show:
//! - r0/r1/r3/r7 always clobbered; r6 is reloaded (`mov.l @r4,r6`) on the deep
//!   path only.
//! - stores happen through the slots in instruction order: optional
//!   (*r4,*r5) swap, then *r6, then either (*r5=r6 + rts-delay *r4=r0) or
//!   (*r5=r0).
//! - the final `rts` at 0x8c111c96 has delay slot `mov.l r0,@r4` — a store on
//!   the deep path; the `rts` at 0x8c111c9c has a nop delay slot.
//! - T ends as the last `tst` result: 1 when a `bt` exit was taken, 0 on the
//!   deep `rts` path.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let r4 = cpu.r[4];
    let r5 = cpu.r[5];
    let r6 = cpu.r[6];
    let mut r3 = mem.read_u32(r4).unwrap(); // mov.l @r4,r3
    let mut r7 = mem.read_u32(r5).unwrap(); // mov.l @r5,r7
    let mut r0 = mem.read_u32(r3.wrapping_add(24)).unwrap(); // mov.l @(24,r3),r0
    let mut r1 = mem.read_u32(r7.wrapping_add(24)).unwrap(); // mov.l @(24,r7),r1
    cpu.set_t(r0 == r1); // cmp/eq r0,r1
    if r0 == r1 {
        r1 = mem.read_u32(r7).unwrap(); // mov.l @r7,r1
        r0 = mem.read_u32(r3).unwrap(); // mov.l @r3,r0
        cpu.set_t((r0 as i32) > (r1 as i32)); // cmp/gt r1,r0
        r0 = cpu.t() as u32; // movt r0 (delay slot of bra 0x8c111c4a)
    } else {
        cpu.set_t((r1 as i32) > (r0 as i32)); // cmp/gt r0,r1
        r0 = cpu.t() as u32; // movt r0
    }
    r0 &= 0xFF; // extu.b r0,r0
    cpu.set_t(r0 & r0 == 0); // tst r0,r0
    if r0 != 0 {
        // bt 0x8c111c54 not taken: swap
        let _ = mem.write_u32(r4, r7); // mov.l r7,@r4
        let _ = mem.write_u32(r5, r3); // mov.l r3,@r5
    }
    r7 = mem.read_u32(r5).unwrap(); // mov.l @r5,r7
    r0 = mem.read_u32(r6).unwrap(); // mov.l @r6,r0
    r1 = mem.read_u32(r7.wrapping_add(24)).unwrap(); // mov.l @(24,r7),r1
    r3 = mem.read_u32(r0.wrapping_add(24)).unwrap(); // mov.l @(24,r0),r3
    cpu.set_t(r1 == r3); // cmp/eq r1,r3
    if r1 == r3 {
        r3 = mem.read_u32(r0).unwrap(); // mov.l @r0,r3
        r1 = mem.read_u32(r7).unwrap(); // mov.l @r7,r1
        cpu.set_t((r1 as i32) > (r3 as i32)); // cmp/gt r3,r1
        r1 = cpu.t() as u32; // movt r1 (delay slot of bra 0x8c111c6e)
    } else {
        cpu.set_t((r3 as i32) > (r1 as i32)); // cmp/gt r1,r3
        r1 = cpu.t() as u32; // movt r1
    }
    r1 &= 0xFF; // extu.b r1,r1
    cpu.set_t(r1 & r1 == 0); // tst r1,r1
    if r1 != 0 {
        // bt 0x8c111c9c not taken: deep path
        let _ = mem.write_u32(r6, r7); // mov.l r7,@r6
        let r6 = mem.read_u32(r4).unwrap(); // mov.l @r4,r6
        r3 = mem.read_u32(r0.wrapping_add(24)).unwrap(); // mov.l @(24,r0),r3
        r1 = mem.read_u32(r6.wrapping_add(24)).unwrap(); // mov.l @(24,r6),r1
        cpu.set_t(r1 == r3); // cmp/eq r1,r3
        if r1 == r3 {
            r3 = mem.read_u32(r0).unwrap(); // mov.l @r0,r3
            r1 = mem.read_u32(r6).unwrap(); // mov.l @r6,r1
            cpu.set_t((r1 as i32) > (r3 as i32)); // cmp/gt r3,r1
            r1 = cpu.t() as u32; // movt r1 (delay slot of bra 0x8c111c8e)
        } else {
            cpu.set_t((r3 as i32) > (r1 as i32)); // cmp/gt r1,r3
            r1 = cpu.t() as u32; // movt r1
        }
        r1 &= 0xFF; // extu.b r1,r1
        cpu.set_t(r1 & r1 == 0); // tst r1,r1
        if r1 != 0 {
            // bt 0x8c111c9a not taken: rts at 0x8c111c96, delay slot stores
            let _ = mem.write_u32(r5, r6); // mov.l r6,@r5
            let _ = mem.write_u32(r4, r0); // delay slot of rts: mov.l r0,@r4
        } else {
            let _ = mem.write_u32(r5, r0); // 0x8c111c9a: mov.l r0,@r5
        }
        cpu.r[6] = r6;
    }
    cpu.r[0] = r0;
    cpu.r[1] = r1;
    cpu.r[3] = r3;
    cpu.r[7] = r7;
}
