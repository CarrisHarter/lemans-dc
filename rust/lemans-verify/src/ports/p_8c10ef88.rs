//! 0x8c10ef88 — clear a 16-byte record: `strncpy(r4, *(*(r5)+0xc), 15)`
//! followed by a hard NUL at r4+15, return r4.
//!
//! r5 is a pointer to a word that points at a descriptor; the source string
//! is the descriptor's +0xc word, so two 4-byte loads feed the call. The
//! `jsr` delay slot at 0x8c10ef96 supplies the source register *after* the
//! branch is issued, i.e. the callee (strncpy, 0x8c011344) sees
//! r4 = param_1, r5 = descriptor[3], r6 = 15.
//!
//! Hidden side effects: r14 is param_1 (restored from the frame on exit),
//! r1 ends 0 because the callee clears it, r6 stays 15, and r0 is param_1
//! via `mov r14,r0`. r3/r5/r7 keep the callee's exit values. The pushes of
//! r14 and pr are modelled so store ordering matches when the caller's
//! stack lies outside the synthetic window; on the normal stack they are
//! filtered out.

use sh4_oracle::{Cpu, Memory};

use super::p_8c011344;

/// 0x8c10efa4 — literal holding the strncpy entry point.
const LIT_CALLEE: u32 = 0x8c10efa4;
/// Return address pushed by `jsr` (the instruction after the delay slot).
const RET_ADDR: u32 = 0x8c10ef98;

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    // 0x8c10ef88: mov.l r14,@-r15 ; sts.l pr,@-r15
    let sp = cpu.r[15];
    let _ = mem.write_u32(sp.wrapping_sub(4), cpu.r[14]);
    let _ = mem.write_u32(sp.wrapping_sub(8), cpu.pr);
    cpu.r[15] = sp.wrapping_sub(8);

    // 0x8c10ef8c: mov r4,r14 — the destination and return value.
    cpu.r[14] = cpu.r[4];
    // 0x8c10ef8e: mov.l @r5,r1 — descriptor pointer.
    cpu.r[1] = mem.read_u32(cpu.r[5]).unwrap_or(0);
    // 0x8c10ef90: mov #15,r6.
    cpu.r[6] = 15;
    // 0x8c10ef92: mov.l @0x8c10efa4,r0 — call target 0x8c011344.
    cpu.r[0] = mem.read_u32(LIT_CALLEE).unwrap_or(0x8c011344);
    // 0x8c10ef94: jsr @r0 — delay slot feeds r5 before the callee runs.
    cpu.pr = RET_ADDR;
    cpu.r[5] = mem.read_u32(cpu.r[1].wrapping_add(0xc)).unwrap_or(0);
    p_8c011344::run(cpu, mem);

    // 0x8c10ef98: mov #0,r0 ; 0x8c10ef9a: mov.b r0,@(15,r14)
    cpu.r[0] = 0;
    let _ = mem.write_u8(cpu.r[14].wrapping_add(15), 0);
    // 0x8c10ef9c: mov r14,r0 — the return value.
    cpu.r[0] = cpu.r[14];
    // 0x8c10ef9e: lds.l @r15+,pr ; delay slot of rts: mov.l @r15+,r14.
    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[14] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}
