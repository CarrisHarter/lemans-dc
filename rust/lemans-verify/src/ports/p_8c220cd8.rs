//! 0x8c220cd8 — raise IMASK to 15, store the word at `[0x8c23cc68]` into the
//! indexed table, then restore the caller's IMASK nibble.
//!
//! The old interrupt level is saved on the stack, SR is masked with 0xffffff0f
//! and OR-ed with 0xf0, `set_indexed_word_if_in_range(*(r2), 0)` (0x8c21a04a)
//! runs with interrupts fully masked, and the final SR keeps everything the
//! callee left except the I3..I0 bits, which come from the saved nibble.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr); // sts.l pr,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4); // add #-4,r15
    cpu.r[0] = cpu.sr; // stc sr,r0
    cpu.r[3] = mem.read_u16(0x8c220d66).unwrap_or(0) as i16 as i32 as u32; // 0xff0f
    cpu.r[0] >>= 2; // shlr2
    cpu.r[0] >>= 2; // shlr2
    cpu.r[0] &= 15;
    let _ = mem.write_u32(cpu.r[15], cpu.r[0]); // mov.l r0,@r15
    cpu.r[0] = cpu.sr; // stc sr,r0
    cpu.r[0] &= cpu.r[3];
    cpu.r[0] |= 240;
    cpu.sr = cpu.r[0]; // ldc r0,sr
    cpu.r[3] = 0x8c21a04a;
    cpu.r[5] = 0;
    cpu.r[2] = 0x8c23cc68;
    cpu.r[4] = mem.read_u32(cpu.r[2]).unwrap_or(0); // delay slot of jsr @r3
    crate::registry::p_8c21a04a::run(cpu, mem);
    cpu.r[0] = mem.read_u32(cpu.r[15]).unwrap_or(0); // mov.l @r15,r0
    cpu.r[1] = cpu.sr; // stc sr,r1
    cpu.r[3] = mem.read_u16(0x8c220d66).unwrap_or(0) as i16 as i32 as u32; // 0xff0f
    cpu.r[0] &= 15;
    cpu.r[0] <<= 2; // shll2
    cpu.r[1] &= cpu.r[3];
    cpu.r[0] <<= 2; // shll2
    cpu.r[0] |= cpu.r[1];
    cpu.sr = cpu.r[0]; // ldc r0,sr
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0); // lds.l @r15+,pr
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}
