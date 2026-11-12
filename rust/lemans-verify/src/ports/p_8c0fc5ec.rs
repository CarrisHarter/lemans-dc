//! 0x8c0fc5ec — release a reference, then maybe free the owner.
//!
//! `void *FUN_8c0fc5ec(void *p, short owner)`: if `p` is null, return it.
//! If `p == -4`, skip straight to the owner test. Otherwise `q = [p+4]` is a
//! reference-counted block: a zero count at `[q+8]` is bumped to one first, the
//! count is then decremented and stored back; when it reaches zero the pointer
//! itself survives in `[p+4]`, otherwise `[p+4]` is cleared. While `r12 = [p+4]`
//! is non-null the block's payload `[r12+12]` is freed, `0x8c018e14(r12+4, -1)`
//! runs (a signed-16 flag of -1, so it frees nothing), and `[p+4]` is freed.
//! Finally, when the sign-extended low half of the owner argument is positive,
//! `p` itself is freed. Returns `p`.
//!
//! ```text
//! 8c0fc5ec  mov.l r14,@-r15 ; mov.l r13,@-r15 ; mov r5,r14
//! 8c0fc5f2  mov.l r12,@-r15 ; mov r4,r13 ; sts.l pr,@-r15
//! 8c0fc5f8  tst r13,r13 ; bt 8c0fc65e                 ; null exit
//! 8c0fc5fc  mov r13,r0 ; add #4,r0 ; tst r0,r0 ; bt 8c0fc652  ; p == -4
//! 8c0fc604  mov.l @(4,r13),r1 ; mov.l @(8,r1),r0 ; tst r0,r0 ; bf 8c0fc61a
//! 8c0fc60c  mov.l @(8,r1),r0 ; tst r0,r0 ; bt 8c0fc616
//! 8c0fc612  bra 8c0fc618 ; nop                         ; dead: load was zero
//! 8c0fc616  mov #1,r0
//! 8c0fc618  mov.l r0,@(8,r1)
//! 8c0fc61a  mov.l @(8,r1),r0 ; add #-1,r0 ; tst r0,r0
//! 8c0fc620  bf.s 8c0fc628 ; mov.l r0,@(8,r1)           ; delay always stores
//! 8c0fc624  bra 8c0fc62a ; nop
//! 8c0fc628  mov #0,r1
//! 8c0fc62a  mov.l r1,@(4,r13)
//! 8c0fc62c  mov.l @(4,r13),r12 ; tst r12,r12 ; bt 8c0fc64c
//! 8c0fc632  tst r12,r12 ; bt 8c0fc646                  ; dead: r12 != 0
//! 8c0fc636  mov.l 8c0fc66c,r0 ; jsr @r0 ; mov.l @(12,r12),r4
//! 8c0fc63c  mov.l 8c0fc670,r0 ; add #4,r12 ; mov r12,r4 ; jsr @r0 ; mov #-1,r5
//! 8c0fc646  mov.l 8c0fc66c,r0 ; jsr @r0 ; mov.l @(4,r13),r4
//! 8c0fc64c  mov r13,r0 ; add #4,r0 ; tst r0,r0
//! 8c0fc652  exts.w r14,r0 ; cmp/pl r0 ; bf 8c0fc65e
//! 8c0fc658  mov.l 8c0fc66c,r0 ; jsr @r0 ; mov r13,r4
//! 8c0fc65e  mov r13,r0 ; lds.l @r15+,pr ; mov.l @r15+,r12
//! 8c0fc664  mov.l @r15+,r13 ; rts ; mov.l @r15+,r14
//! ```
//!
//! The `tst r0,r0` at 8c0fc650 is dead: `exts.w` and `cmp/pl` overwrite both
//! `r0` and T before either is read. The `bra` at 8c0fc612 is unreachable
//! because the value it branches on was just loaded from `[r1+8]` and tested
//! zero, but it is transcribed as the fall-through arm.
//!
//! T at exit is the `cmp/pl` result on the paths that skip the last free. When
//! the owner is positive the last free runs and the callee's final T stands;
//! nothing in the epilogue touches it.
//!
//! `p == -4` with a positive owner is the one arm the oracle cannot reach: the
//! original then calls `free(0xfffffffc)`, which reads its chunk header at
//! 0xfffffff8 and faults on unmapped memory. The call is transcribed here all
//! the same; `0x8c0fc5ec.neg4.json` proves the same arm with a non-positive
//! owner.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    // 0x8c0fc5ec: mov.l r14,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[14]);
    // 0x8c0fc5ee: mov.l r13,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[13]);
    // 0x8c0fc5f0: mov r5,r14
    cpu.r[14] = cpu.r[5];
    // 0x8c0fc5f2: mov.l r12,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[12]);
    // 0x8c0fc5f4: mov r4,r13
    cpu.r[13] = cpu.r[4];
    // 0x8c0fc5f6: sts.l pr,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr);
    // 0x8c0fc5f8: tst r13,r13
    cpu.set_t(cpu.r[13] == 0);
    // 0x8c0fc5fa: bt 0x8c0fc65e
    if !cpu.t() {
        // 0x8c0fc5fc: mov r13,r0
        cpu.r[0] = cpu.r[13];
        // 0x8c0fc5fe: add #4,r0
        cpu.r[0] = cpu.r[0].wrapping_add(4);
        // 0x8c0fc600: tst r0,r0
        cpu.set_t(cpu.r[0] == 0);
        // 0x8c0fc602: bt 0x8c0fc652
        if !cpu.t() {
            // 0x8c0fc604: mov.l @(4,r13),r1
            cpu.r[1] = mem.read_u32(cpu.r[13].wrapping_add(4)).unwrap_or(0);
            // 0x8c0fc606: mov.l @(8,r1),r0
            cpu.r[0] = mem.read_u32(cpu.r[1].wrapping_add(8)).unwrap_or(0);
            // 0x8c0fc608: tst r0,r0
            cpu.set_t(cpu.r[0] == 0);
            // 0x8c0fc60a: bf 0x8c0fc61a
            if cpu.t() {
                // 0x8c0fc60c: mov.l @(8,r1),r0
                cpu.r[0] = mem.read_u32(cpu.r[1].wrapping_add(8)).unwrap_or(0);
                // 0x8c0fc60e: tst r0,r0
                cpu.set_t(cpu.r[0] == 0);
                // 0x8c0fc610: bt 0x8c0fc616
                if cpu.t() {
                    // 0x8c0fc616: mov #1,r0
                    cpu.r[0] = 1;
                }
                // 0x8c0fc612: bra 0x8c0fc618 — taken only when the load above
                // was non-zero, which contradicts the test; fall through.
                // 0x8c0fc618: mov.l r0,@(8,r1)
                let _ = mem.write_u32(cpu.r[1].wrapping_add(8), cpu.r[0]);
            }
            // 0x8c0fc61a: mov.l @(8,r1),r0
            cpu.r[0] = mem.read_u32(cpu.r[1].wrapping_add(8)).unwrap_or(0);
            // 0x8c0fc61c: add #-1,r0
            cpu.r[0] = cpu.r[0].wrapping_sub(1);
            // 0x8c0fc61e: tst r0,r0
            cpu.set_t(cpu.r[0] == 0);
            // 0x8c0fc620: bf.s 0x8c0fc628
            // 0x8c0fc622: mov.l r0,@(8,r1)  (delay slot, always runs)
            let _ = mem.write_u32(cpu.r[1].wrapping_add(8), cpu.r[0]);
            if !cpu.t() {
                // 0x8c0fc628: mov #0,r1
                cpu.r[1] = 0;
            }
            // 0x8c0fc624: bra 0x8c0fc62a (not-taken arm)
            // 0x8c0fc62a: mov.l r1,@(4,r13)
            let _ = mem.write_u32(cpu.r[13].wrapping_add(4), cpu.r[1]);
            // 0x8c0fc62c: mov.l @(4,r13),r12
            cpu.r[12] = mem.read_u32(cpu.r[13].wrapping_add(4)).unwrap_or(0);
            // 0x8c0fc62e: tst r12,r12
            cpu.set_t(cpu.r[12] == 0);
            // 0x8c0fc630: bt 0x8c0fc64c
            if !cpu.t() {
                // 0x8c0fc632: tst r12,r12
                cpu.set_t(cpu.r[12] == 0);
                // 0x8c0fc634: bt 0x8c0fc646 — dead guard, r12 is non-null.
                // 0x8c0fc636: mov.l 0x8c0fc66c,r0
                cpu.r[0] = mem.read_u32(0x8c0fc66c).unwrap_or(0); // 0x8c01032c
                // 0x8c0fc638: jsr @r0
                // 0x8c0fc63a: mov.l @(12,r12),r4  (delay slot)
                cpu.r[4] = mem.read_u32(cpu.r[12].wrapping_add(12)).unwrap_or(0);
                cpu.pr = 0x8c0fc63c;
                crate::registry::p_8c01032c::run(cpu, mem);
                // 0x8c0fc63c: mov.l 0x8c0fc670,r0
                cpu.r[0] = mem.read_u32(0x8c0fc670).unwrap_or(0); // 0x8c018e14
                // 0x8c0fc63e: add #4,r12
                cpu.r[12] = cpu.r[12].wrapping_add(4);
                // 0x8c0fc640: mov r12,r4
                cpu.r[4] = cpu.r[12];
                // 0x8c0fc642: jsr @r0
                // 0x8c0fc644: mov #-1,r5  (delay slot)
                cpu.r[5] = 0xffff_ffff;
                cpu.pr = 0x8c0fc646;
                crate::registry::p_8c018e14::run(cpu, mem);
                // 0x8c0fc646: mov.l 0x8c0fc66c,r0
                cpu.r[0] = mem.read_u32(0x8c0fc66c).unwrap_or(0); // 0x8c01032c
                // 0x8c0fc648: jsr @r0
                // 0x8c0fc64a: mov.l @(4,r13),r4  (delay slot)
                cpu.r[4] = mem.read_u32(cpu.r[13].wrapping_add(4)).unwrap_or(0);
                cpu.pr = 0x8c0fc64c;
                crate::registry::p_8c01032c::run(cpu, mem);
            }
            // 0x8c0fc64c: mov r13,r0
            cpu.r[0] = cpu.r[13];
            // 0x8c0fc64e: add #4,r0
            cpu.r[0] = cpu.r[0].wrapping_add(4);
            // 0x8c0fc650: tst r0,r0
            cpu.set_t(cpu.r[0] == 0);
        }
        // 0x8c0fc652: exts.w r14,r0
        cpu.r[0] = cpu.r[14] as i16 as i32 as u32;
        // 0x8c0fc654: cmp/pl r0
        let owner_positive = (cpu.r[0] as i32) > 0;
        cpu.set_t(owner_positive);
        // 0x8c0fc656: bf 0x8c0fc65e
        if owner_positive {
            // 0x8c0fc658: mov.l 0x8c0fc66c,r0
            cpu.r[0] = mem.read_u32(0x8c0fc66c).unwrap_or(0); // 0x8c01032c
            // 0x8c0fc65a: jsr @r0
            // 0x8c0fc65c: mov r13,r4  (delay slot)
            cpu.r[4] = cpu.r[13];
            cpu.pr = 0x8c0fc65e;
            crate::registry::p_8c01032c::run(cpu, mem);
        }
    }
    // 0x8c0fc65e: mov r13,r0
    cpu.r[0] = cpu.r[13];
    // 0x8c0fc660: lds.l @r15+,pr
    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    // 0x8c0fc662: mov.l @r15+,r12
    cpu.r[12] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    // 0x8c0fc664: mov.l @r15+,r13
    cpu.r[13] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    // 0x8c0fc666: rts ; 0x8c0fc668: mov.l @r15+,r14 (delay slot)
    cpu.r[14] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}
