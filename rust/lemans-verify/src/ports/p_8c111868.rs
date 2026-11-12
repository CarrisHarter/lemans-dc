//! 0x8c111868 — insertion sort over a pointer array [r4, r5), ascending.
//!
//! Twin of 0x8c10d5f0 with two image-confirmed differences:
//! - the key loader is 0x8c10e35c via pool word 0x8c1118d0: key = [[elem]+68]
//! - the inner compare is `cmp/gt r14,r0` (T = (r0 > r14) SIGNED) where
//!   r14 = key(insert value) and r0 = key(element to the left), so the loop
//!   shifts while key(left) > key(insert): ascending order (the twin, with
//!   swapped operands, sorts descending).
//!
//! Hidden side effects the disassembly shows:
//! - r8-r14 and pr are saved on the stack and restored; net register effects
//!   are r0 (last key/shifted word), r1 (= r5), r4 (last r10) and r8
//!   (first slot >= r5). T is 0 on every exit path (both exits leave a
//!   `cmp/hi` with T == 0).
//! - the `bra` at 0x8c11188e has delay slot `add #-4,r12`: r12 starts one
//!   slot before r11.
//! - each `jsr` delay slot runs before the key loader: `mov.l @r12,r10`
//!   feeds the second call, `mov r10,r4` sets its argument.
//! - the `jsr`s clobber pr mid-function, but the epilogue restores it.

use sh4_oracle::{Cpu, Memory};

/// Key loader 0x8c10e35c inlined: `r0=[r4] ; r0+=68 ; rts ; r0=[r0]`.
/// Only r0 is clobbered; the caller's pr is restored by this function's
/// own epilogue, so the net effect is just the returned word.
#[inline]
fn key(mem: &Memory, elem: u32) -> u32 {
    let mid = mem.read_u32(elem).unwrap();
    mem.read_u32(mid.wrapping_add(68)).unwrap()
}

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    // Prologue pushes r14..r8, then pr, then stores r5 below them; all of
    // that is stack traffic inside the excluded window and is restored by
    // the epilogue, so only the live values are tracked here.
    let r5 = cpu.r[5];
    let r9 = cpu.r[4]; // mov r4,r9
    let mut r8 = r9.wrapping_add(4); // mov r9,r8 ; add #4,r8
    let mut r1 = r5; // mov r5,r1
    let mut r0 = cpu.r[0];
    let mut r4 = cpu.r[4];

    cpu.set_t(r1 > r8); // cmp/hi r8,r1 (unsigned)
    if r1 > r8 {
        // bf 0x8c1118bc not taken: outer loop over slots r4+4..r5
        loop {
            let r13 = mem.read_u32(r8).unwrap(); // mov.l @r8,r13
            let mut r11 = r8; // mov r8,r11
            // bra 0x8c11189a with delay slot `add #-4,r12` (r12 = r8 - 4)
            let mut r12 = r8.wrapping_sub(4);
            // inner loop: shift left neighbours right while larger
            loop {
                cpu.set_t(r11 > r9); // cmp/hi r9,r11 (unsigned)
                if !(r11 > r9) {
                    break; // bf 0x8c1118b2: insert point
                }
                // jsr key(r13); delay slot `mov.l @r12,r10` runs first
                let r10 = mem.read_u32(r12).unwrap();
                let r14 = key(mem, r13); // mov r0,r14 after return
                // jsr key(r10); delay slot `mov r10,r4` runs first
                r4 = r10;
                r0 = key(mem, r10);
                // cmp/gt r14,r0: T = (r0 > r14) SIGNED
                let t = (r0 as i32) > (r14 as i32);
                cpu.set_t(t);
                if !t {
                    break; // bt 0x8c111892 not taken: insert
                }
                r0 = mem.read_u32(r12).unwrap(); // mov.l @r12,r0
                let _ = mem.write_u32(r11, r0); // mov.l r0,@r11
                r12 = r12.wrapping_sub(4); // add #-4,r12
                r11 = r11.wrapping_sub(4); // add #-4,r11
            }
            let _ = mem.write_u32(r11, r13); // mov.l r13,@r11
            r8 = r8.wrapping_add(4); // add #4,r8
            r1 = r5; // mov.l @r15,r1 (the saved r5)
            cpu.set_t(r1 > r8); // cmp/hi r8,r1 (unsigned)
            if !(r1 > r8) {
                break; // bt 0x8c111888 not taken: done
            }
        }
    }

    cpu.r[0] = r0;
    cpu.r[1] = r1;
    cpu.r[4] = r4;
    // r2,r3,r5-r7 never touched; r8-r14, pr, r15 restored by the epilogue.
}
