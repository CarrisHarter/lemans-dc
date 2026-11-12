//! 0x8c138614 - release every 8-byte slot in `[r4+8]`, then free the array.
//!
//! `end = base + ([r4+4] << 3)` (`shll2` then `shll`; the latter sets T but the
//! following `cmp/hi` overwrites it). The plain `bf` has no delay slot, so
//! `r12 = -1` is only written when the walk runs. The free reloads the base.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let mut saved = [0u32; 5];
    for (i, reg) in [14usize, 13, 12, 11].iter().enumerate() {
        saved[i] = cpu.r[*reg];
        cpu.r[15] = cpu.r[15].wrapping_sub(4);
        let _ = mem.write_u32(cpu.r[15], cpu.r[*reg]);
    }
    saved[4] = cpu.pr;
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr);

    cpu.r[11] = cpu.r[4]; // mov r4,r11
    cpu.r[13] = mem.read_u32(cpu.r[11].wrapping_add(8)).unwrap_or(0); // mov.l @(8,r11),r13
    cpu.set_t(cpu.r[13] == 0); // tst r13,r13
    if cpu.r[13] != 0 {
        cpu.r[0] = mem.read_u32(cpu.r[11].wrapping_add(4)).unwrap_or(0); // mov.l @(4,r11),r0
        cpu.r[14] = cpu.r[13]; // mov r13,r14
        cpu.r[0] = cpu.r[0].wrapping_shl(2); // shll2 r0
        cpu.set_t(cpu.r[0] >> 31 != 0); // shll r0
        cpu.r[0] = cpu.r[0].wrapping_shl(1);
        cpu.r[14] = cpu.r[14].wrapping_add(cpu.r[0]); // add r0,r14
        cpu.set_t(cpu.r[14] > cpu.r[13]); // cmp/hi r13,r14
        if cpu.t() {
            cpu.r[12] = 0xffff_ffff; // mov #-1,r12
            loop {
                cpu.r[0] = mem.read_u32(0x8c138658).unwrap_or(0x8c0d21e4); // mov.l 0x8c138658,r0
                cpu.r[5] = cpu.r[12]; // mov r12,r5
                cpu.r[4] = cpu.r[13]; // jsr delay: mov r13,r4
                cpu.pr = 0x8c13863e;
                crate::registry::p_8c0d21e4::run(cpu, mem);
                cpu.r[13] = cpu.r[13].wrapping_add(8); // add #8,r13
                cpu.set_t(cpu.r[14] > cpu.r[13]); // cmp/hi r13,r14
                if !cpu.t() {
                    break;
                }
            }
        }
        cpu.r[0] = mem.read_u32(0x8c13865c).unwrap_or(0x8c01032c); // mov.l 0x8c13865c,r0
        cpu.r[4] = mem.read_u32(cpu.r[11].wrapping_add(8)).unwrap_or(0); // jsr delay: mov.l @(8,r11),r4
        cpu.pr = 0x8c13864a;
        crate::registry::p_8c01032c::run(cpu, mem);
    }
    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    for (i, reg) in [11usize, 12, 13].iter().enumerate() {
        cpu.r[*reg] = mem.read_u32(cpu.r[15]).unwrap_or(saved[i + 1]);
        cpu.r[15] = cpu.r[15].wrapping_add(4);
    }
    cpu.r[14] = mem.read_u32(cpu.r[15]).unwrap_or(0); // rts delay: mov.l @r15+,r14
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}
