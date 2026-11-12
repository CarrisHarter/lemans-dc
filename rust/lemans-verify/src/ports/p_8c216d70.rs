//! 0x8c216d70 - length-bounded case-insensitive compare via 0x8c216d44.
//!
//! With `n = r6 <= 0` the function returns 1 immediately. Otherwise it walks both
//! strings while `n > 0`: a zero byte in `r4` ends the walk (returning 1 only if
//! the other string also ended), and a folded-byte mismatch returns 0. Running
//! out of `n` also returns 1.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[14]);
    cpu.r[14] = cpu.r[6]; // mov r6,r14
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[13]);
    cpu.set_t((cpu.r[14] as i32) > 0); // cmp/pl r14
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[12]);
    cpu.r[13] = cpu.r[4]; // mov r4,r13
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[8]);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr);
    cpu.r[12] = cpu.r[5]; // bf.s delay: mov r5,r12

    let mut result = 1u32;
    if cpu.t() {
        loop {
            cpu.r[2] = mem.read_u8(cpu.r[13]).unwrap_or(0) as u32; // mov.b @r13,r2
            cpu.set_t(cpu.r[2] == 0); // tst r2,r2
            if cpu.t() {
                cpu.r[2] = mem.read_u8(cpu.r[12]).unwrap_or(0) as u32; // mov.b @r12,r2
                cpu.set_t(cpu.r[2] == 0); // tst r2,r2
                if cpu.t() {
                    result = 1; // at 0x8c216dae
                } else {
                    result = 0; // bra delay at 0x8c216da6
                }
                break;
            }
            cpu.r[4] = mem.read_u8(cpu.r[13]).unwrap_or(0) as u32; // bsr delay: mov.b @r13+,r4
            cpu.r[13] = cpu.r[13].wrapping_add(1);
            cpu.pr = 0x8c216d8e;
            crate::registry::p_8c216d44::run(cpu, mem);
            cpu.r[8] = cpu.r[0] as i8 as i32 as u32; // exts.b r0,r8
            cpu.r[4] = mem.read_u8(cpu.r[12]).unwrap_or(0) as u32; // bsr delay: mov.b @r12+,r4
            cpu.r[12] = cpu.r[12].wrapping_add(1);
            cpu.pr = 0x8c216d96;
            crate::registry::p_8c216d44::run(cpu, mem);
            cpu.r[0] = cpu.r[0] as i8 as i32 as u32; // exts.b r0,r0
            cpu.set_t(cpu.r[0] == cpu.r[8]); // cmp/eq r0,r8
            if !cpu.t() {
                result = 0; // bra delay at 0x8c216da6
                break;
            }
            cpu.r[14] = cpu.r[14].wrapping_sub(1); // add #-1,r14
            cpu.set_t((cpu.r[14] as i32) > 0); // cmp/pl r14
            if !cpu.t() {
                result = 1; // falls to 0x8c216dae
                break;
            }
        }
    }
    cpu.r[0] = result;
    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[8] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[12] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[13] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[14] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}
