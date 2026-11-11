//! 0x8c014dc0 — store `*(u16 *)r5` big-endian into the 2 bytes at r4.
//!
//! Hidden side effects the C does not show: r0/r6 end at r4+2, r7 at -8 (the
//! `add #-8,r7` delay slot runs on the last pass), r2 at 0, r3 keeps the
//! zero-extended u16 because the last `shld` amount is 0, and T is left set.
//! The `mov.w @r5,r3` load sign-extends before `extu.w` re-truncates.

use sh4_oracle::{Cpu, Memory};

fn shld(amount: i32, v: u32) -> u32 {
    if amount >= 0 {
        v << (amount as u32 & 31)
    } else {
        v >> ((-amount) as u32 & 31)
    }
}

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[0] = cpu.r[4].wrapping_add(2); // mov r4,r0 ; add #2,r0
    cpu.r[7] = 8; // mov #8,r7
    cpu.r[6] = cpu.r[4]; // mov r4,r6
    loop {
        cpu.r[3] = mem.read_u16(cpu.r[5]).unwrap() as i16 as i32 as u32; // mov.w @r5,r3
        cpu.r[2] = (cpu.r[7] as i32).wrapping_neg() as u32; // neg r7,r2
        cpu.r[3] &= 0xFFFF; // extu.w r3,r3
        cpu.r[3] = shld(cpu.r[2] as i32, cpu.r[3]); // shld r2,r3
        let _ = mem.write_u8(cpu.r[6], cpu.r[3] as u8); // mov.b r3,@r6
        cpu.r[6] = cpu.r[6].wrapping_add(1); // add #1,r6
        let t = cpu.r[6] >= cpu.r[0]; // cmp/hs r0,r6
        cpu.r[7] = cpu.r[7].wrapping_add((-8i32) as u32); // bf.s delay slot: add #-8,r7
        cpu.set_t(t);
        if t {
            break;
        }
    }
    // rts ; nop
}
