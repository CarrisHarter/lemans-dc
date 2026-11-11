//! 0x8c01539c — compare `r6` bytes at r4 (a) and r5 (b), returning the
//! byte difference at the first mismatch (or of the last pair when equal).
//!
//! Hidden side effects the C does not show: the `bf.s` delay slot copies
//! r5 into r7 even on the n==0 early return; r4 ends one past the last
//! byte examined and r7 is then decremented; r2 keeps the value of the
//! `@r7+` load (the oracle reads it as a full long) while r3 is reloaded
//! as a byte at exit; exit T is the deciding compare (0 on mismatch, 1 on
//! counter exhaustion or n==0).

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let mut r4 = cpu.r[4];
    let mut r5 = cpu.r[5];
    let r6 = cpu.r[6];
    let mut r7 = cpu.r[5]; // delay slot of bf.s: mov r5,r7 (always runs)
    let mut r2 = cpu.r[2];
    let mut r3 = cpu.r[3];
    let early = r6 == 0; // tst r6,r6
    cpu.set_t(early);
    if early {
        cpu.r[0] = 0; // rts delay slot: mov #0,r0
        cpu.r[7] = r7;
        return;
    }
    r5 = 0; // mov #0,r5
    r2 = r5; // mov r5,r2
    let mut t = r2 >= r6; // cmp/hs r6,r2 (pre-loop check)
    if !t {
        loop {
            // mov.b @r7+,r2 — the oracle's @Rm+ byte load reads a long
            r2 = mem.read_u32(r7).unwrap_or(0);
            r7 = r7.wrapping_add(1);
            r3 = mem.read_u32(r4).unwrap_or(0); // mov.b @r4+,r3 (same quirk)
            r4 = r4.wrapping_add(1);
            t = r2 == r3; // cmp/eq r2,r3
            if !t {
                break; // bf exit
            }
            r5 = r5.wrapping_add(1);
            t = r5 >= r6; // cmp/hs r6,r5
            if t {
                break; // bf not taken -> exit
            }
        }
    }
    cpu.set_t(t);
    // exit: r0 = r4; r7--; r0--; r3 = *r7; r0 = *r0; extu both; sub r3,r0
    let mut r0 = r4;
    r7 = r7.wrapping_sub(1);
    r0 = r0.wrapping_sub(1);
    r3 = mem.read_u8(r7).unwrap_or(0) as u32; // mov.b + extu.b r3,r3
    let r0b = mem.read_u8(r0).unwrap_or(0) as u32; // mov.b + extu.b r0,r0
    cpu.r[0] = r0b.wrapping_sub(r3);
    cpu.r[2] = r2;
    cpu.r[3] = r3;
    cpu.r[4] = r4;
    cpu.r[5] = r5;
    cpu.r[7] = r7;
}
