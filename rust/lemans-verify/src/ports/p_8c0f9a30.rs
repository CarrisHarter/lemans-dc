//! 0x8c0f9a30 — keyed list search over the `[r4+4]` chain.
//!
//! ```text
//! push r14,r13,r12,r11,pr ; r12 = r5 (key holder), r11 = r4 (list holder)
//! r13 = [r11+4]            ; head node, or null
//! r14 = 0                  ; bt.s delay slot: always runs
//! loop:
//!   strcmp([r1],[r3])      ; r4 = [[r12]], r5 = [[r13+12]] via the 0x8c0f9aa4 thunk
//!   T = (0 > r0)           ; r1 = 0; cmp/gt r0,r1 (signed); movt; extu.b; tst
//!   strcmp >= 0: r14 = r13, r13 = [r13+4]   ; bt L62 arm
//!   strcmp <  0: r13 = [r13]                ; bra delay slot only
//!   while r13 != 0
//! r14 == 0: [out] = r11+4
//! else:
//!   strcmp([[r14+12]], [[r12]])             ; note the swapped roles
//!   >= 0: [out] = r14  (bt L8e)
//!   <  0: [out] = r11+4 (falls into L86)
//! pop pr,r11,r12,r13,r14
//! ```
//!
//! The two loop arms advance through different links: the taken (`bt`) arm
//! records the node and follows `[r13+4]`, the fall-through arm follows
//! `[r13+0]` in the `bra` delay slot. `out` is the caller's word at
//! `[r15+20]` — the frame pushes 5 words, so that slot sits just above the
//! frame and must be seeded by the caller. Both `jsr`s return to the
//! instruction after their delay slot (0x8c0f9a52 / 0x8c0f9a7a); the strcmp
//! thunk is a `bra`, so `pr` survives both calls and is restored from the
//! stack at the end.

use sh4_oracle::{Cpu, Memory};

/// Literal pool word at 0x8c0f9aa0: the strcmp thunk both calls target.
const STRCMP: u32 = 0x8c0f9aa4;

/// Shared post-`jsr` tail: `r1 = 0; cmp/gt r0,r1; movt r0; extu.b r0,r0;
/// tst r0,r0`. Returns the final T: true iff the strcmp result was >= 0.
fn strcmp_sign(cpu: &mut Cpu) -> bool {
    cpu.r[1] = 0; // mov #0,r1
    cpu.set_t((cpu.r[0] as i32) < 0); // cmp/gt r0,r1: T = (0 > r0) signed
    cpu.r[0] = cpu.t() as u32; // movt r0 (extu.b is a no-op on 0/1)
    cpu.set_t(cpu.r[0] == 0); // tst r0,r0
    cpu.t()
}

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[15] = cpu.r[15].wrapping_sub(4); // mov.l r14,@-r15
    let _ = mem.write_u32(cpu.r[15], cpu.r[14]);
    cpu.r[15] = cpu.r[15].wrapping_sub(4); // mov.l r13,@-r15
    let _ = mem.write_u32(cpu.r[15], cpu.r[13]);
    cpu.r[15] = cpu.r[15].wrapping_sub(4); // mov.l r12,@-r15
    let _ = mem.write_u32(cpu.r[15], cpu.r[12]);
    cpu.r[15] = cpu.r[15].wrapping_sub(4); // mov.l r11,@-r15
    let _ = mem.write_u32(cpu.r[15], cpu.r[11]);
    cpu.r[12] = cpu.r[5]; // mov r5,r12
    cpu.r[15] = cpu.r[15].wrapping_sub(4); // sts.l pr,@-r15
    let _ = mem.write_u32(cpu.r[15], cpu.pr);
    cpu.r[11] = cpu.r[4]; // mov r4,r11
    cpu.r[13] = mem.read_u32(cpu.r[11].wrapping_add(4)).unwrap_or(0);
    cpu.set_t(cpu.r[13] == 0); // tst r13,r13
    cpu.r[14] = 0; // delay slot of `bt.s L6a`: always runs
    if !cpu.t() {
        // bt.s not taken: walk the chain.
        loop {
            // 0x8c0f9a46: r3 = [r13+12]; r1 = [r12]
            cpu.r[3] = mem.read_u32(cpu.r[13].wrapping_add(12)).unwrap_or(0);
            cpu.r[1] = mem.read_u32(cpu.r[12]).unwrap_or(0);
            cpu.r[0] = mem.read_u32(0x8c0f9aa0).unwrap_or(STRCMP);
            cpu.r[5] = mem.read_u32(cpu.r[3]).unwrap_or(0); // mov.l @r3,r5
            cpu.pr = 0x8c0f9a52; // jsr @r0
            cpu.r[4] = mem.read_u32(cpu.r[1]).unwrap_or(0); // delay: mov.l @r1,r4
            crate::registry::p_8c0f9aa4::run(cpu, mem);
            // 0x8c0f9a52 tail; bt L62 iff strcmp >= 0.
            if strcmp_sign(cpu) {
                cpu.r[14] = cpu.r[13]; // mov r13,r14
                cpu.r[13] = mem.read_u32(cpu.r[13].wrapping_add(4)).unwrap_or(0);
            } else {
                // bra L66; the delay slot is the whole fall-through arm.
                cpu.r[13] = mem.read_u32(cpu.r[13]).unwrap_or(0); // mov.l @r13,r13
            }
            cpu.set_t(cpu.r[13] == 0); // L66: tst r13,r13
            if cpu.t() {
                break; // bf loop not taken
            }
        }
    }
    // L6a: tst r14,r14; bt L86 iff r14 == 0.
    cpu.set_t(cpu.r[14] == 0);
    if !cpu.t() {
        // r3 = [r12]; r1 = [r14+12]; r5 = [r3]; jsr; delay r4 = [r1].
        cpu.r[3] = mem.read_u32(cpu.r[12]).unwrap_or(0);
        cpu.r[1] = mem.read_u32(cpu.r[14].wrapping_add(12)).unwrap_or(0);
        cpu.r[0] = mem.read_u32(0x8c0f9aa0).unwrap_or(STRCMP);
        cpu.r[5] = mem.read_u32(cpu.r[3]).unwrap_or(0);
        cpu.pr = 0x8c0f9a7a; // jsr @r0
        cpu.r[4] = mem.read_u32(cpu.r[1]).unwrap_or(0); // delay slot
        crate::registry::p_8c0f9aa4::run(cpu, mem);
        // 0x8c0f9a7a tail; bt L8e iff strcmp >= 0.
        if strcmp_sign(cpu) {
            // L8e: [out] = r14.
            cpu.r[0] = mem.read_u32(cpu.r[15].wrapping_add(20)).unwrap_or(0);
            let _ = mem.write_u32(cpu.r[0], cpu.r[14]);
        } else {
            // Falls into L86: [out] = r11+4, store in the bra delay slot.
            cpu.r[0] = mem.read_u32(cpu.r[15].wrapping_add(20)).unwrap_or(0);
            cpu.r[11] = cpu.r[11].wrapping_add(4); // add #4,r11
            let _ = mem.write_u32(cpu.r[0], cpu.r[11]); // bra delay: mov.l r11,@r0
        }
    } else {
        // L86: r0 = [r15+20]; r11 += 4; bra L92; delay [r0] = r11.
        cpu.r[0] = mem.read_u32(cpu.r[15].wrapping_add(20)).unwrap_or(0);
        cpu.r[11] = cpu.r[11].wrapping_add(4);
        let _ = mem.write_u32(cpu.r[0], cpu.r[11]);
    }
    // L92: pop pr,r11,r12,r13; rts; delay pop r14.
    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0); // lds.l @r15+,pr
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[11] = mem.read_u32(cpu.r[15]).unwrap_or(0); // mov.l @r15+,r11
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[12] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[13] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[14] = mem.read_u32(cpu.r[15]).unwrap_or(0); // rts delay: mov.l @r15+,r14
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}
