//! 0x8c11181c — insertion sort over a pointer array [r4, r5).
//!
//! Compares element structs by the u32 at +24 (secondary key) and, when those
//! are equal, by the u32 at +0 (primary key); signed `cmp/gt`, `movt` in the
//! `bra` delay slot feeds the compare result into r0.
//!
//! Hidden side effects the C does not show:
//! - r3/r6/r7 are left clobbered (r14/r13 are saved/restored via the stack).
//! - the `bra` at 0x8c11182e has delay slot `add #-4,r7` — r7 starts one slot
//!   before r3.
//! - the `bt.s` at 0x8c11185e runs its delay slot `mov.l r6,@r14` (the actual
//!   insertion store) whether or not the branch is taken.
//! - T is 0 on every exit path (both exits are taken/not-taken `cmp/hi` with
//!   T == 0).

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    // prologue: mov.l r14,@-r15 ; mov.l r13,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[14]);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[13]);

    let r4 = cpu.r[4];
    let r5 = cpu.r[5];
    let mut r3 = r4.wrapping_add(4); // mov r4,r3 ; add #4,r3
    let mut r6 = cpu.r[6];
    let mut r7 = cpu.r[7];
    let mut r0 = cpu.r[0];
    let mut r1 = cpu.r[1];

    'outer: loop {
        cpu.set_t(r5 > r3); // cmp/hi r3,r5 (unsigned)
        if !(r5 > r3) {
            break 'outer; // bf 0x8c111862
        }
        r6 = mem.read_u32(r3).unwrap(); // mov.l @r3,r6
        r7 = r3; // mov r3,r7
        let mut r14 = r3; // mov r3,r14
        r7 = r7.wrapping_sub(4); // delay slot of bra 0x8c111838
        loop {
            cpu.set_t(r14 > r4); // cmp/hi r4,r14 (unsigned)
            if !(r14 > r4) {
                break; // bf 0x8c11185a (insert point)
            }
            let r13 = mem.read_u32(r7).unwrap(); // mov.l @r7,r13
            r1 = mem.read_u32(r6.wrapping_add(24)).unwrap(); // mov.l @(24,r6),r1
            r0 = mem.read_u32(r13.wrapping_add(24)).unwrap(); // mov.l @(24,r13),r0
            cpu.set_t(r0 == r1); // cmp/eq r0,r1
            if r0 == r1 {
                r1 = mem.read_u32(r6).unwrap(); // mov.l @r6,r1
                r0 = mem.read_u32(r13).unwrap(); // mov.l @r13,r0
                cpu.set_t((r0 as i32) > (r1 as i32)); // cmp/gt r1,r0
            } else {
                cpu.set_t((r1 as i32) > (r0 as i32)); // cmp/gt r0,r1
            }
            r0 = cpu.t() as u32; // movt r0 (delay slot of bra 0x8c111854 / fallthrough)
            r0 &= 0xFF; // extu.b r0,r0
            cpu.set_t(r0 & r0 == 0); // tst r0,r0
            if r0 != 0 {
                // bf 0x8c111832 (shift): delay-slot-free fall into the loop body
                let _ = mem.write_u32(r14, r13); // mov.l r13,@r14
                r7 = r7.wrapping_sub(4); // add #-4,r7
                r14 = r14.wrapping_sub(4); // add #-4,r14
                continue;
            }
            break; // fall through to 0x8c11185a
        }
        // 0x8c11185a: add #4,r3 ; cmp/hi r3,r5 ; bt.s 0x8c111828 ; mov.l r6,@r14
        r3 = r3.wrapping_add(4);
        let _ = mem.write_u32(r14, r6); // delay slot of bt.s: ALWAYS runs
        cpu.set_t(r5 > r3); // cmp/hi r3,r5
        if r5 > r3 {
            continue 'outer; // bt.s 0x8c111828
        }
        break 'outer;
    }

    cpu.r[3] = r3;
    cpu.r[6] = r6;
    cpu.r[7] = r7;
    cpu.r[0] = r0;
    cpu.r[1] = r1;
    // epilogue: mov.l @r15+,r13 ; rts ; mov.l @r15+,r14
    cpu.r[13] = mem.read_u32(cpu.r[15]).unwrap();
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[14] = mem.read_u32(cpu.r[15]).unwrap();
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}
