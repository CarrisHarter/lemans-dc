//! 0x8c193618 - release every 28-byte record in `[r4+8]`, then free the array.
//!
//! `end = base + ((count*4)*2 - count)*4` = `base + 28*count`, built with shifts
//! and a subtract. The plain `bf` has no delay slot, so `r12 = -1` only appears
//! when the walk runs. Each record goes to 0x8c01a710; the array is freed after.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[14]);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[13]);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[12]);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[11]);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr);

    cpu.r[11] = cpu.r[4]; // mov r4,r11
    cpu.r[13] = mem.read_u32(cpu.r[11].wrapping_add(8)).unwrap_or(0); // mov.l @(8,r11),r13
    cpu.set_t(cpu.r[13] == 0); // tst r13,r13
    if cpu.r[13] != 0 {
        cpu.r[0] = mem.read_u32(cpu.r[11].wrapping_add(4)).unwrap_or(0); // mov.l @(4,r11),r0
        cpu.r[14] = cpu.r[0]; // mov r0,r14
        cpu.r[14] = cpu.r[14].wrapping_shl(2); // shll2 r14
        cpu.set_t(cpu.r[14] >> 31 != 0); // shll r14
        cpu.r[14] = cpu.r[14].wrapping_shl(1);
        cpu.r[14] = cpu.r[14].wrapping_sub(cpu.r[0]); // sub r0,r14
        cpu.r[14] = cpu.r[14].wrapping_shl(2); // shll2 r14
        cpu.r[14] = cpu.r[14].wrapping_add(cpu.r[13]); // add r13,r14
        cpu.set_t(cpu.r[14] > cpu.r[13]); // cmp/hi r13,r14
        if cpu.t() {
            cpu.r[12] = 0xffff_ffff; // mov #-1,r12
            loop {
                cpu.r[0] = mem.read_u32(0x8c193660).unwrap_or(0x8c01a710); // mov.l 0x8c193660,r0
                cpu.r[5] = cpu.r[12]; // mov r12,r5
                cpu.r[4] = cpu.r[13]; // jsr delay: mov r13,r4
                cpu.pr = 0x8c193646;
                crate::registry::p_8c01a710::run(cpu, mem);
                cpu.r[13] = cpu.r[13].wrapping_add(28); // add #28,r13
                cpu.set_t(cpu.r[14] > cpu.r[13]); // cmp/hi r13,r14
                if !cpu.t() {
                    break;
                }
            }
        }
        cpu.r[0] = mem.read_u32(0x8c193664).unwrap_or(0x8c01032c); // mov.l 0x8c193664,r0
        cpu.r[4] = mem.read_u32(cpu.r[11].wrapping_add(8)).unwrap_or(0); // jsr delay: mov.l @(8,r11),r4
        cpu.pr = 0x8c193652;
        crate::registry::p_8c01032c::run(cpu, mem);
    }
    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[11] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[12] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[13] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[14] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}
