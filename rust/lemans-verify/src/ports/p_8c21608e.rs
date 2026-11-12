//! 0x8c21608e - validate a save block's checksum, then blank it.
//!
//! Scans 60 bytes at `r5+2` for 0xff; any other byte returns 0. When all are
//! 0xff, 0x8c215f6c folds 62 bytes at `r5` and the result must equal the word at
//! `[r5+62]`. On a match the 64 bytes at `r5` are filled with 0xff and the
//! function returns 1.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[14]);
    cpu.r[4] = 60; // mov #60,r4
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[13]);
    cpu.r[13] = cpu.r[5]; // mov r5,r13
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr);
    cpu.r[14] = mem.read_u16(0x8c216158).unwrap_or(0xff) as u32; // mov.w 0x8c216158,r14
    cpu.r[5] = cpu.r[5].wrapping_add(2); // add #2,r5

    let mut result = 0u32;
    let mut all_ff = true;
    loop {
        cpu.r[3] = mem.read_u8(cpu.r[5]).unwrap_or(0) as u32; // mov.b @r5+,r3
        cpu.r[5] = cpu.r[5].wrapping_add(1);
        cpu.r[3] &= 0xff; // extu.b r3,r3
        cpu.set_t(cpu.r[3] == cpu.r[14]); // cmp/eq r14,r3
        if !cpu.t() {
            all_ff = false; // bf 0x8c2160c0
            break;
        }
        cpu.r[4] = cpu.r[4].wrapping_sub(1); // add #-1,r4
        cpu.set_t((cpu.r[4] as i32) > 0); // cmp/pl r4
        if !cpu.t() {
            break;
        }
    }
    if all_ff {
        cpu.r[3] = mem.read_u32(0x8c21615c).unwrap_or(0x8c215f6c); // mov.l 0x8c21615c,r3
        cpu.r[5] = cpu.r[13]; // mov r13,r5
        cpu.r[4] = 62; // jsr delay: mov #62,r4
        cpu.pr = 0x8c2160b2;
        crate::registry::p_8c215f6c::run(cpu, mem);
        cpu.r[4] = cpu.r[0]; // mov r0,r4
        cpu.r[0] = 62; // mov #62,r0
        cpu.r[2] = mem.read_u16(cpu.r[13].wrapping_add(cpu.r[0])).unwrap_or(0) as u32; // mov.w @(r0,r13),r2
        cpu.r[4] &= 0xffff; // extu.w r4,r4
        cpu.r[2] &= 0xffff; // extu.w r2,r2
        cpu.set_t(cpu.r[4] == cpu.r[2]); // cmp/eq r4,r2
        if cpu.t() {
            cpu.r[5] = cpu.r[13]; // mov r13,r5
            cpu.r[4] = 64; // bra delay: mov #64,r4
            loop {
                let _ = mem.write_u8(cpu.r[5], cpu.r[14] as u8); // mov.b r14,@r5
                cpu.r[4] = cpu.r[4].wrapping_sub(1); // add #-1,r4
                cpu.r[5] = cpu.r[5].wrapping_add(1); // add #1,r5
                cpu.set_t(cpu.r[4] == 0); // tst r4,r4
                if cpu.t() {
                    break;
                }
            }
            result = 1; // mov #1,r0
        }
    }
    cpu.r[0] = result;
    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[13] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[14] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}
