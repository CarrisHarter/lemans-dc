//! 0x8c1a49ec - copy `r5` into a 10-byte space-padded slot in a 11-byte table.
//!
//! `idx = 0x8c1a4a68()`; the work only happens for `16 <= idx < 32`. Then
//! `len = min(strlen(r5), 10)`, `base = 0x8c2111e8 + 11*idx`, `memset(base, ' ', 10)`,
//! `memcpy(base, r5, len)` and `base[10] = 0`.

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
    cpu.r[12] = cpu.r[5]; // mov r5,r12
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr);
    cpu.r[0] = mem.read_u32(0x8c1a4a54).unwrap_or(0x8c1a4a68); // mov.l 0x8c1a4a54,r0
    cpu.pr = 0x8c1a49fe; // jsr delay is nop
    crate::registry::p_8c1a4a68::run(cpu, mem);

    cpu.r[1] = 16; // mov #16,r1
    cpu.set_t((cpu.r[0] as i32) >= (cpu.r[1] as i32)); // cmp/ge r1,r0
    cpu.r[14] = cpu.r[0]; // bf.s delay: mov r0,r14
    if cpu.t() {
        cpu.r[1] = 32; // mov #32,r1
        cpu.set_t((cpu.r[1] as i32) > (cpu.r[0] as i32)); // cmp/gt r0,r1
        if cpu.t() {
            cpu.r[0] = mem.read_u32(0x8c1a4a58).unwrap_or(0x8c0112f4); // mov.l 0x8c1a4a58,r0
            cpu.r[4] = cpu.r[12]; // jsr delay: mov r12,r4
            cpu.pr = 0x8c1a4a12;
            crate::registry::p_8c0112f4::run(cpu, mem);
            cpu.r[1] = 10; // mov #10,r1
            cpu.set_t((cpu.r[1] as i32) > (cpu.r[0] as i32)); // cmp/gt r1,r0
            cpu.r[13] = cpu.r[0]; // bf.s delay: mov r0,r13
            if !cpu.t() {
                cpu.r[13] = cpu.r[1]; // mov r1,r13
            }
            cpu.r[0] = 11; // mov #11,r0
            let product = (cpu.r[0] as i32 as i64) * (cpu.r[14] as i32 as i64); // mul.l r0,r14
            cpu.mach = (product >> 32) as u32;
            cpu.macl = product as u32;
            cpu.r[14] = cpu.macl; // sts macl,r14
            cpu.r[1] = mem.read_u32(0x8c1a4a5c).unwrap_or(0x8c2111e8); // mov.l 0x8c1a4a5c,r1
            cpu.r[11] = cpu.r[14]; // mov r14,r11
            cpu.r[0] = mem.read_u32(0x8c1a4a60).unwrap_or(0x8c010f88); // mov.l 0x8c1a4a60,r0
            cpu.r[11] = cpu.r[11].wrapping_add(cpu.r[1]); // add r1,r11
            cpu.r[6] = 10; // mov #10,r6
            cpu.r[5] = 32; // mov #32,r5
            cpu.r[4] = cpu.r[11]; // jsr delay: mov r11,r4
            cpu.pr = 0x8c1a4a32;
            crate::registry::p_8c010f88::run(cpu, mem);
            cpu.r[0] = mem.read_u32(0x8c1a4a64).unwrap_or(0x8c010e84); // mov.l 0x8c1a4a64,r0
            cpu.r[6] = cpu.r[13]; // mov r13,r6
            cpu.r[5] = cpu.r[12]; // mov r12,r5
            cpu.r[4] = cpu.r[11]; // jsr delay: mov r11,r4
            cpu.pr = 0x8c1a4a3c;
            crate::registry::p_8c010e84::run(cpu, mem);
            cpu.r[3] = mem.read_u32(0x8c1a4a5c).unwrap_or(0x8c2111e8); // mov.l 0x8c1a4a5c,r3
            cpu.r[0] = cpu.r[14]; // mov r14,r0
            cpu.r[1] = 0; // mov #0,r1
            cpu.r[3] = cpu.r[3].wrapping_add(10); // add #10,r3
            let _ = mem.write_u8(cpu.r[0].wrapping_add(cpu.r[3]), cpu.r[1] as u8); // mov.b r1,@(r0,r3)
        }
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
