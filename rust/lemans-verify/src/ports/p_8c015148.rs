//! 0x8c015148 — rotate a byte array right through carry, walking from
//! `r4` up for `r5` bytes; returns the final carry bit.
//!
//! Hidden side effects the C does not show: r12/r13/r14 are pushed and
//! popped, so only r0,r3,r4,r6,r7 survive; the entry `bf.s` delay slot
//! sets r13 = 1, `r4` and `r6` both advance to base+n (they stay equal),
//! r14 counts the iterations, r3 keeps the last byte >> 1, r12 keeps
//! last & 1, and the exit T bit is the final `cmp/ge r5,r14` (true when
//! the loop ran, `cmp/pl r5` when it did not).

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let n = cpu.r[5];
    let mut r4 = cpu.r[4];
    let mut r6 = cpu.r[4]; // mov #0,r6 ; add r4,r6
    let enter = (n as i32) > 0; // cmp/pl r5
    let mut r7: u32 = 0;
    let r13: u32 = 1; // delay slot of bf.s: mov #1,r13 (saved/restored reg)
    let mut r14: u32 = 0; // mov r7,r14
    let mut r3 = cpu.r[3];
    let mut r12 = cpu.r[12];
    r4 = r6; // mov r6,r4
    if enter {
        loop {
            let b = mem.read_u8(r4).unwrap_or(0);
            r3 = (b as u32) >> 1; // extu.b r3,r3 ; shlr r3
            r12 = (b as u32) & r13; // extu.b r12,r12 ; and r13,r12
            let carry_in = r7 != 0; // tst r7,r7
            // bt.s delay slot always stores the shifted byte
            let _ = mem.write_u8(r4, r3 as u8);
            if carry_in {
                let r0 = ((mem.read_u8(r6).unwrap_or(0) as i8 as i32) as u32) | 0x80;
                let _ = mem.write_u8(r6, r0 as u8);
            }
            r14 = r14.wrapping_add(1);
            r7 = r12;
            let cont = (r14 as i32) >= (n as i32); // cmp/ge r5,r14
            r6 = r6.wrapping_add(1);
            r4 = r4.wrapping_add(1); // delay slot of bf.s
            if !cont {
                continue;
            }
            break;
        }
        cpu.set_t((r14 as i32) >= (n as i32));
    } else {
        cpu.set_t(enter);
    }
    cpu.r[0] = r7; // mov r7,r0
    cpu.r[3] = r3;
    cpu.r[4] = r4;
    cpu.r[6] = r6;
    cpu.r[7] = r7;
}
