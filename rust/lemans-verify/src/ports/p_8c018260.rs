//! 0x8c018260 — acquire the lock at `*(u32 *)0x8c1f2f98 + 8`.
//!
//! ```text
//! 0x8c018260  sts.l pr,@-r15      ; save return address
//! 0x8c018262  mov.l 0x8c018280,r3 ; r3 = 0x8c1f2f98 (pool word)
//! 0x8c018264  mov.l @r3,r4        ; r4 = global pointer
//! 0x8c018266  add   #8,r4         ; r4 += 8
//! 0x8c018268  mov.l 0x8c018284,r2 ; r2 = 0x8c018484
//! 0x8c01826a  jsr   @r2           ; call the tas.b spinlock
//! 0x8c01826c  nop                 ; delay slot
//! 0x8c01826e  lds.l @r15+,pr      ; restore return address
//! 0x8c018270  rts
//! 0x8c018272  nop                 ; delay slot
//! ```
//!
//! The callee (0x8c018484) sets T and stores one byte `old | 0x80`; it leaves
//! r0-r15 untouched, so r2/r3/r4 keep the values this routine loaded. The
//! literal words are read from memory rather than hard-coded, exactly as the
//! original PC-relative loads do, so a spec may redirect the pointer (or the
//! register-side effect) by patching the pool.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    // sts.l pr,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr);
    // mov.l 0x8c018280,r3 ; mov.l @r3,r4 ; add #8,r4
    cpu.r[3] = mem.read_u32(0x8c018280).unwrap_or(0);
    cpu.r[4] = mem.read_u32(cpu.r[3]).unwrap_or(0).wrapping_add(8);
    // mov.l 0x8c018284,r2 ; jsr @r2 / nop delay slot
    cpu.r[2] = mem.read_u32(0x8c018284).unwrap_or(0);
    cpu.pr = 0x8c01826e; // return address past the delay slot
    crate::registry::p_8c018484::run(cpu, mem);
    // lds.l @r15+,pr ; rts / nop delay slot
    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}
