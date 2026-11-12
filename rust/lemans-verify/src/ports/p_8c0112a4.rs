//! 0x8c0112a4 — strcpy: copy r5 (incl. NUL) onto the end of r4.
//!
//! The `bra` delay slot copies src into r7. Skip loop scans dst to its NUL
//! in r6; copy loop then stores through r6 with the `bf.s` delay slot
//! advancing r6 even on the final NUL. Final: r0 = r4 (rts delay slot),
//! r2 = 0 (extu.b of the NUL), r6/r7 one past the copied NUL, T set.
//! `mov.b @r7+,r2` is a byte load (sign-extended); `mov.b r2,@r6` a byte
//! store. Reading a dword here only matched while the trailing bytes happened
//! to be mapped — at the end of a region the read fails and yields a bogus NUL.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let dst = cpu.r[4];
    let src = cpu.r[5];
    let mut r6 = dst;
    let mut r7 = src; // delay slot of `bra`
    // skip to end of dst
    loop {
        let c = mem.read_u8(r6).unwrap_or(0) as i8 as i32 as u32; // mov.b @r6,r2
        cpu.r[2] = c;
        if c == 0 {
            cpu.set_t(true);
            break; // bf not taken
        }
        r6 = r6.wrapping_add(1); // 0x8c0112aa
    }
    // copy loop
    loop {
        let c = mem.read_u8(r7).unwrap_or(0) as i8 as i32 as u32; // mov.b @r7+,r2
        r7 = r7.wrapping_add(1);
        cpu.r[2] = c;
        let _ = mem.write_u8(r6, c as u8);
        let z = c as u8 as u32; // extu.b r2,r2
        cpu.r[2] = z;
        cpu.set_t(z == 0);
        r6 = r6.wrapping_add(1); // delay slot of `bf.s`, runs even on exit
        if z == 0 {
            break;
        }
    }
    cpu.r[6] = r6;
    cpu.r[7] = r7;
    cpu.r[0] = dst; // delay slot of `rts`
}
