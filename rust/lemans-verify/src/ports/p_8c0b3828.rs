//! 0x8c0b3828 — copy 9 words from `[r5]` to `[r4]` in a permuted order via a
//! stack temp at `r15-36` (add #36,r15 is the rts delay slot, so r15 is
//! unchanged on exit).
//!
//! dst+0=[src+0] dst+4=[src+12] dst+8=[src+24] dst+12=[src+4] dst+16=[src+16]
//! dst+20=[src+28] dst+24=[src+8] dst+28=[src+20] dst+32=[src+32].
//!
//! Leftovers the C does not show: fr0=[src+32], fr1=[src+12], fr2=[src+24] as
//! f32 bits; r0=[src+32], r1=[src+8], r2=r15-4, r3=r15-36, r5=src+32,
//! r6=src+28, r7=src+4.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let src = cpu.r[5];
    let dst = cpu.r[4];
    let rd = |mem: &Memory, a: u32| mem.read_u32(a).unwrap_or(0);

    let _ = mem.write_u32(dst, rd(mem, src));
    let _ = mem.write_u32(dst + 4, rd(mem, src + 12));
    let _ = mem.write_u32(dst + 8, rd(mem, src + 24));
    let _ = mem.write_u32(dst + 12, rd(mem, src + 4));
    let _ = mem.write_u32(dst + 16, rd(mem, src + 16));
    let _ = mem.write_u32(dst + 20, rd(mem, src + 28));
    let _ = mem.write_u32(dst + 24, rd(mem, src + 8));
    let _ = mem.write_u32(dst + 28, rd(mem, src + 20));
    let s32 = rd(mem, src + 32);
    let _ = mem.write_u32(dst + 32, s32);

    cpu.fr[0] = f32::from_bits(s32);
    cpu.fr[1] = f32::from_bits(rd(mem, src + 12));
    cpu.fr[2] = f32::from_bits(rd(mem, src + 24));
    cpu.r[0] = s32;
    cpu.r[1] = rd(mem, src + 8);
    cpu.r[2] = cpu.r[15].wrapping_sub(4);
    cpu.r[3] = cpu.r[15].wrapping_sub(36);
    cpu.r[5] = src.wrapping_add(32);
    cpu.r[6] = src.wrapping_add(28);
    cpu.r[7] = src.wrapping_add(4);
}
