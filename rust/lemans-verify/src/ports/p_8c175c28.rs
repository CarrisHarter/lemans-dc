//! 0x8c175c28 - three-exit dispatch that fills a slot through 0x8c177fa0.
//!
//! `r4 == 0` emits str@0x8c1e118c with code 101 and returns 0; a non-null
//! `[r14+56]` emits str@0x8c1e11b4 with code 0x3eb and returns 0. Otherwise
//! 0x8c177fa0 runs on `[r14+44]` with the saved r5, then `[r14+60] = r5` and r0 = 1.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[14]);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr);
    cpu.r[15] = cpu.r[15].wrapping_sub(4); // add #-4,r15
    cpu.r[14] = cpu.r[4]; // mov r4,r14
    cpu.set_t(cpu.r[14] == 0); // tst r14,r14
    let _ = mem.write_u32(cpu.r[15], cpu.r[5]); // bf.s delay: mov.l r5,@r15
    if cpu.t() {
        cpu.r[2] = mem.read_u32(0x8c175cd4).unwrap_or(0x8c174564); // mov.l 0x8c175cd4,r2
        cpu.r[5] = mem.read_u32(0x8c175cd0).unwrap_or(0x8c1e118c); // mov.l 0x8c175cd0,r5
        cpu.r[4] = 101; // jsr delay: mov #101,r4
        cpu.pr = 0x8c175c3e;
        crate::registry::p_8c174564::run(cpu, mem);
        cpu.r[0] = 0; // rts delay at 0x8c175c5a
    } else {
        cpu.r[3] = mem.read_u32(cpu.r[14].wrapping_add(56)).unwrap_or(0); // mov.l @(56,r14),r3
        cpu.set_t(cpu.r[3] == 0); // tst r3,r3
        if !cpu.t() {
            cpu.r[3] = mem.read_u32(0x8c175cd4).unwrap_or(0x8c174564); // mov.l 0x8c175cd4,r3
            cpu.r[4] = mem.read_u16(0x8c175cc2).unwrap_or(0x3eb) as i16 as i32 as u32; // mov.w
            cpu.r[5] = mem.read_u32(0x8c175cd8).unwrap_or(0x8c1e11b4); // mov.l 0x8c175cd8,r5
            cpu.pr = 0x8c175c52; // jsr delay is nop
            crate::registry::p_8c174564::run(cpu, mem);
            cpu.r[0] = 0; // rts delay at 0x8c175c5a
        } else {
            cpu.r[3] = mem.read_u32(0x8c175cdc).unwrap_or(0x8c177fa0); // mov.l 0x8c175cdc,r3
            cpu.r[5] = mem.read_u32(cpu.r[15]).unwrap_or(0); // mov.l @r15,r5
            cpu.r[4] = mem.read_u32(cpu.r[14].wrapping_add(44)).unwrap_or(0); // jsr delay: mov.l @(44,r14),r4
            cpu.pr = 0x8c175c64;
            crate::registry::p_8c177fa0::run(cpu, mem);
            cpu.r[2] = mem.read_u32(cpu.r[15]).unwrap_or(0); // mov.l @r15,r2
            cpu.r[0] = 1; // mov #1,r0
            let _ = mem.write_u32(cpu.r[14].wrapping_add(60), cpu.r[2]); // mov.l r2,@(60,r14)
        }
    }
    cpu.r[15] = cpu.r[15].wrapping_add(4); // add #4,r15
    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[14] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}
