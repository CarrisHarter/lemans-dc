//! 0x8c016484 — strcpy: dst=r0, src=r1, calling convention aside.
//!
//! Entry pushes r3 and r4, computes `(dst & 3) | (src & 3)` and branches:
//!
//!   * both 4-aligned → `mov.l @(4,pc),r3` (literal at 0x8c0164d0 holds
//!     0x8c0163e8) then `jmp @r3`, a tail call into the word-at-a-time
//!     strcpy. That callee pushes r5,r6 but pops r6,r5,r4,r3: its last two
//!     pops consume OUR saved r4 and r3, so it unwinds our frame completely.
//!     Every register except r0/r1 ends at its entry value, including r15,
//!     and r0 = dst (callee `mov r3,r0`).
//!
//!   * otherwise → unrolled byte loop: `mov.b` loads (sign-extended) and
//!     stores at r3+0..3, `bra` back with `add #4,r3` in the delay slot. A
//!     zero byte is stored, its `cmp/eq #0,r0` sets T, then `bt` exits.
//!     `mov r4,r0` puts dst in r0 and the two pops restore r3/r4.
//!
//! Residue: r0 = original dst, r1 = one past the copied zero byte, r2 and
//! r7..r14 untouched, r15 unchanged (2 pushes + 2 pops on the byte path,
//! 2 pushes + the callee's 4 pops on the aligned path).
//!
//! T: the byte path always exits on a matching `cmp/eq #0,r0`, so T = 1.
//! The aligned path ends with the callee's byte tail, which runs `cmp/eq`
//! after the 1st, 2nd and 3rd byte only: T = 1 unless the terminating zero is
//! the 4th byte of its word (then T keeps the 3rd byte's clear result, 0).
//!
//! The pushes' stores are inside the caller's frame (the synthetic stack
//! window the verifier excludes), not modelled — same convention as the
//! standalone 0x8c0163e8 port.

use sh4_oracle::{Cpu, Memory};

fn has_zero_byte(v: u32) -> bool {
    v & 0x0000_00ff == 0
        || v & 0x0000_ff00 == 0
        || v & 0x00ff_0000 == 0
        || v & 0xff00_0000 == 0
}

/// 0x8c0163e8's byte tail (0x8c016420), entered with the source pointer rewound
/// four and the write base at the word that contains the zero byte.
/// Returns the source pointer (one past the zero) and the exit T bit.
fn byte_tail(mem: &mut Memory, mut src: u32, base: u32) -> (u32, bool) {
    let mut t = false;
    for off in 0..4u32 {
        let byte = mem.read_u8(src).unwrap_or(0); // mov.b @r1+,r0 (sign-extended)
        src = src.wrapping_add(1);
        if off < 3 {
            t = byte == 0; // cmp/eq #0,r0
        }
        let _ = mem.write_u8(base.wrapping_add(off), byte); // mov.b r0,@(off,r6)
        if off < 3 && t {
            break; // bt 0x8c016440
        }
    }
    (src, t)
}

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let dst = cpu.r[0];
    let mut src = cpu.r[1];

    // mov #3,r3 ; mov r0,r4 ; and r3,r4 ; and r1,r3 ; or r4,r3 ; tst r3,r3
    let aligned = (dst & 3) | (src & 3) == 0;
    cpu.set_t(aligned);

    if aligned {
        // 0x8c0164c8 mov.l @(4,pc),r3 ; 0x8c0164ca jmp @r3 ; delay nop
        // → 0x8c0163e8, the word-at-a-time strcpy.
        let mut d = dst;
        let mut word = mem.read_u32(src).unwrap_or(0); // 0x8c0163ee
        src = src.wrapping_add(4);
        let (tail_src, t) = loop {
            if has_zero_byte(word) {
                // 0x8c0163f2 cmp/str r4,r6 ; 0x8c0163f4 bt 0x8c016420
                break byte_tail(mem, src.wrapping_sub(4), d);
            }
            let mut next = mem.read_u32(src).unwrap_or(0); // 0x8c0163f6
            src = src.wrapping_add(4);
            let _ = mem.write_u32(d, word); // 0x8c0163f8 mov.l r4,@r0
            if has_zero_byte(next) {
                // 0x8c0163fc bt 0x8c016416 ; bra 0x8c016420 ; delay add #4,r0
                break byte_tail(mem, src.wrapping_sub(4), d.wrapping_add(4));
            }
            word = mem.read_u32(src).unwrap_or(0); // 0x8c0163fe
            src = src.wrapping_add(4);
            let _ = mem.write_u32(d.wrapping_add(4), next); // 0x8c016400
            if has_zero_byte(word) {
                // 0x8c016404 bt 0x8c01641a ; delay add #8,r0
                break byte_tail(mem, src.wrapping_sub(4), d.wrapping_add(8));
            }
            next = mem.read_u32(src).unwrap_or(0); // 0x8c016406
            src = src.wrapping_add(4);
            let _ = mem.write_u32(d.wrapping_add(8), word); // 0x8c016408
            if has_zero_byte(next) {
                // 0x8c01640c bt 0x8c01641e ; add #12,r0 (no delay slot)
                break byte_tail(mem, src.wrapping_sub(4), d.wrapping_add(12));
            }
            word = mem.read_u32(src).unwrap_or(0); // 0x8c01640e
            src = src.wrapping_add(4);
            let _ = mem.write_u32(d.wrapping_add(12), next); // 0x8c016410
            d = d.wrapping_add(16); // 0x8c016414 delay slot of bra 0x8c0163f2
        };
        src = tail_src;
        cpu.set_t(t);
        cpu.r[0] = dst; // 0x8c016442 mov r3,r0 (callee's r3 = dst)
        cpu.r[1] = src;
        return;
    }

    // Byte path: 0x8c016496 mov r0,r4 ; 0x8c016498 mov.b @r1+,r0 ; mov r4,r3.
    let mut d = dst;
    let mut cur = mem.read_u8(src).unwrap_or(0) as i8 as i32 as u32;
    src = src.wrapping_add(1);
    loop {
        cpu.set_t(cur == 0); // 0x8c01649c cmp/eq #0,r0
        let _ = mem.write_u8(d, cur as u8); // 0x8c01649e mov.b r0,@r3
        if cur == 0 {
            break; // 0x8c0164a0 bt 0x8c0164c0
        }
        cur = mem.read_u8(src).unwrap_or(0) as i8 as i32 as u32;
        src = src.wrapping_add(1);
        cpu.set_t(cur == 0);
        let _ = mem.write_u8(d.wrapping_add(1), cur as u8); // mov.b r0,@(1,r3)
        if cur == 0 {
            break;
        }
        cur = mem.read_u8(src).unwrap_or(0) as i8 as i32 as u32;
        src = src.wrapping_add(1);
        cpu.set_t(cur == 0);
        let _ = mem.write_u8(d.wrapping_add(2), cur as u8); // mov.b r0,@(2,r3)
        if cur == 0 {
            break;
        }
        cur = mem.read_u8(src).unwrap_or(0) as i8 as i32 as u32;
        src = src.wrapping_add(1);
        cpu.set_t(cur == 0);
        let _ = mem.write_u8(d.wrapping_add(3), cur as u8); // mov.b r0,@(3,r3)
        if cur == 0 {
            break;
        }
        cur = mem.read_u8(src).unwrap_or(0) as i8 as i32 as u32; // 0x8c0164ba
        src = src.wrapping_add(1);
        d = d.wrapping_add(4); // 0x8c0164be delay slot of bra 0x8c01649c
    }
    cpu.r[0] = dst; // 0x8c0164c0 mov r4,r0 (r4 = dst)
    cpu.r[1] = src;
}
