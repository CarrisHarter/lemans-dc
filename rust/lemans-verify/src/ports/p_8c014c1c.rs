//! 0x8c014c1c — shift a byte buffer's bits down by r6, returning 1 if a bit
//! was carried out past the start.
//!
//! Hidden side effects the C does not show: r1 is left holding the
//! `shad(n, 0xff)` mask, r7 the final carry byte, r3 = -n, r5 ends at r4-1
//! (or r4+len-1 when the loop is skipped), and the return value in r0 on the
//! n==0 path is the caller's incoming r0, not a constant. r14/r15 are
//! preserved via the stack.

use sh4_oracle::{Cpu, Memory};

fn shad(amount: i32, v: u32) -> u32 {
    if amount >= 0 {
        v << (amount as u32 & 31)
    } else {
        ((v as i32) >> ((-amount) as u32 & 31)) as u32
    }
}

fn shld(amount: i32, v: u32) -> u32 {
    if amount >= 0 {
        v << (amount as u32 & 31)
    } else {
        v >> ((-amount) as u32 & 31)
    }
}

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[7] = 8u32.wrapping_sub(cpu.r[6]); // mov #8,r7 ; sub r6,r7
    let n = (cpu.r[7] as u16 as i16 as i32) as u32; // exts.w r7,r14 (local; r14 restored by pop)
    cpu.set_t(n == 0); // tst r14,r14
    if n == 0 {
        return; // bt 0x8c014c62 -> rts ; mov.l @r15+,r14
    }
    cpu.r[5] = cpu.r[5].wrapping_sub(1); // add #-1,r5
    cpu.r[1] = mem.read_u16(0x8C014C66).unwrap() as i16 as i32 as u32; // mov.w 0x8c014c66,r1 (0x00ff)
    cpu.r[5] = cpu.r[5].wrapping_add(cpu.r[4]); // add r4,r5
    let t = cpu.r[5] >= cpu.r[4]; // cmp/hs r4,r5
    cpu.r[1] = shad(n as i32, cpu.r[1]); // shad r14,r1
    cpu.r[7] = 0; // bf.s delay slot: mov #0,r7
    if t {
        loop {
            cpu.r[2] = mem.read_u8(cpu.r[5]).unwrap() as i8 as i32 as u32; // mov.b @r5,r2
            let r3 = (cpu.r[6] as u16 as i16 as i32) as u32; // exts.w r6,r3
            cpu.r[0] = mem.read_u8(cpu.r[5]).unwrap() as i8 as i32 as u32; // mov.b @r5,r0
            cpu.r[2] = shld(r3 as i32, cpu.r[2]); // shld r3,r2
            let _ = mem.write_u8(cpu.r[5], cpu.r[2] as u8); // mov.b r2,@r5
            cpu.r[0] &= cpu.r[1]; // and r1,r0
            cpu.r[3] = mem.read_u8(cpu.r[5]).unwrap() as i8 as i32 as u32; // mov.b @r5,r3
            cpu.r[3] |= cpu.r[7]; // or r7,r3
            let _ = mem.write_u8(cpu.r[5], cpu.r[3] as u8); // mov.b r3,@r5
            cpu.r[5] = cpu.r[5].wrapping_sub(1); // add #-1,r5
            let t2 = cpu.r[5] >= cpu.r[4]; // cmp/hs r4,r5
            cpu.r[7] = cpu.r[0] & 0xFF; // extu.b r0,r7
            cpu.r[3] = (n as i32).wrapping_neg() as u32; // neg r14,r3
            cpu.r[7] = shld(cpu.r[3] as i32, cpu.r[7]); // bt.s delay slot: shld r3,r7
            if !t2 {
                break; // bt.s not taken
            }
        }
    }
    cpu.r[7] &= 0xFF; // extu.b r7,r7
    cpu.set_t(cpu.r[7] == 0); // tst r7,r7
    cpu.r[0] = if cpu.t() { 0 } else { 1 }; // bt 0x8c014c60 / mov #1,r0
    // rts ; mov.l @r15+,r14
}
