//! 0x8c173580 — publish [0x8c2b52f8] at [r4], run the 0x8c172662 clearer, return 1.
//!
//! ```text
//! mov.l pool,r2      ; r2 = 0x8c2b52f8
//! sts.l pr,@-r15     ; spill pr
//! mov.l pool,r1      ; r1 = 0x8c172662
//! mov.l @r2,r3       ; r3 = [0x8c2b52f8]
//! jsr @r1            ; pr = 0x8c17358c
//! mov.l r3,@r4       ; (delay slot) [r4] = r3 — runs BEFORE the call
//! lds.l @r15+,pr     ; restore pr
//! rts
//! mov #1,r0          ; (delay slot) return 1
//! ```
//!
//! The callee zeroes 0x100 bytes at 0x8c2b5418, clears [0x8c2b5518], and
//! clobbers r4/r0/r3/r5/r6/r7 plus T (via its tail-called memset); r1/r2 keep
//! the pool values. The pr spill is in the excluded stack window, so r15 is
//! left untouched and pr is saved/restored through a local.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[2] = mem.read_u32(0x8c1735e8).unwrap_or(0x8c2b52f8);
    let saved_pr = cpu.pr; // sts.l pr,@-r15
    cpu.r[1] = mem.read_u32(0x8c1735e4).unwrap_or(0x8c172662);
    cpu.r[3] = mem.read_u32(cpu.r[2]).unwrap_or(0);
    let _ = mem.write_u32(cpu.r[4], cpu.r[3]); // jsr delay slot: [r4] before the call
    cpu.pr = 0x8c17358c; // jsr @r1
    crate::registry::p_8c172662::run(cpu, mem);
    cpu.pr = saved_pr; // lds.l @r15+,pr
    cpu.r[0] = 1; // rts delay slot
}
