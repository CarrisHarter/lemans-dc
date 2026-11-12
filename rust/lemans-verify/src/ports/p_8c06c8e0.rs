//! 0x8c06c8e0 — retire a node: unlink it from the busy-list head at
//! 0x8c23f438+0xc, push it onto the free-list head at +0x10, decrement the
//! live counter at +0x14 and drop it from the pair table.
//!
//! ```text
//! push r14; push pr
//! r14 = r4
//! 0x8c06be1c(r4 = node, r5 = G+0xc)   ; jsr -> pr = 0x8c06c8ee
//! 0x8c06be50(r4 = node, r5 = G+0x10)  ; jsr -> pr = 0x8c06c8f8
//! r1 = [G+0x14] - 1
//! 0x8c06c244(r4 = G, r5 = node)       ; jsr -> pr = 0x8c06c906
//!                                     ; delay slot stores the new counter
//! pop pr; rts; pop r14
//! ```
//!
//! The two `mov.l 0x8c06c90c,r5` loads read the literal-pool copy of G
//! (0x8c23f438), so the port reads it too. The counter decrement is ordered
//! before the third call, matching the `jsr` delay slot at 0x8c06c904.
//! r4/r5/r14/pr are left as the original sequence leaves them.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    // 0x8c06c8e0..0x8c06c8e2: save r14 and pr.
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[14]);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr);

    // 0x8c06c8e4..0x8c06c8e6: G in r5, node kept in r14.
    let base = mem.read_u32(0x8c06c90c).unwrap_or(0); // 0x8c23f438
    cpu.r[14] = cpu.r[4];

    // 0x8c06c8e8..0x8c06c8ec: unlink the node from the busy list (head G+0xc).
    cpu.r[5] = base.wrapping_add(12);
    cpu.pr = 0x8c06c8ee;
    crate::registry::p_8c06be1c::run(cpu, mem);

    // 0x8c06c8ee..0x8c06c8f6: push it onto the free list (head G+0x10).
    cpu.r[4] = cpu.r[14];
    cpu.r[5] = base.wrapping_add(16);
    cpu.pr = 0x8c06c8f8;
    crate::registry::p_8c06be50::run(cpu, mem);

    // 0x8c06c8f8..0x8c06c904: live counter--, then drop the node from the
    // pair table. The store is the jsr delay slot, so it precedes the callee.
    cpu.r[4] = base;
    cpu.r[5] = cpu.r[14];
    let counter = mem.read_u32(base.wrapping_add(20)).unwrap_or(0);
    cpu.r[1] = counter.wrapping_sub(1);
    cpu.pr = 0x8c06c906;
    let _ = mem.write_u32(base.wrapping_add(20), cpu.r[1]);
    crate::registry::p_8c06c244::run(cpu, mem);

    // 0x8c06c906..0x8c06c90a: restore pr and r14.
    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[14] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}
