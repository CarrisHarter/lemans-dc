//! 0x8c06ae60 — tear down a resource struct: free every buffer it owns and,
//! when the `short` in r5 is positive, free the struct itself.
//!
//! Layout (disassembly 0x8c06ae60-0x8c06aef6):
//!   +0x14  pointer to a table whose `*(int *)(p + 0x2c)` entries are freed
//!   +0x18  a block freed unconditionally
//!   +0x1c  a block freed unconditionally
//!   +0x20  pointer to `*(int *)(p + 0x24)` rows of four pointers, all freed
//!   then the +0x20 base itself is freed, and finally `p` when
//!   `(int16)r5 > 0`. Returns `p` (mov r13,r0) regardless.
//!
//! Both loops are signed `cmp/gt` countdowns (zero or negative counts skip
//! them), and the inner loop's `bt.s` jumps back past its own `r12 = r11 = 0`
//! initialisation, so a row is always exactly four frees at +0, +4, +8, +12
//! from `base[i * 16]`. Every free goes through the 0x8c0102cc thunk, whose
//! `r0`/`r1`/`r4..r6` residue (and the `r14`-dependent branch's) survives
//! because the thunk tail-calls 0x8c159fc0 without touching `pr`.

use sh4_oracle::{Cpu, Memory};

/// Literal word read for `mov.l 0x8c06aef8,rn` (holds 0x8c0102cc).
const FREE_THUNK: u32 = 0x8c06aef8;
/// Literal word read for `mov.l 0x8c06aefc,rn` (holds 0x8c01032c).
const FREE_SELF: u32 = 0x8c06aefc;

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    // 0x8c06ae60..0x8c06ae70: save the callee-saved registers, pr last.
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[14]);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[13]);
    cpu.r[14] = cpu.r[5];
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[12]);
    cpu.r[13] = cpu.r[4];
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[11]);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[10]);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[9]);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr);

    // 0x8c06ae72: tst r13,r13 ; 0x8c06ae74: bt 0x8c06aee6
    cpu.set_t(cpu.r[13] == 0);
    if !cpu.t() {
        // 0x8c06ae76..0x8c06ae90: free the *(int *)(p + 0x2c) entries at
        // *(p + 0x14), stride 4. Body re-loads the table pointer each trip.
        cpu.r[12] = 0;
        cpu.r[11] = 0; // mov r12,r11 ; shll2 r11
        loop {
            cpu.r[0] = mem.read_u32(cpu.r[13].wrapping_add(44)).unwrap_or(0);
            let count = cpu.r[0];
            cpu.set_t((count as i32) > (cpu.r[12] as i32)); // cmp/gt r12,r0
            if !cpu.t() {
                break;
            }
            cpu.r[3] = mem.read_u32(cpu.r[13].wrapping_add(20)).unwrap_or(0);
            cpu.r[0] = cpu.r[11];
            cpu.r[1] = mem.read_u32(FREE_THUNK).unwrap_or(0);
            cpu.pr = 0x8c06ae88;
            // jsr delay slot: mov.l @(r0,r3),r4
            cpu.r[4] = mem.read_u32(cpu.r[3].wrapping_add(cpu.r[0])).unwrap_or(0);
            crate::registry::p_8c0102cc::run(cpu, mem);
            cpu.r[11] = cpu.r[11].wrapping_add(4);
            cpu.r[12] = cpu.r[12].wrapping_add(1);
        }

        // 0x8c06ae92..0x8c06aea2: free the +0x14/+0x18/+0x1c blocks. These
        // jsr through r0; r1 keeps the thunk pointer from the loop above.
        for (offset, ret) in [(20u32, 0x8c06ae98u32), (24, 0x8c06ae9c), (28, 0x8c06aea0)] {
            cpu.r[0] = mem.read_u32(FREE_THUNK).unwrap_or(0);
            cpu.pr = ret;
            cpu.r[4] = mem.read_u32(cpu.r[13].wrapping_add(offset)).unwrap_or(0);
            crate::registry::p_8c0102cc::run(cpu, mem);
        }

        // 0x8c06aea4..0x8c06aed2: for each of the *(int *)(p + 0x24) rows,
        // free the four pointers at *(p + 0x20) + row * 16 + {0,4,8,12}.
        cpu.r[9] = 0;
        cpu.r[10] = 0; // mov r9,r10 ; shll2 r10
        loop {
            cpu.r[0] = mem.read_u32(cpu.r[13].wrapping_add(36)).unwrap_or(0);
            let count = cpu.r[0];
            cpu.set_t((count as i32) > (cpu.r[9] as i32)); // cmp/gt r9,r0
            if !cpu.t() {
                break;
            }
            cpu.r[12] = 0;
            cpu.r[11] = 0; // mov r12,r11 ; shll2 r11
            loop {
                cpu.r[3] = mem.read_u32(cpu.r[13].wrapping_add(32)).unwrap_or(0);
                cpu.r[0] = cpu.r[11];
                cpu.r[1] = mem.read_u32(FREE_THUNK).unwrap_or(0);
                cpu.r[3] = cpu.r[3].wrapping_add(cpu.r[10]);
                cpu.pr = 0x8c06aec0;
                // jsr delay slot: mov.l @(r0,r3),r4
                cpu.r[4] = mem.read_u32(cpu.r[3].wrapping_add(cpu.r[0])).unwrap_or(0);
                crate::registry::p_8c0102cc::run(cpu, mem);
                cpu.r[12] = cpu.r[12].wrapping_add(1);
                cpu.r[0] = 4; // mov #4,r0
                cpu.set_t((cpu.r[0] as i32) > (cpu.r[12] as i32)); // cmp/gt r12,r0
                cpu.r[11] = cpu.r[11].wrapping_add(4); // bt.s delay slot
                if !cpu.t() {
                    break;
                }
            }
            cpu.r[10] = cpu.r[10].wrapping_add(16);
            cpu.r[9] = cpu.r[9].wrapping_add(1);
        }

        // 0x8c06aed4..0x8c06aed8: free the row-table base.
        cpu.r[0] = mem.read_u32(FREE_THUNK).unwrap_or(0);
        cpu.pr = 0x8c06aeda;
        cpu.r[4] = mem.read_u32(cpu.r[13].wrapping_add(32)).unwrap_or(0);
        crate::registry::p_8c0102cc::run(cpu, mem);

        // 0x8c06aeda..0x8c06aee4: free the struct itself when (int16)r5 > 0.
        cpu.r[0] = (cpu.r[14] as i16) as i32 as u32; // exts.w r14,r0
        cpu.set_t((cpu.r[0] as i32) > 0); // cmp/pl r0
        if cpu.t() {
            cpu.r[0] = mem.read_u32(FREE_SELF).unwrap_or(0);
            cpu.pr = 0x8c06aee6;
            cpu.r[4] = cpu.r[13]; // jsr delay slot: mov r13,r4
            crate::registry::p_8c01032c::run(cpu, mem);
        }
    }

    // 0x8c06aee6..0x8c06aef6: r0 = p, then pop in reverse save order; the
    // rts delay slot restores r14.
    cpu.r[0] = cpu.r[13];
    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[9] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[10] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[11] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[12] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[13] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[14] = mem.read_u32(cpu.r[15]).unwrap_or(0); // rts delay slot
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}
