//! 0x8c091c34 — merge N records of 0x148 bytes into one, field by field,
//! keeping the signed minimum of each dword.
//!
//! The destination pointer is the third argument and arrives on the stack: the
//! caller pushes it before the `jsr` (see 0x8c0920ae), so the routine reads it
//! from its entry sp. That slot is also where the running destination pointer
//! lives — the code spills it to `buf+0x168` (== entry sp) and reloads it for
//! every store, so the caller's argument slot ends up pointing one record past
//! the merged record.
//!
//! Shape: zero a 0x148-byte scratch record (0x8c091a80), seed the 46 dwords of
//! the record's first block and the 36 dwords of the second block with 0x1869f,
//! take the signed minimum of each field across the N records (strict `>`, so a
//! tie keeps the running value), then copy both blocks to the destination, the
//! first block 2 dwords at a time, the second 2 dwords at a time from +0xb8.
//!
//! Side effects: r0 = the last dword copied (second block's final word),
//! r1 = dest+0x148, r2 = the frame slot address (scratch+0x168), r3 = 0,
//! r4 = scratch+0x148, r5 = N (0 when N <= 0), r6 = N*0x148 (0 when N <= 0),
//! r7 = 0x90, r8..r14/r15/pr restored, T set by the final `dt`.

use sh4_oracle::{Cpu, Memory};

const SEED: u32 = 0x0001_869f;
const STRIDE: u32 = 0x148;
const TAIL: u32 = 0xb8;
const FIRST_WORDS: u32 = 46;
const SECOND_WORDS: u32 = 36;
const SLOT: u32 = 0x168;

fn ld(mem: &Memory, addr: u32) -> u32 {
    mem.read_u32(addr).unwrap_or(0)
}

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    // 0x8c091c34..0x8c091c4c — save r8..r14 and pr, then the 0x148-byte frame.
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[14]);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[13]);
    cpu.r[14] = cpu.r[5]; // N
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[12]);
    cpu.r[13] = cpu.r[4]; // source
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[11]);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[10]);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[9]);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[8]);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr);
    cpu.r[15] = cpu.r[15].wrapping_sub(0x148);
    // 0x8c091c4e — jsr 0x8c091a80, delay slot r4 = scratch
    cpu.r[0] = 0x8c091a80;
    cpu.pr = 0x8c091c54;
    cpu.r[4] = cpu.r[15];
    crate::registry::p_8c091a80::run(cpu, mem);
    let buf = cpu.r[15];

    // 0x8c091c54..0x8c091c60 — first block: 46 dwords, signed minimum.
    cpu.r[11] = SEED;
    cpu.r[5] = 0;
    cpu.r[10] = STRIDE;
    cpu.r[12] = cpu.r[5];
    cpu.r[3] = cpu.r[5];
    cpu.r[4] = buf;
    cpu.r[8] = FIRST_WORDS;
    let n = cpu.r[14] as i32;
    loop {
        // 0x8c091c62 — buf[i] = 0x1869f, then the inner min loop for N > 0.
        cpu.r[0] = cpu.r[4];
        let _ = mem.write_u32(cpu.r[0].wrapping_add(cpu.r[12]), cpu.r[11]);
        let take = n > 0; // cmp/pl r14; bf.s 0x8c091c90
        cpu.set_t(take);
        cpu.r[6] = cpu.r[3]; // delay slot, taken or not
        if take {
            cpu.r[7] = cpu.r[3];
            loop {
                // 0x8c091c6e — r1 = src[j*0x148 + i*4], r9 = buf[i]
                cpu.r[1] = cpu.r[13];
                cpu.r[0] = cpu.r[12];
                cpu.r[1] = cpu.r[1].wrapping_add(cpu.r[7]);
                cpu.r[1] = ld(mem, cpu.r[0].wrapping_add(cpu.r[1]));
                cpu.r[0] = cpu.r[4];
                cpu.r[9] = ld(mem, cpu.r[0].wrapping_add(cpu.r[12]));
                // 0x8c091c7a — cmp/gt r9,r1; bf keeps the new value.
                let keep = (cpu.r[1] as i32) > (cpu.r[9] as i32);
                cpu.set_t(keep);
                if !keep {
                    cpu.r[9] = cpu.r[1];
                }
                cpu.r[0] = cpu.r[4];
                cpu.r[6] = cpu.r[6].wrapping_add(1);
                let _ = mem.write_u32(cpu.r[0].wrapping_add(cpu.r[12]), cpu.r[9]);
                // 0x8c091c8a — cmp/gt r6,r14; delay slot walks the stride.
                let more = n > (cpu.r[6] as i32);
                cpu.set_t(more);
                cpu.r[7] = cpu.r[7].wrapping_add(cpu.r[10]);
                if !more {
                    break;
                }
            }
        }
        // 0x8c091c90 — next dword, 46 of them.
        cpu.r[5] = cpu.r[5].wrapping_add(1);
        let more = (cpu.r[8] as i32) > (cpu.r[5] as i32);
        cpu.set_t(more);
        cpu.r[12] = cpu.r[12].wrapping_add(4);
        if !more {
            break;
        }
    }

    // 0x8c091c98..0x8c091caa — second block: 36 dwords from src+0xb8.
    cpu.r[8] = SEED;
    cpu.r[10] = 0;
    cpu.r[9] = STRIDE;
    cpu.r[7] = cpu.r[10];
    cpu.r[0] = TAIL;
    cpu.r[3] = cpu.r[10];
    cpu.r[4] = TAIL;
    cpu.r[13] = cpu.r[13].wrapping_add(cpu.r[0]);
    cpu.r[12] = SECOND_WORDS;
    cpu.r[4] = cpu.r[4].wrapping_add(buf);
    loop {
        cpu.r[0] = cpu.r[4];
        let _ = mem.write_u32(cpu.r[0].wrapping_add(cpu.r[7]), cpu.r[8]);
        let take = (cpu.r[14] as i32) > 0; // cmp/pl r14; bf.s 0x8c091cda
        cpu.set_t(take);
        cpu.r[5] = cpu.r[3]; // delay slot, taken or not
        if take {
            cpu.r[6] = cpu.r[3];
            loop {
                cpu.r[1] = cpu.r[13];
                cpu.r[0] = cpu.r[7];
                cpu.r[1] = cpu.r[1].wrapping_add(cpu.r[6]);
                cpu.r[1] = ld(mem, cpu.r[0].wrapping_add(cpu.r[1]));
                cpu.r[0] = cpu.r[4];
                cpu.r[11] = ld(mem, cpu.r[0].wrapping_add(cpu.r[7]));
                let keep = (cpu.r[1] as i32) > (cpu.r[11] as i32);
                cpu.set_t(keep);
                if !keep {
                    cpu.r[11] = cpu.r[1];
                }
                cpu.r[0] = cpu.r[4];
                cpu.r[5] = cpu.r[5].wrapping_add(1);
                let _ = mem.write_u32(cpu.r[0].wrapping_add(cpu.r[7]), cpu.r[11]);
                let more = (cpu.r[14] as i32) > (cpu.r[5] as i32);
                cpu.set_t(more);
                cpu.r[6] = cpu.r[6].wrapping_add(cpu.r[9]);
                if !more {
                    break;
                }
            }
        }
        // 0x8c091cda — 36 dwords.
        cpu.r[10] = cpu.r[10].wrapping_add(1);
        let more = (cpu.r[12] as i32) > (cpu.r[10] as i32);
        cpu.set_t(more);
        cpu.r[7] = cpu.r[7].wrapping_add(4);
        if !more {
            break;
        }
    }

    // 0x8c091ce2..0x8c091cf6 — copy the first block to the stack-held pointer.
    cpu.r[0] = SLOT;
    cpu.r[4] = buf;
    cpu.r[1] = 23;
    cpu.r[3] = ld(mem, cpu.r[0].wrapping_add(buf));
    loop {
        cpu.r[0] = ld(mem, cpu.r[4]);
        cpu.r[4] = cpu.r[4].wrapping_add(4);
        cpu.r[1] = cpu.r[1].wrapping_sub(1);
        let last = cpu.r[1] == 0; // dt r1
        cpu.set_t(last);
        let _ = mem.write_u32(cpu.r[3], cpu.r[0]);
        cpu.r[0] = ld(mem, cpu.r[4]);
        cpu.r[4] = cpu.r[4].wrapping_add(4);
        let _ = mem.write_u32(cpu.r[3].wrapping_add(4), cpu.r[0]);
        cpu.r[3] = cpu.r[3].wrapping_add(8); // delay slot of bf.s
        if last {
            break;
        }
    }

    // 0x8c091cf8..0x8c091d08 — dest = dest+0xb8, back into the frame slot.
    cpu.r[2] = SLOT;
    cpu.r[3] = 18;
    cpu.r[0] = TAIL;
    cpu.r[2] = cpu.r[2].wrapping_add(buf);
    cpu.r[1] = ld(mem, cpu.r[2]);
    cpu.r[2] = SLOT;
    cpu.r[1] = cpu.r[1].wrapping_add(cpu.r[0]);
    cpu.r[2] = cpu.r[2].wrapping_add(buf);
    let _ = mem.write_u32(cpu.r[2], cpu.r[1]);
    cpu.r[4] = TAIL;
    cpu.r[4] = cpu.r[4].wrapping_add(buf);

    // 0x8c091d0e..0x8c091d26 — copy the second block (dest reloaded per pair).
    loop {
        cpu.r[2] = SLOT;
        cpu.r[3] = cpu.r[3].wrapping_sub(1);
        let last = cpu.r[3] == 0; // dt r3
        cpu.set_t(last);
        cpu.r[0] = ld(mem, cpu.r[4]);
        cpu.r[4] = cpu.r[4].wrapping_add(4);
        cpu.r[2] = cpu.r[2].wrapping_add(buf);
        cpu.r[1] = ld(mem, cpu.r[2]);
        let _ = mem.write_u32(cpu.r[1], cpu.r[0]);
        cpu.r[0] = ld(mem, cpu.r[4]);
        cpu.r[4] = cpu.r[4].wrapping_add(4);
        let _ = mem.write_u32(cpu.r[1].wrapping_add(4), cpu.r[0]);
        cpu.r[1] = cpu.r[1].wrapping_add(8);
        cpu.r[2] = SLOT;
        cpu.r[2] = cpu.r[2].wrapping_add(buf);
        let _ = mem.write_u32(cpu.r[2], cpu.r[1]); // delay slot of bf.s
        if last {
            break;
        }
    }

    // 0x8c091d28..0x8c091d3e — drop the frame, restore the saved registers.
    cpu.r[15] = cpu.r[15].wrapping_add(0x148);
    cpu.pr = ld(mem, cpu.r[15]);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    for i in [8, 9, 10, 11, 12, 13, 14] {
        cpu.r[i] = ld(mem, cpu.r[15]);
        cpu.r[15] = cpu.r[15].wrapping_add(4);
    }
}
