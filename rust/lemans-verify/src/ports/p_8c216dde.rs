//! 0x8c216dde - case-insensitive string compare via 0x8c216d44.
//!
//! Each byte is post-incremented, folded by 0x8c216d44 and sign-extended before
//! `cmp/eq`. A mismatch returns 0; when both strings end together it returns 1.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[14]);
    cpu.r[14] = cpu.r[4]; // mov r4,r14
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[13]);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[8]);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr);
    cpu.r[13] = cpu.r[5]; // bra delay: mov r5,r13
    let mut result = 1u32;
    loop {
        cpu.r[3] = mem.read_u8(cpu.r[14]).unwrap_or(0) as i8 as i32 as u32; // mov.b @r14,r3
        cpu.set_t(cpu.r[3] == 0); // tst r3,r3
        if cpu.t() {
            cpu.r[1] = mem.read_u8(cpu.r[13]).unwrap_or(0) as i8 as i32 as u32; // mov.b @r13,r1
            cpu.set_t(cpu.r[1] == 0); // tst r1,r1
            if cpu.t() {
                result = 1; // at 0x8c216e0c
            } else {
                result = 0; // bra delay at 0x8c216e0a
            }
            break;
        }
        cpu.r[4] = mem.read_u8(cpu.r[14]).unwrap_or(0) as i8 as i32 as u32; // bsr delay: mov.b @r14+,r4
        cpu.r[14] = cpu.r[14].wrapping_add(1);
        cpu.pr = 0x8c216df0;
        crate::registry::p_8c216d44::run(cpu, mem);
        cpu.r[8] = cpu.r[0] as i8 as i32 as u32; // exts.b r0,r8
        cpu.r[4] = mem.read_u8(cpu.r[13]).unwrap_or(0) as i8 as i32 as u32; // bsr delay: mov.b @r13+,r4
        cpu.r[13] = cpu.r[13].wrapping_add(1);
        cpu.pr = 0x8c216df8;
        crate::registry::p_8c216d44::run(cpu, mem);
        cpu.r[0] = cpu.r[0] as i8 as i32 as u32; // exts.b r0,r0
        cpu.set_t(cpu.r[0] == cpu.r[8]); // cmp/eq r0,r8
        if !cpu.t() {
            result = 0; // bra delay at 0x8c216e0a
            break;
        }
    }
    cpu.r[0] = result;
    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[8] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[13] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[14] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}
