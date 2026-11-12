//! 0x8c103604 — resolve selector then broadcast one word to six records.
//!
//! ```text
//! push r14,r13 ; push pr ; r13=r4
//! r14 = (r5 in 0..4) ? 0x8c0c0ecc([0x8c27a238], r5) : 0
//! six calls to 0x8c16eadc (indexed store [table+field]=r14):
//!   (r13+76), (r13+0xc0), (r13+0x134), (r13+0x1a8), (r13+0x21c), (r13+0x290)
//! pop pr,r13,r14 ; rts
//! ```
//!
//! Condition details: `cmp/pz r5 ; bf.s` exits to the store loop with
//! `r14=0` for negative `r5`; otherwise `cmp/gt r5,r0` with `r0=4` tests
//! `4>r5` signed, so the compute call runs only for `0<=r5<4`.
//!
//! The last call differs from the first five: `r13` itself is incremented by
//! `0x290` *before* the `jsr` delay slot copies it to `r4` (the first five
//! keep `r13` and only offset `r4`). `r13`/`r14` are callee-saved and
//! restored, so no residue from them survives.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let r13 = cpu.r[4]; // mov r4,r13
    // cmp/pz r5 ; bf.s: taken (with delay r14=0) iff r5 < 0 signed.
    // Otherwise cmp/gt r5,r0 tests 4>r5 signed. Final T equals the taken
    // branch in both exits: cmp/pz leaves 0 on the negative exit, cmp/gt
    // leaves (4>r5) on the non-negative exit, and neither callee (adds and
    // mov.l loads only) touches T afterwards.
    let take = (cpu.r[5] as i32) >= 0 && 4 > (cpu.r[5] as i32);
    let r14 = if take {
        // r0=0x8c27a238 (pool word) ; r4=[r0] ; jsr 0x8c0c0ecc ; nop ; r14=r0.
        cpu.r[0] = 0x8c27a238;
        cpu.r[4] = mem.read_u32(cpu.r[0]).unwrap_or(0);
        cpu.r[0] = 0x8c0c0ecc;
        crate::registry::p_8c0c0ecc::run(cpu, mem);
        cpu.r[0]
    } else {
        0 // mov #0,r14 (bf.s delay slot, runs either way)
    };
    cpu.set_t(take);
    // Six indexed stores; r0 and r5 are reloaded before each call, and the
    // callee leaves r4 = target+92, r1 = [target+92], r0 = table base.
    for off in [76u32, 0xc0, 0x134, 0x1a8, 0x21c, 0x290] {
        cpu.r[0] = 0x8c16eadc;
        cpu.r[4] = r13.wrapping_add(off);
        cpu.r[5] = r14;
        crate::registry::p_8c16eadc::run(cpu, mem);
    }
}
