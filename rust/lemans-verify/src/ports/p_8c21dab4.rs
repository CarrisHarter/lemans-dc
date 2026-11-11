//! 0x8c21dab4 — checksum validate of 8 bytes at r4.
//!
//! r7=6, r5=r6=0; loop i=0..5: r6 += zero_ext(*(u8*)(r4+2+i)) (bf.s delay slot
//! `add r0,r6` runs every iteration, final T from cmp/ge r7,r5 = 1, r5=6).
//! r4 = r6 & 0xff, r5 = r4; r0 = zero_ext(*(r4+1)). If r0 == r5 then
//! r2 = (~r5)&0xff, r3 = zero_ext(*(r4+0)); if r3 == r2 -> r0=1 (T=1) else
//! r0=0 (T=0). If r0 != r5 -> r0=0 (T=0), r2/r3 keep loop values.
//! rts delay slot add #8,r15 cancels the add #-8,r15. Stack window excluded.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let r4e = cpu.r[4];
    let r2e = cpu.r[2];
    let r7: u32 = 6; // mov #6,r7
    let mut r5: u32 = 0; // mov #0,r5
    let mut r6: u32 = 0; // mov #0,r6
    let mut r3 = 0u32;
    loop {
        r3 = r4e.wrapping_add(r5); // mov r5,r3 ; add r4,r3
        let r0 = mem.read_u8(r3.wrapping_add(2)).unwrap_or(0) as u32; // mov.b @(2,r3),r0
        r5 = r5.wrapping_add(1); // add #1,r5
        let t = (r5 as i32) >= (r7 as i32); // cmp/ge r7,r5
        cpu.set_t(t);
        let r0 = r0 & 0xff; // extu.b r0,r0
        r6 = r6.wrapping_add(r0); // bf.s delay slot: add r0,r6 (always)
        if t {
            break;
        }
    }
    let b1 = mem.read_u8(r4e + 1).unwrap_or(0) as u32; // mov.b @(4,r15),r0 (stack held *(r4+1))
    let r4n = r6 & 0xff; // extu.b r6,r4
    let r5n = r4n & 0xff; // extu.b r4,r5
    let r0 = b1 & 0xff; // extu.b r0,r0
    cpu.set_t(r0 == r5n); // cmp/eq r0,r5
    let mut r0f = 0u32;
    let mut r2 = r2e;
    let mut r3f = r3;
    if r0 == r5n {
        let b0 = mem.read_u8(r4e).unwrap_or(0) as u32; // mov.b @r15,r3 (stack held *(r4))
        r2 = (!r5n) & 0xff; // not r5,r2 ; extu.b r2,r2
        r3f = b0 & 0xff; // extu.b r3,r3
        cpu.set_t(r3f == r2); // cmp/eq r3,r2
        if r3f == r2 {
            r0f = 1; // mov #1,r0
        }
    }
    cpu.r[0] = r0f;
    cpu.r[2] = r2;
    cpu.r[3] = r3f;
    cpu.r[4] = r4n;
    cpu.r[5] = r5n;
    cpu.r[6] = r6;
    cpu.r[7] = r7;
    // r15 net unchanged (add #-8,r15 ... rts delay add #8,r15)
}
