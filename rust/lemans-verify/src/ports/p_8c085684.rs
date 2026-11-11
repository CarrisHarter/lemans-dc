//! 0x8c085684 — add two 4-short time-of-day vectors with carry normalisation.
//!
//! p3 (loaded from @(12,r15) after three pushes, i.e. @r15 at entry) receives
//! [sum0 mod 1000, secs, mins, hours] with carries: while se16(sum0) >= 1000,
//! sum0 -= 1000 and secs += 1; while se16(secs) >= 60, secs -= 60 and
//! mins += 1; while se16(mins) >= 60, mins -= 60 and hours += 1.
//! Side effects the C does not show: mov.w loads sign-extend, so the sums are
//! of sign-extended shorts; r5 is clobbered to 60 (`mov #60,r5`), r7 = 1000,
//! r0 = final r4, and the pushed r12/r13/r14 plus r15 are restored by the
//! pops (r15 net unchanged). T ends 0 (last `cmp/ge` of the minutes loop).

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let p1 = cpu.r[4];
    let p2 = cpu.r[5];
    // mov.w loads sign-extend on SH-4
    let ldw = |a: u32| mem.read_u16(a).unwrap_or(0) as i16 as i32 as u32;
    let se = |v: u32| v as u16 as i16 as i32;
    let p3 = mem.read_u32(cpu.r[15]).unwrap_or(0); // mov.l @(12,r15),r14
    let mut r3 = ldw(p1 + 2).wrapping_add(ldw(p2 + 2)); // secs
    let mut r1 = ldw(p1).wrapping_add(ldw(p2)); // exts.w r1,r1 is a no-op on it
    let mut r6 = ldw(p1 + 4).wrapping_add(ldw(p2 + 4)); // mins
    let mut r4 = ldw(p1 + 6).wrapping_add(ldw(p2 + 6)); // bf.s delay: add r0,r4
    let r7 = mem.read_u32(0x8c0856fc).unwrap_or(0); // mov.l literal (1000)
    let r13 = se(r7); // exts.w r7,r13
    let r12 = se(r1); // exts.w r1,r12
    if r12 >= r13 {
        // bf.s not taken: loop at 0x8c0856b8
        loop {
            r1 = r1.wrapping_sub(r7); // sub r7,r1
            let t = se(r1) >= r13; // exts.w r1,r0 ; cmp/ge r13,r0
            r3 = r3.wrapping_add(1); // bt.s delay slot: add #1,r3
            if !t {
                break;
            }
        }
    }
    cpu.r[5] = 60; // mov #60,r5
    if se(r3) >= 60 {
        // bf not taken: loop at 0x8c0856ca
        loop {
            r3 = r3.wrapping_sub(60); // add #-60,r3
            let t = se(r3) >= 60; // exts.w r3,r0 ; cmp/ge r5,r0
            r6 = r6.wrapping_add(1); // bt.s delay slot: add #1,r6
            if !t {
                break;
            }
        }
    }
    cpu.r[5] = 60; // mov #60,r5
    let mut t = se(r6) >= 60; // exts.w r6,r0 ; cmp/ge r5,r0
    if t {
        // bf not taken: loop at 0x8c0856dc
        loop {
            r6 = r6.wrapping_sub(60); // add #-60,r6
            t = se(r6) >= 60;
            r4 = r4.wrapping_add(1); // bt.s delay slot: add #1,r4
            if !t {
                break;
            }
        }
    }
    cpu.set_t(t);
    let _ = mem.write_u16(p3, r1 as u16); // mov.w r1,@r14
    let _ = mem.write_u16(p3 + 2, r3 as u16); // mov r3,r0 ; mov.w r0,@(2,r14)
    let _ = mem.write_u16(p3 + 4, r6 as u16); // mov r6,r0 ; mov.w r0,@(4,r14)
    let _ = mem.write_u16(p3 + 6, r4 as u16); // mov r4,r0 ; mov.w r0,@(6,r14)
    cpu.r[0] = r4; // mov r4,r0 (rts delay slot only pops r14)
    cpu.r[1] = r1;
    cpu.r[3] = r3;
    cpu.r[4] = r4;
    cpu.r[6] = r6;
    cpu.r[7] = r7;
    // r12/r13/r14 are popped back to their entry values; r15 net unchanged
}
