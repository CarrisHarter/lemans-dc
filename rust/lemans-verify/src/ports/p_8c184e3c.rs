//! 0x8c184e3c - release every 88-byte record in `[r4+8]`, then free the array.
//!
//! `end = base + low32(88 * [r4+4])` (signed `mul.l`). The plain `bf` has no delay
//! slot, so `r11 = -1` only appears when the walk runs. Each record goes to
//! 0x8c1842f0; the array itself is freed afterwards (reloaded from `[r14+8]`).

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

    cpu.r[14] = cpu.r[4]; // mov r4,r14
    cpu.r[12] = mem.read_u32(cpu.r[14].wrapping_add(8)).unwrap_or(0); // mov.l @(8,r14),r12
    cpu.set_t(cpu.r[12] == 0); // tst r12,r12
    if cpu.r[12] != 0 {
        cpu.r[1] = mem.read_u32(cpu.r[14].wrapping_add(4)).unwrap_or(0); // mov.l @(4,r14),r1
        cpu.r[0] = 88; // mov #88,r0
        cpu.r[13] = cpu.r[12]; // mov r12,r13
        let product = (cpu.r[0] as i32 as i64) * (cpu.r[1] as i32 as i64); // mul.l r0,r1
        cpu.mach = (product >> 32) as u32;
        cpu.macl = product as u32;
        cpu.r[1] = cpu.macl; // sts macl,r1
        cpu.r[13] = cpu.r[13].wrapping_add(cpu.r[1]); // add r1,r13
        cpu.set_t(cpu.r[13] > cpu.r[12]); // cmp/hi r12,r13
        if cpu.t() {
            cpu.r[11] = 0xffff_ffff; // mov #-1,r11
            loop {
                cpu.r[0] = mem.read_u32(0x8c184e80).unwrap_or(0x8c1842f0); // mov.l 0x8c184e80,r0
                cpu.r[5] = cpu.r[11]; // mov r11,r5
                cpu.r[4] = cpu.r[12]; // jsr delay: mov r12,r4
                cpu.pr = 0x8c184e68;
                crate::registry::p_8c1842f0::run(cpu, mem);
                cpu.r[12] = cpu.r[12].wrapping_add(88); // add #88,r12
                cpu.set_t(cpu.r[13] > cpu.r[12]); // cmp/hi r12,r13
                if !cpu.t() {
                    break;
                }
            }
        }
        cpu.r[0] = mem.read_u32(0x8c184e84).unwrap_or(0x8c01032c); // mov.l 0x8c184e84,r0
        cpu.r[4] = mem.read_u32(cpu.r[14].wrapping_add(8)).unwrap_or(0); // jsr delay: mov.l @(8,r14),r4
        cpu.pr = 0x8c184e74;
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
