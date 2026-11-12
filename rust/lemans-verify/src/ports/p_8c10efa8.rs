//! 0x8c10efa8 — NUL-terminate a 16-byte field after a bounded copy.
//!
//! `r4` is saved in `r14`, then strncpy (`0x8c011344`) is called with `r4`/`r5`
//! untouched and `r6 = 15` loaded in the `jsr` delay slot. After the call the
//! byte at `r14+15` is cleared and `r14` is returned in `r0`. The prologue only
//! pushes r14/pr so the epilogue restores both to their entry values; T is
//! left set by the callee's terminating `cmp/hs`, and the remaining
//! instructions are flag-neutral. The call clobbers r0/r1/r3/r5/r7.

use sh4_oracle::{Cpu, Memory};

use super::p_8c011344;

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    // mov.l r14,@-r15 ; sts.l pr,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[14]);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr);
    cpu.r[14] = cpu.r[4]; // mov r4,r14
    cpu.r[0] = 0x8c011344; // mov.l 0x8c10efc0,r0
    cpu.pr = 0x8c10efb4; // jsr @r0
    cpu.r[6] = 15; // delay slot: mov #15,r6
    p_8c011344::run(cpu, mem);
    cpu.r[0] = 0; // mov #0,r0
    let _ = mem.write_u8(cpu.r[14].wrapping_add(15), 0); // mov.b r0,@(15,r14)
    cpu.r[0] = cpu.r[14]; // mov r14,r0
    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0); // lds.l @r15+,pr
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[14] = mem.read_u32(cpu.r[15]).unwrap_or(0); // delay slot of `rts`
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}
