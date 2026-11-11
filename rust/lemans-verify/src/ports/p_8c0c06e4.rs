//! 0x8c0c06e4 — descend a binary tree from `*(r4+4)` comparing `*r5` with
//! each node's key at offset 12; stores the insertion pointer to `*r15`.
//!
//! ```text
//! mov.l @(4,r4),r6 ; tst r6,r6 ; bt.s 0x8c0c0700 ; delay: mov #0,r3
//! loop 0x8c0c06ec:
//!   mov.l @r5,r1 ; mov.l @(12,r6),r0 ; cmp/gt r1,r0 (T = r0 > r1)
//!   bf 0x8c0c06f8 ; bra 0x8c0c06fc ; delay: mov.l @r6,r6   ; left child
//!   0x8c0c06f8: mov r6,r3 ; mov.l @(4,r6),r6                ; right child
//!   0x8c0c06fc: tst r6,r6 ; bf 0x8c0c06ec
//! 0x8c0c0700: tst r3,r3 ; bt 0x8c0c070c
//!   mov.l @(12,r3),r1 ; mov.l @r5,r0 ; cmp/gt r1,r0 ; bf 0x8c0c0714
//! 0x8c0c070c: mov.l @r15,r0 ; add #4,r4 ; rts ; delay: mov.l r4,@r0
//! 0x8c0c0714: mov.l @r15,r0 ; rts ; delay: mov.l r3,@r0   ; r4 NOT incremented
//! ```
//! Hidden side effects: r4 ends +4 only on the `r4`-store paths (the
//! `r3`-store path leaves it), r0 = *r15, r1 = last key load, r6 = 0, and
//! T ends 1 on the r3==0 / left paths, or at the final cmp/gt otherwise.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[6] = mem.read_u32(cpu.r[4].wrapping_add(4)).unwrap();
    cpu.set_t(cpu.r[6] == 0); // tst r6,r6
    if cpu.r[6] == 0 {
        cpu.r[3] = 0; // bt.s delay slot: mov #0,r3
        cpu.set_t(cpu.r[3] == 0); // 0x8c0c0700: tst r3,r3 → bt taken
        cpu.r[0] = mem.read_u32(cpu.r[15]).unwrap();
        cpu.r[4] = cpu.r[4].wrapping_add(4);
        let _ = mem.write_u32(cpu.r[0], cpu.r[4]); // rts delay: mov.l r4,@r0
        return;
    }
    let mut r3: u32 = 0;
    let mut r6 = cpu.r[6];
    loop {
        // 0x8c0c06ec
        let r1 = mem.read_u32(cpu.r[5]).unwrap();
        let r0 = mem.read_u32(r6.wrapping_add(12)).unwrap();
        cpu.r[1] = r1;
        // cmp/gt r1,r0 → T = r0 > r1
        if (r0 as i32) > (r1 as i32) {
            r6 = mem.read_u32(r6).unwrap(); // bra delay: mov.l @r6,r6 (left)
        } else {
            r3 = r6; // 0x8c0c06f8: mov r6,r3
            r6 = mem.read_u32(r6.wrapping_add(4)).unwrap(); // right child
        }
        cpu.r[6] = r6;
        cpu.set_t(r6 == 0); // 0x8c0c06fc: tst r6,r6
        if r6 == 0 {
            break; // bf not taken → 0x8c0c0700
        }
    }
    // 0x8c0c0700
    cpu.r[3] = r3;
    cpu.set_t(r3 == 0); // tst r3,r3
    if r3 == 0 {
        // bt 0x8c0c070c
        cpu.r[0] = mem.read_u32(cpu.r[15]).unwrap();
        cpu.r[4] = cpu.r[4].wrapping_add(4);
        let _ = mem.write_u32(cpu.r[0], cpu.r[4]); // rts delay: mov.l r4,@r0
        return;
    }
    // 0x8c0c0704
    let r1 = mem.read_u32(r3.wrapping_add(12)).unwrap();
    let r0 = mem.read_u32(cpu.r[5]).unwrap();
    cpu.r[1] = r1;
    let t = (r0 as i32) > (r1 as i32); // cmp/gt r1,r0 at 0x8c0c0708
    cpu.set_t(t);
    cpu.r[0] = mem.read_u32(cpu.r[15]).unwrap();
    if t {
        // fall through to 0x8c0c070c
        cpu.r[4] = cpu.r[4].wrapping_add(4);
        let _ = mem.write_u32(cpu.r[0], cpu.r[4]);
    } else {
        // bf 0x8c0c0714: rts ; delay: mov.l r3,@r0 — r4 NOT incremented
        let _ = mem.write_u32(cpu.r[0], r3);
    }
}
