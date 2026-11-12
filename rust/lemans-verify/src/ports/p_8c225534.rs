//! 0x8c225534 - index of the first zero byte in a 32-byte-stride list.
//!
//! Record is `0x8c2c02b8 + 0x148*r4`. Cursor is `[record+40]`, limit is
//! `[[record+28]+48]`. Returns the index of the first zero byte, or -1.
//! `mul.l` writes both MAC halves.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[6] = mem.read_u16(0x8c2255ea).unwrap_or(0x0148) as i16 as i32 as u32; // mov.w
    cpu.r[3] = mem.read_u32(0x8c2255f0).unwrap_or(0x8c2c02b8); // mov.l
    let product = (cpu.r[6] as i32 as i64) * (cpu.r[4] as i32 as i64); // mul.l r6,r4
    cpu.mach = (product >> 32) as u32;
    cpu.macl = product as u32;
    cpu.r[6] = cpu.macl; // sts macl,r6
    cpu.r[6] = cpu.r[6].wrapping_add(cpu.r[3]); // add r3,r6
    cpu.r[5] = mem.read_u32(cpu.r[6].wrapping_add(40)).unwrap_or(0); // mov.l @(40,r6),r5
    cpu.r[4] = 0; // bra delay: mov #0,r4
    loop {
        cpu.r[2] = mem.read_u32(cpu.r[6].wrapping_add(28)).unwrap_or(0); // mov.l @(28,r6),r2
        cpu.r[3] = mem.read_u32(cpu.r[2].wrapping_add(48)).unwrap_or(0); // mov.l @(48,r2),r3
        cpu.set_t((cpu.r[4] as i32) >= (cpu.r[3] as i32)); // cmp/ge r3,r4
        if cpu.t() {
            cpu.r[0] = 0xffff_ffff; // mov #-1,r0
            return;
        }
        cpu.r[7] = mem.read_u8(cpu.r[5]).unwrap_or(0) as i8 as i32 as u32; // mov.b @r5,r7
        cpu.r[7] &= 0xff; // extu.b r7,r7
        cpu.set_t(cpu.r[7] == 0); // tst r7,r7
        if cpu.t() {
            cpu.r[0] = cpu.r[4]; // rts delay: mov r4,r0
            return;
        }
        cpu.r[5] = cpu.r[5].wrapping_add(32); // add #32,r5
        cpu.r[4] = cpu.r[4].wrapping_add(1); // add #1,r4
    }
}
