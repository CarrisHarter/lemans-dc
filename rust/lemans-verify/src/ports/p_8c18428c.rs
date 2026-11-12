//! 0x8c18428c - scan 88-byte records for one whose name contains `r5`.
//!
//! `base = [r10+0x17c]`, `end = base + 88*[r10+0x178]`. For each record,
//! `r1 = [r14+84]` and 0x8c17fc44 runs on `(r5, [r1+12])`. A non-zero result
//! returns `r14`; otherwise the cursor advances by 88. Falls out with 0.

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
    let _ = mem.write_u32(cpu.r[15], cpu.r[10]);
    cpu.r[11] = cpu.r[5]; // mov r5,r11
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr);
    cpu.r[10] = cpu.r[4]; // mov r4,r10
    cpu.r[0] = mem.read_u32(0x8c1842e4).unwrap_or(0x17c); // mov.l 0x8c1842e4,r0
    cpu.r[13] = mem.read_u32(0x8c1842e8).unwrap_or(0x178); // mov.l 0x8c1842e8,r13
    cpu.r[12] = cpu.r[0]; // mov r0,r12
    cpu.r[14] = mem.read_u32(cpu.r[10].wrapping_add(cpu.r[0])).unwrap_or(0); // bra delay: mov.l @(r0,r10),r14
    let mut result = 0u32;
    loop {
        cpu.r[0] = cpu.r[13]; // mov r13,r0
        cpu.r[1] = mem.read_u32(cpu.r[10].wrapping_add(cpu.r[0])).unwrap_or(0); // mov.l @(r0,r10),r1
        cpu.r[0] = 88; // mov #88,r0
        let product = (cpu.r[0] as i32 as i64) * (cpu.r[1] as i32 as i64); // mul.l r0,r1
        cpu.mach = (product >> 32) as u32;
        cpu.macl = product as u32;
        cpu.r[1] = cpu.macl; // sts macl,r1
        cpu.r[0] = cpu.r[12]; // mov r12,r0
        cpu.r[0] = mem.read_u32(cpu.r[10].wrapping_add(cpu.r[0])).unwrap_or(0); // mov.l @(r0,r10),r0
        cpu.r[0] = cpu.r[0].wrapping_add(cpu.r[1]); // add r1,r0
        cpu.set_t(cpu.r[0] > cpu.r[14]); // cmp/hi r14,r0
        if !cpu.t() {
            break;
        }
        cpu.r[0] = cpu.r[14].wrapping_add(84); // mov r14,r0 ; add #84,r0
        cpu.r[1] = mem.read_u32(cpu.r[0]).unwrap_or(0); // mov.l @r0,r1
        cpu.r[5] = cpu.r[11]; // mov r11,r5
        cpu.r[0] = mem.read_u32(0x8c1842ec).unwrap_or(0x8c17fc44); // mov.l 0x8c1842ec,r0
        cpu.r[4] = mem.read_u32(cpu.r[1].wrapping_add(12)).unwrap_or(0); // jsr delay: mov.l @(12,r1),r4
        cpu.pr = 0x8c1842b4;
        crate::registry::p_8c17fc44::run(cpu, mem);
        cpu.r[0] &= 0xff; // extu.b r0,r0
        cpu.set_t(cpu.r[0] == 0); // tst r0,r0
        if !cpu.t() {
            result = cpu.r[14]; // bra delay: mov r14,r0
            break;
        }
        cpu.r[14] = cpu.r[14].wrapping_add(88); // add #88,r14
    }
    cpu.r[0] = result; // 0 when the loop falls through
    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[10] = mem.read_u32(cpu.r[15]).unwrap_or(0);
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
