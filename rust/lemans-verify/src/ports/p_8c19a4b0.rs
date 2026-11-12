//! 0x8c19a4b0 - release every 0xe4-byte record in `[r4+8]`, then zero the count.
//!
//! `end = base + low32(0xe4 * [r4+4])` (signed `mul.l`). The plain `bf` has no
//! delay slot, so `r11 = -1` only appears when the walk runs. Each record goes to
//! 0x8c199dd0; `[r4+4] = 0` afterwards and r0 = 0.

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
    cpu.r[1] = mem.read_u32(cpu.r[14].wrapping_add(4)).unwrap_or(0); // mov.l @(4,r14),r1
    cpu.r[0] = mem.read_u32(0x8c19a4f0).unwrap_or(0xe4); // mov.l 0x8c19a4f0,r0
    cpu.r[13] = mem.read_u32(cpu.r[14].wrapping_add(8)).unwrap_or(0); // mov.l @(8,r14),r13
    let product = (cpu.r[0] as i32 as i64) * (cpu.r[1] as i32 as i64); // mul.l r0,r1
    cpu.mach = (product >> 32) as u32;
    cpu.macl = product as u32;
    cpu.r[1] = cpu.macl; // sts macl,r1
    cpu.r[12] = cpu.r[13]; // mov r13,r12
    cpu.r[12] = cpu.r[12].wrapping_add(cpu.r[1]); // add r1,r12
    cpu.set_t(cpu.r[12] > cpu.r[13]); // cmp/hi r13,r12
    if cpu.t() {
        cpu.r[11] = 0xffff_ffff; // mov #-1,r11
        loop {
            cpu.r[0] = mem.read_u32(0x8c19a4f4).unwrap_or(0x8c199dd0); // mov.l 0x8c19a4f4,r0
            cpu.r[5] = cpu.r[11]; // mov r11,r5
            cpu.r[4] = cpu.r[13]; // jsr delay: mov r13,r4
            cpu.pr = 0x8c19a4d8;
            crate::registry::p_8c199dd0::run(cpu, mem);
            cpu.r[0] = mem.read_u32(0x8c19a4f0).unwrap_or(0xe4); // mov.l 0x8c19a4f0,r0
            cpu.r[13] = cpu.r[13].wrapping_add(cpu.r[0]); // add r0,r13
            cpu.set_t(cpu.r[12] > cpu.r[13]); // cmp/hi r13,r12
            if !cpu.t() {
                break;
            }
        }
    }
    cpu.r[0] = 0; // mov #0,r0
    let _ = mem.write_u32(cpu.r[14].wrapping_add(4), cpu.r[0]); // mov.l r0,@(4,r14)
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
