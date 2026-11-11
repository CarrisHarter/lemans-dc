//! 0x8c215f6c — table-driven CRC-style fold over `r4` bytes at `r5`.
//!
//! Original:
//! ```text
//! mov.l r14,@-r15 ; r14 = 0x8c23bdc8 (table ptr global) ; r6 = 0xffff
//! bra check ; nop
//! body: extu.w r6,r7 ; r0 = r7 ; r3 = -8 ; shad r3,r0 (r0 = r7 >> 8)
//!       r3 = mov.b @r5+ ; r2 = r7 ; shll8 r2 ; extu.b r3,r3 ; r6 = r2
//!       xor r3,r0 ; shll r0 ; r1 = mov.w @(r0,r14) ; xor r1,r6
//! check: add #-1,r4 ; cmp/pz r4 ; bt body
//! r0 = extu.w r6 ; not r0,r0 ; extu.w r0,r0 ; rts ; mov.l @r15+,r14 (delay)
//! ```
//! `mov.w` sign-extends the table entry before the xor. The loop is
//! bottom-tested: `r4` bytes are folded for r4 >= 0 (none for r4 <= 0).
//! `r14` is restored from the stack in the rts delay slot, r15 net unchanged.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let saved_r14 = cpu.r[14];
    let table = mem.read_u32(0x8c215fa4).unwrap_or(0); // pool value IS the table base
    cpu.r[14] = table;
    let table_base = table;
    let mut r6 = mem.read_u32(0x8c215fa0).unwrap_or(0); // 0xffff
    cpu.r[6] = r6;
    let mut r4 = cpu.r[4];
    // bra to the check first (delay slot: nop)
    loop {
        // 0x8c215f90: add #-1,r4 ; cmp/pz r4 ; bt body
        r4 = r4.wrapping_sub(1);
        cpu.r[4] = r4;
        cpu.set_t((r4 as i32) >= 0);
        if (r4 as i32) < 0 {
            break;
        }
        // 0x8c215f76 body
        let r7 = r6 & 0xFFFF; // extu.w r6,r7
        cpu.r[7] = r7;
        let mut r0 = r7;
        // shad r3,r0 with r3 = -8: arithmetic shift right by 8
        r0 = ((r0 as i32) >> 8) as u32;
        cpu.r[0] = r0;
        let mut r3 = mem.read_u8(cpu.r[5]).unwrap_or(0) as i8 as i32 as u32; // mov.b @r5+,r3
        cpu.r[5] = cpu.r[5].wrapping_add(1);
        let mut r2 = r7 << 8; // shll8
        cpu.r[2] = r2;
        r3 &= 0xFF; // extu.b r3,r3
        cpu.r[3] = r3;
        r6 = r2;
        cpu.r[6] = r6;
        r0 ^= r3; // xor r3,r0
        cpu.r[0] = r0;
        cpu.set_t(r0 >> 31 != 0); // shll r0
        r0 = r0 << 1;
        cpu.r[0] = r0;
        let r1 = mem.read_u16(table_base.wrapping_add(r0)).unwrap_or(0) as u16 as i16 as i32 as u32;
        cpu.r[1] = r1; // mov.w @(r0,r14),r1 (sign-extended)
        r6 ^= r1; // xor r1,r6
        cpu.r[6] = r6;
    }
    // 0x8c215f96: extu.w r6,r0 ; not r0,r0 ; extu.w r0,r0
    cpu.r[0] = (!(r6 & 0xFFFF)) & 0xFFFF;
    // rts delay slot: mov.l @r15+,r14 restores the caller's r14
    cpu.r[14] = saved_r14;
}
