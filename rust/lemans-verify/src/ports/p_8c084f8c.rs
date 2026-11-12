//! 0x8c084f8c — copy r5 32-byte records from r6 to r4 (stride 32 on r4 only).
//!
//! Original side effects the C does not show: r8..r14 are pushed and popped
//! so they end unchanged; r0 = the halfword loaded from src+28 of the last
//! copied record, r1 = src+4, r3 = src+20, r7 = dst+20 (last iteration that
//! had r4 != 0), fr0 = the float from src+24, r4 = r4+32*count, r5 = 0 and
//! T = 1 (final tst r5,r5). The `add #32,r4` is the bf.s delay slot.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.set_t(cpu.r[5] == 0); // tst r5,r5
    while cpu.r[5] != 0 {
        let src = cpu.r[6];
        let dst = cpu.r[4];
        cpu.r[1] = src + 4; // mov r6,r1 ; add #4,r1
        cpu.r[3] = src + 20; // mov r6,r3 ; add #20,r3
        let s8 = src + 8;
        let s12 = src + 12;
        let s16 = src + 16;
        let s24 = src + 24;
        cpu.set_t(dst == 0); // tst r4,r4
        if dst != 0 {
            let b = mem.read_u8(src).unwrap_or(0);
            let _ = mem.write_u8(dst, b); // mov.b r7,@r4 (r7 = extu.b)
            let mut r0 = dst;
            let mut r9 = dst;
            let mut r7 = dst;
            let mut r11 = dst;
            let mut r13 = dst;
            cpu.fr[0] = mem.read_f32(cpu.r[1]).unwrap_or(0.0);
            r0 = dst + 4;
            let _ = mem.write_f32(r0, cpu.fr[0]);
            r7 = dst;
            cpu.fr[0] = mem.read_f32(s8).unwrap_or(0.0);
            r9 = dst + 8;
            let _ = mem.write_f32(r9, cpu.fr[0]);
            r11 = dst + 12;
            cpu.fr[0] = mem.read_f32(s12).unwrap_or(0.0);
            r13 = dst + 16;
            r7 = dst + 20;
            let _ = mem.write_f32(r11, cpu.fr[0]);
            r0 = dst + 24;
            cpu.fr[0] = mem.read_f32(s16).unwrap_or(0.0);
            let _ = mem.write_f32(r13, cpu.fr[0]);
            cpu.fr[0] = mem.read_f32(cpu.r[3]).unwrap_or(0.0);
            let _ = mem.write_f32(r7, cpu.fr[0]);
            cpu.fr[0] = mem.read_f32(s24).unwrap_or(0.0);
            let _ = mem.write_f32(r0, cpu.fr[0]);
            let hw = mem.read_u16(src + 28).unwrap_or(0);
            cpu.r[0] = hw as i16 as i32 as u32; // mov.w @(28,r6),r0 (sign-extended)
            let _ = mem.write_u16(dst + 28, hw);
            cpu.r[7] = r7; // r7 = dst+20 survives (not saved/restored)
        }
        cpu.r[5] = cpu.r[5].wrapping_sub(1); // add #-1,r5
        cpu.set_t(cpu.r[5] == 0); // tst r5,r5
        cpu.r[4] = cpu.r[4].wrapping_add(32); // bf.s delay slot
    }
    // r8..r14 were pushed at entry and popped before rts: unchanged.
}
