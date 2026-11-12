//! 0x8c01113c — LCG step (`rand`): new = state*0x41c64e6d + 0x3039, return the
//! top 15 bits of the new state; the state lives at `*(u32*)0x8c011160`.
//!
//! ```
//! 113c mov.l 0x1160,r4     r4 = *(u32*)0x8c011160  (0x8c1f2eb0: state pointer)
//! 113e mov.l 0x1164,r3     r3 = *(u32*)0x8c011164  (0x41c64e6d: multiplier)
//! 1140 sts.l macl,@-r15    push MACL
//! 1142 mov.l @r4,r2        r2 = state
//! 1144 mov.w 0x115a,r1     r1 = sext16(*(u16*)0x8c01115a) = 0x00003039
//! 1146 mul.l r3,r2         MACH:MACL = sext32(r3) * sext32(r2)
//! 1148 mov.w 0x115c,r2     r2 = sext16(*(u16*)0x8c01115c) = 0x00007fff
//! 114a sts macl,r3         r3 = low word of the product
//! 114c add r1,r3           r3 = state*K + C (mod 2^32)
//! 114e mov r3,r0
//! 1150 shlr16 r0           r0 = r3 >> 16 (logical)
//! 1152 and r2,r0           r0 &= 0x7fff
//! 1154 mov.l r3,@r4        *state = new state
//! 1156 rts
//! 1158 lds.l @r15+,macl    delay slot: pop MACL
//! ```
//!
//! The decompiler renders the update as `*(int*)p * K + C`, which hides three
//! things this port keeps: `mul.l` also leaves `MACH` = the high half of the
//! *signed* product (the verifier compares it), the push/pop restores `MACL` so
//! the routine leaves it exactly as seeded, and the constants are read from the
//! image at run time with `mov.w`, i.e. sign-extended words, not zero-extended.
//! No instruction here touches T, so the entry T flag survives.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[4] = mem.read_u32(0x8c011160).unwrap_or(0); // mov.l 0x1160,r4
    cpu.r[3] = mem.read_u32(0x8c011164).unwrap_or(0); // mov.l 0x1164,r3
    cpu.r[15] = cpu.r[15].wrapping_sub(4); // sts.l macl,@-r15
    let _ = mem.write_u32(cpu.r[15], cpu.macl);
    cpu.r[2] = mem.read_u32(cpu.r[4]).unwrap_or(0); // mov.l @r4,r2
    cpu.r[1] = mem.read_u16(0x8c01115a).unwrap_or(0) as i16 as i32 as u32; // mov.w 0x115a,r1
    let p = (cpu.r[3] as i32 as i64) * (cpu.r[2] as i32 as i64); // mul.l r3,r2
    cpu.mach = (p >> 32) as u32;
    cpu.macl = p as u32;
    cpu.r[2] = mem.read_u16(0x8c01115c).unwrap_or(0) as i16 as i32 as u32; // mov.w 0x115c,r2
    cpu.r[3] = cpu.macl; // sts macl,r3
    cpu.r[3] = cpu.r[3].wrapping_add(cpu.r[1]); // add r1,r3
    cpu.r[0] = cpu.r[3]; // mov r3,r0
    cpu.r[0] >>= 16; // shlr16 r0
    cpu.r[0] &= cpu.r[2]; // and r2,r0
    let _ = mem.write_u32(cpu.r[4], cpu.r[3]); // mov.l r3,@r4
    // rts delay slot: lds.l @r15+,macl
    cpu.macl = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}
