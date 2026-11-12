//! 0x8c183e04 — scan a pointer array for the element named `param_2`.
//!
//! `param_1` is a three-word struct: `[4]` element count, `[8]` array base.
//! The scan walks `base .. base + count*4` and calls 0x8c070b0c (the
//! case-insensitive string compare) with `param_2` and `[[*cursor]+12]`, i.e.
//! the name reached through the element's `[*cursor]` record:
//!
//! ```text
//! for (p = param_1[2]; p != param_1[2] + param_1[1]*4; p++) {
//!   if (f_8c070b0c(*(u32*)(*(u32*)*p + 12), param_2) == 0)
//!     return *p + 4;
//! }
//! return 0;
//! ```
//!
//! The `mov.l 0x8c183e44,r0` literal (inside the following routine's bytes) is
//! 0x8c070b0c; the jsr delay slot loads the callee's first argument.
//!
//! Side effects: r0/r1/r4/r5 are whatever the last callee left (r4/r5 advance,
//! r1 = last viewed byte) with r0 replaced by `*cursor + 4` on the match path
//! or 0 on exhaustion; r12-r14 and pr come back from the four-word push. T is
//! 1 on both exits: `cmp/eq` equal on the exhaustion path, `tst r0,r0` true on
//! the match path. No stores outside the stack window.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    // Prologue: mov.l r14,@-r15 ; mov.l r13,@-r15 ; mov.l r12,@-r15 ;
    // sts.l pr,@-r15.
    for reg in [14usize, 13, 12] {
        cpu.r[15] = cpu.r[15].wrapping_sub(4);
        let _ = mem.write_u32(cpu.r[15], cpu.r[reg]);
    }
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr);

    cpu.r[13] = cpu.r[5]; // mov r5,r13 — param_2
    cpu.r[12] = cpu.r[4]; // mov r4,r12 — param_1
    cpu.r[14] = mem.read_u32(cpu.r[12].wrapping_add(8)).unwrap_or(0); // bra delay slot

    loop {
        // mov.l @(4,r12),r0 ; mov.l @(8,r12),r1 ; shll2 r0 ; add r0,r1 ;
        // cmp/eq r14,r1 ; bf 0x8c183e14.
        cpu.r[0] = mem.read_u32(cpu.r[12].wrapping_add(4)).unwrap_or(0);
        cpu.r[1] = mem.read_u32(cpu.r[12].wrapping_add(8)).unwrap_or(0);
        cpu.r[0] = cpu.r[0] << 2; // shll2 r0
        cpu.r[1] = cpu.r[1].wrapping_add(cpu.r[0]); // add r0,r1
        cpu.set_t(cpu.r[14] == cpu.r[1]); // cmp/eq r14,r1
        if cpu.t() {
            cpu.r[0] = 0; // mov #0,r0
            break;
        }

        // 0x8c183e14: mov.l @r14,r1 ; mov r13,r5 ; mov.l 0x8c183e44,r0 ;
        // mov.l @r1,r1 ; jsr @r0 ; mov.l @(12,r1),r4 (delay slot).
        cpu.r[1] = mem.read_u32(cpu.r[14]).unwrap_or(0);
        cpu.r[5] = cpu.r[13];
        cpu.r[0] = mem.read_u32(0x8C183E44).unwrap_or(0);
        cpu.r[1] = mem.read_u32(cpu.r[1]).unwrap_or(0);
        cpu.r[4] = mem.read_u32(cpu.r[1].wrapping_add(12)).unwrap_or(0);
        cpu.pr = 0x8C183E20;
        crate::registry::p_8c070b0c::run(cpu, mem);

        cpu.set_t(cpu.r[0] == 0); // tst r0,r0
        if cpu.t() {
            cpu.r[0] = mem.read_u32(cpu.r[14]).unwrap_or(0); // mov.l @r14,r0
            cpu.r[0] = cpu.r[0].wrapping_add(4); // bra delay slot: add #4,r0
            break;
        }
        cpu.r[14] = cpu.r[14].wrapping_add(4); // add #4,r14
    }

    // Epilogue: lds.l @r15+,pr ; mov.l @r15+,r12 ; mov.l @r15+,r13 ; rts ;
    // mov.l @r15+,r14 (delay slot).
    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    for reg in [12usize, 13, 14] {
        cpu.r[reg] = mem.read_u32(cpu.r[15]).unwrap_or(0);
        cpu.r[15] = cpu.r[15].wrapping_add(4);
    }
}
