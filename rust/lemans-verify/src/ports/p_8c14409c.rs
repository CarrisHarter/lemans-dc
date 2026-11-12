//! 0x8c14409c - scan 0x98-byte records for one whose name contains `r5`.
//!
//! `end = [r12+8] + low32(0x98 * [r12+4])`. For each record, `r1 = [r14]` and
//! 0x8c17fc44 runs on `(r5, [r1+12])`. A non-zero result returns `r14 + 4`;
//! otherwise the cursor advances by 0x98. Falls out with 0.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[14]);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[13]);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[12]);
    cpu.r[13] = cpu.r[5]; // mov r5,r13
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr);
    cpu.r[12] = cpu.r[4]; // mov r4,r12
    cpu.r[14] = mem.read_u32(cpu.r[12].wrapping_add(8)).unwrap_or(0); // bra delay: mov.l @(8,r12),r14
    let mut result = 0u32;
    loop {
        cpu.r[3] = mem.read_u32(cpu.r[12].wrapping_add(4)).unwrap_or(0); // mov.l @(4,r12),r3
        cpu.r[0] = mem.read_u32(0x8c1440e8).unwrap_or(0x98); // mov.l 0x8c1440e8,r0
        cpu.r[1] = mem.read_u32(cpu.r[12].wrapping_add(8)).unwrap_or(0); // mov.l @(8,r12),r1
        let product = (cpu.r[0] as i32 as i64) * (cpu.r[3] as i32 as i64); // mul.l r0,r3
        cpu.mach = (product >> 32) as u32;
        cpu.macl = product as u32;
        cpu.r[3] = cpu.macl; // sts macl,r3
        cpu.r[1] = cpu.r[1].wrapping_add(cpu.r[3]); // add r3,r1
        cpu.set_t(cpu.r[1] > cpu.r[14]); // cmp/hi r14,r1
        if !cpu.t() {
            break;
        }
        cpu.r[1] = mem.read_u32(cpu.r[14]).unwrap_or(0); // mov.l @r14,r1
        cpu.r[4] = cpu.r[13]; // mov r13,r4
        cpu.r[0] = mem.read_u32(0x8c1440e4).unwrap_or(0x8c17fc44); // mov.l 0x8c1440e4,r0
        cpu.r[5] = mem.read_u32(cpu.r[1].wrapping_add(12)).unwrap_or(0); // jsr delay: mov.l @(12,r1),r5
        cpu.pr = 0x8c1440b6;
        crate::registry::p_8c17fc44::run(cpu, mem);
        cpu.r[0] &= 0xff; // extu.b r0,r0
        cpu.set_t(cpu.r[0] == 0); // tst r0,r0
        if !cpu.t() {
            cpu.r[14] = cpu.r[14].wrapping_add(4); // add #4,r14
            result = cpu.r[14]; // bra delay: mov r14,r0
            break;
        }
        cpu.r[0] = mem.read_u32(0x8c1440e8).unwrap_or(0x98); // mov.l 0x8c1440e8,r0
        cpu.r[14] = cpu.r[14].wrapping_add(cpu.r[0]); // add r0,r14
    }
    cpu.r[0] = result; // 0 when the loop falls through
    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[12] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[13] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[14] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}
