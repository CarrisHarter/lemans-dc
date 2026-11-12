//! 0x8c111648 — insertion sort over [r4,r5) by triple-indirect key.
//!
//! Slots are 4-byte element pointers from r4 (inclusive) to r5 (exclusive).
//! Each element's key is `[[[elem+4]]]` via 0x8c10e374, compared signed; the
//! sort is ascending. The inner test is NULL-guarded:
//!
//! ```text
//! while r11 > r9 (unsigned):
//!   r1 = [r12]                       // delay slot of `bt.s`, always runs
//!   if r13 == 0:          shift      // null insert bubbles to the front
//!   elif r1 == 0:         stop       // null left slot under non-null insert
//!   elif key(left) > key(insert) (signed): shift
//!   else:                 stop
//! ```
//!
//! `bf`/`bt` are plain (no delay slot); `bt.s`, both `bra`s, both `jsr`s and
//! the final `rts` run their delay slots. The `jsr`s return to 0x8c111690 and
//! 0x8c111698 (post-delay insns). Callee-saved r8-r14/pr/r15 are restored; r0,
//! r1, r4 and T keep their last inner-test values (T is always 0 at exit: both
//! exits leave it clear). Stack traffic is excluded from the diff, so the push
//! slots and the spilled end pointer are modelled as locals.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    // Prologue pushes r14..r8 and pr, then spills the end pointer.
    let (save8, save9, save10, save11, save12, save13, save14) = (
        cpu.r[8],
        cpu.r[9],
        cpu.r[10],
        cpu.r[11],
        cpu.r[12],
        cpu.r[13],
        cpu.r[14],
    );
    cpu.r[9] = cpu.r[4]; // base
    let end = cpu.r[5]; // [r15] spill
    cpu.r[8] = cpu.r[9].wrapping_add(4); // cursor
    cpu.r[1] = cpu.r[5];
    cpu.set_t(cpu.r[1] > cpu.r[8]); // cmp/hi r8,r1
    if cpu.t() {
        // `bf empty` not taken: at least two slots.
        loop {
            // outer: r13 = insert element; r11/r12 = write/scan cursors.
            cpu.r[13] = mem.read_u32(cpu.r[8]).unwrap_or(0);
            cpu.r[12] = cpu.r[8];
            cpu.r[11] = cpu.r[8];
            cpu.r[12] = cpu.r[12].wrapping_sub(4); // delay slot of `bra test`
            loop {
                // test:
                cpu.set_t(cpu.r[11] > cpu.r[9]); // cmp/hi r9,r11
                if !cpu.t() {
                    break; // bf place
                }
                cpu.set_t(cpu.r[13] == 0); // tst r13,r13
                let null_key = cpu.t();
                cpu.r[1] = mem.read_u32(cpu.r[12]).unwrap_or(0); // delay slot of `bt.s`
                if null_key {
                    // nullkey with r13 == 0: movt yields 1, so shift.
                    cpu.set_t(cpu.r[13] == 0); // tst r13,r13
                    cpu.r[0] = u32::from(cpu.t()); // movt r0
                } else {
                    cpu.set_t(cpu.r[1] == 0); // tst r1,r1
                    if cpu.t() {
                        // nullkey with r13 != 0: movt yields 0, so stop.
                        cpu.set_t(cpu.r[13] == 0); // tst r13,r13
                        cpu.r[0] = u32::from(cpu.t()); // movt r0
                    } else {
                        // Both non-null: shift while key(left) > key(insert).
                        cpu.r[4] = mem.read_u32(cpu.r[13].wrapping_add(4)).unwrap_or(0);
                        cpu.r[10] = mem.read_u32(cpu.r[1].wrapping_add(4)).unwrap_or(0); // delay slot of `jsr`
                        crate::registry::p_8c10e374::run(cpu, mem); // r0 = key(insert)
                        cpu.r[14] = cpu.r[0];
                        cpu.r[4] = cpu.r[10]; // delay slot of `jsr`
                        crate::registry::p_8c10e374::run(cpu, mem); // r0 = key(left)
                        cpu.set_t((cpu.r[0] as i32) > (cpu.r[14] as i32)); // cmp/gt r14,r0
                        cpu.r[0] = u32::from(cpu.t()); // movt r0 (delay slot of `bra`)
                    }
                }
                // merge:
                cpu.r[0] &= 0xff; // extu.b r0,r0
                cpu.set_t(cpu.r[0] == 0); // tst r0,r0
                if cpu.t() {
                    break; // bf shift not taken -> place
                }
                // shift:
                cpu.r[0] = mem.read_u32(cpu.r[12]).unwrap_or(0);
                let _ = mem.write_u32(cpu.r[11], cpu.r[0]);
                cpu.r[12] = cpu.r[12].wrapping_sub(4);
                cpu.r[11] = cpu.r[11].wrapping_sub(4);
            }
            // place:
            let _ = mem.write_u32(cpu.r[11], cpu.r[13]);
            cpu.r[8] = cpu.r[8].wrapping_add(4);
            cpu.r[1] = end; // mov.l @r15,r1
            cpu.set_t(cpu.r[1] > cpu.r[8]); // cmp/hi r8,r1
            if !cpu.t() {
                break; // bt outer not taken -> empty
            }
        }
    }
    // empty: pops pr, r8..r13; the `rts` delay slot pops r14. r15 balanced.
    cpu.r[8] = save8;
    cpu.r[9] = save9;
    cpu.r[10] = save10;
    cpu.r[11] = save11;
    cpu.r[12] = save12;
    cpu.r[13] = save13;
    cpu.r[14] = save14;
}
