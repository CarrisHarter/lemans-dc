//! 0x8c011304 — strncmp: compare up to n bytes of r4 vs r5, byte at a time.
//!
//! The `bf.s` delay slot at 0x8c011308 copies r5 into r7 on every entry,
//! including the n==0 path (which returns 0 through the rts delay slot, T
//! set). On the n>0 paths each `mov.b @Rn+,Rm` loads one sign-extended byte
//! and steps by 1; the loop leaves r4 past the last compared byte, r7 one
//! before it (decremented at 0x8c011330), r2 holding either the mismatched
//! src byte or the address r4-1, r5 the count, and r3 the zero-extended src
//! byte. Return r0 = (u8)s1[k-1] - (u8)s2[k-1]; T reflects the last test:
//! clear on mismatch, set on NUL or count exit.

use sh4_oracle::{Cpu, Memory};

fn sext8(mem: &Memory, addr: u32) -> u32 {
    mem.read_u8(addr).unwrap_or(0) as i8 as i32 as u32
}

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let s1 = cpu.r[4];
    let s2 = cpu.r[5];
    let n = cpu.r[6];
    cpu.r[7] = s2;      // delay slot of `bf.s` always runs
    cpu.set_t(n == 0);  // tst r6,r6
    if n == 0 {
        cpu.r[0] = 0;   // delay slot of rts (0x8c01130c)
        return;
    }
    let mut r4 = s1;
    let mut r7 = s2;
    let mut r5 = 0u32;
    let mut r2: u32;
    let t: bool;
    loop {
        r2 = sext8(mem, r7);          // mov.b @r7+,r2 (one byte, step 1)
        r7 = r7.wrapping_add(1);
        let r3 = sext8(mem, r4);      // mov.b @r4+,r3 (one byte, step 1)
        r4 = r4.wrapping_add(1);
        if r2 != r3 {
            t = false;                // cmp/eq -> bf taken
            break;
        }
        r2 = r4.wrapping_sub(1);      // mov r4,r2 ; add #-1,r2
        if sext8(mem, r2) == 0 {      // mov.b @r2,r3 ; tst r3,r3
            t = true;                 // bt taken
            break;
        }
        r5 = r5.wrapping_add(1);
        if r5 >= n {
            t = true;                 // cmp/hs r6,r5 -> bf not taken
            break;
        }
    }
    cpu.set_t(t);
    cpu.r[2] = r2;
    cpu.r[4] = r4;
    cpu.r[5] = r5;
    let r7f = r7.wrapping_sub(1);
    cpu.r[7] = r7f;
    let a = mem.read_u8(r7f).unwrap_or(0) as u32;        // extu.b r3,r3
    let b = mem.read_u8(r4.wrapping_sub(1)).unwrap_or(0) as u32; // extu.b r0,r0
    cpu.r[3] = a;
    cpu.r[0] = b.wrapping_sub(a); // sub r3,r0
}
