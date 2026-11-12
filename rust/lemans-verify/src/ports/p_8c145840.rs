//! 0x8c145840 — walk a singly-linked list from `*r4` looking for the node whose
//! address equals the pointer stored at `[r4]`, stopping after `r5` steps.
//!
//! ```text
//! push r14,r13,r12,r11,pr ; add #-16,r15       ; 16-byte frame, F
//! if *(char*)0x8c2abc20 == 0 {                 ; one-time lazy init
//!     fr0 = 0.0
//!     *(f32*)0x8c2abc28 = fr0 ; +4 ; +8
//!     *(char*)0x8c2abc20 = 1
//! }
//! r0 = [r12+4]                                 ; limit
//! T  = r13 >= r0                               ; cmp/hs, unsigned
//! if T: return 0x8c2abc28                      ; no traversal
//! 0x8c145594(r12, &local8)                     ; local8 = *(*(u32*)r12)
//! local12 = local8                             ; search starts at *X (X = [r4])
//! loop {
//!    0x8c145580(r12, F)                       ; local0 = *r12 == X
//!    if local0 == local12: break              ; found (bt)
//!    if r14 == r13:        break              ; r5 steps spent
//!    0x8c145588(&local12, 0, &local4)         ; local12 = *local12, local4 = old
//!    r14++
//! }
//! return *(u32*)(local12 + 8)                  ; node payload (or default object)
//! ```
//!
//! `subc r11,r11 ; add #1,r11 ; extu.b ; tst` only reproduces the `cmp/eq`
//! flag (r11 becomes 1-T, then `tst` re-derives T); r11 is popped on exit, so
//! its intermediate value is dead and the port computes the `cmp/eq` directly.
//! `r5` is only cleared by the advance call's delay slot, so a traversal that
//! exits before the first advance keeps the caller's `r5`. All exits leave
//! `T = 1`, `r3` = literal 0x8c2abc20, and `r4` = the entry argument.
//!
//! The three `fmov fr0,@...` stores land in RAM at 0x8c2abc28/2c/30, followed
//! by the flag byte at 0x8c2abc20 — that order is part of the recorded stores.

use sh4_oracle::{Cpu, Memory};

/// Literal at 0x8c1458e0: lazily-initialised flag byte.
const FLAG_LITERAL: u32 = 0x8c1458e0;
const FLAG_DEFAULT: u32 = 0x8c2abc20;
/// Literal at 0x8c1458e4: zero-vector returned when the walk is skipped and
/// as the search base on the flag-init path.
const BASE_LITERAL: u32 = 0x8c1458e4;
const BASE_DEFAULT: u32 = 0x8c2abc28;

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    // Prologue: save r14, r13, r12, r11, pr, then open the 16-byte frame.
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[14]);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[13]);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[12]);
    cpu.r[13] = cpu.r[5]; // limit / step count
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[11]);
    cpu.r[12] = cpu.r[4]; // list owner
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr);
    cpu.r[15] = cpu.r[15].wrapping_sub(16);
    let frame = cpu.r[15];

    // mov.l 0x8c1458e0,r3 ; mov #0,r1 ; mov.b @r3,r0 ; exts.b ; tst r0,r0
    cpu.r[3] = mem.read_u32(FLAG_LITERAL).unwrap_or(FLAG_DEFAULT);
    cpu.r[1] = 0;
    cpu.r[0] = mem.read_u8(cpu.r[3]).unwrap_or(0) as i8 as i32 as u32;
    cpu.set_t(cpu.r[0] == 0);
    // Delay slot of bf.s: frame[12] (the search cursor) starts at zero.
    let _ = mem.write_u32(frame + 12, cpu.r[1]);
    if cpu.t() {
        // First call: zero the three-float default vector, then raise the flag.
        cpu.fr[0] = 0.0; // fldi0 fr0
        cpu.r[1] = mem.read_u32(BASE_LITERAL).unwrap_or(BASE_DEFAULT);
        cpu.r[0] = cpu.r[1];
        let _ = mem.write_f32(cpu.r[1], cpu.fr[0]);
        cpu.r[0] = cpu.r[0].wrapping_add(4);
        let _ = mem.write_f32(cpu.r[0], cpu.fr[0]);
        cpu.r[1] = cpu.r[1].wrapping_add(8);
        let _ = mem.write_f32(cpu.r[1], cpu.fr[0]);
        cpu.r[0] = 1;
        let _ = mem.write_u8(cpu.r[3], cpu.r[0] as u8);
    }

    // mov.l @(4,r12),r0 ; cmp/hs r0,r13 ; bf.s ... ; delay: mov #0,r14
    cpu.r[0] = mem.read_u32(cpu.r[12].wrapping_add(4)).unwrap_or(0);
    cpu.set_t(cpu.r[13] >= cpu.r[0]);
    cpu.r[14] = 0;
    if cpu.t() {
        // Step budget not below the limit: return the default vector.
        cpu.r[0] = mem.read_u32(BASE_LITERAL).unwrap_or(BASE_DEFAULT);
        epilogue(cpu, mem);
        return;
    }

    // 0x8c145594(r12, &frame[8]): frame[8] = *(*(u32*)r12).
    cpu.r[0] = frame + 8;
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[0]);
    cpu.r[4] = cpu.r[12]; // delay slot of jsr
    cpu.pr = 0x8c14588c;
    crate::registry::p_8c145594::run(cpu, mem);
    cpu.r[15] = cpu.r[15].wrapping_add(4);

    // Cursor = first node; the `bra` jumps straight into the load below.
    cpu.r[0] = frame + 8;
    cpu.r[1] = frame + 12;
    cpu.r[0] = mem.read_u32(cpu.r[0]).unwrap_or(0);
    let _ = mem.write_u32(cpu.r[1], cpu.r[0]); // delay slot of `bra 0x8c1458b0`

    loop {
        // 0x8c145580(r12, frame): frame[0] = *(u32*)r12.
        cpu.r[0] = cpu.r[15];
        cpu.r[15] = cpu.r[15].wrapping_sub(4);
        let _ = mem.write_u32(cpu.r[15], cpu.r[0]);
        cpu.r[4] = cpu.r[12]; // delay slot of jsr
        cpu.pr = 0x8c1458ba;
        crate::registry::p_8c145580::run(cpu, mem);
        cpu.r[15] = cpu.r[15].wrapping_add(4);

        cpu.r[1] = mem.read_u32(frame + 12).unwrap_or(0);
        cpu.r[0] = mem.read_u32(frame).unwrap_or(0);
        cpu.set_t(cpu.r[0] == cpu.r[1]); // cmp/eq r1,r0, re-derived by tst
        if cpu.t() {
            break; // bt 0x8c1458d0 — cursor found
        }
        cpu.set_t(cpu.r[14] == cpu.r[13]); // cmp/eq r13,r14
        if cpu.t() {
            break; // bf not taken — step budget spent
        }

        // 0x8c145588(&frame[12], 0, &frame[4]): advance cursor, stash the old one.
        cpu.r[0] = frame + 4;
        cpu.r[15] = cpu.r[15].wrapping_sub(4);
        let _ = mem.write_u32(cpu.r[15], cpu.r[0]);
        cpu.r[4] = cpu.r[15].wrapping_add(16);
        cpu.r[5] = 0; // delay slot of jsr
        cpu.pr = 0x8c1458ac;
        crate::registry::p_8c145588::run(cpu, mem);
        cpu.r[15] = cpu.r[15].wrapping_add(4);
        cpu.r[14] = cpu.r[14].wrapping_add(1);
    }

    // mov.l @(8,r1),r0 with r1 = surviving cursor.
    cpu.r[0] = mem.read_u32(cpu.r[1].wrapping_add(8)).unwrap_or(0);
    epilogue(cpu, mem);
}

fn epilogue(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[15] = cpu.r[15].wrapping_add(16);
    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0); // lds.l @r15+,pr
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[11] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[12] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[13] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    // rts; delay slot: mov.l @r15+,r14
    cpu.r[14] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}
