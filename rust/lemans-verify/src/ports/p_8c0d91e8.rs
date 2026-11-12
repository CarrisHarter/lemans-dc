//! 0x8c0d91e8 — commit a new selection index in a menu/list structure.
//!
//! Structure (`param_1` = r4):
//!
//! ```text
//! +0x40  void  **items    array of `count` item pointers
//! +0x44  u8     locked    1 => do not touch the items' highlight bits
//! +0x48  i32    selected  index written by this routine
//! +0x4c  i32    count     number of items
//! +0x58  f32    scroll    accumulator, advanced by the index delta
//! ```
//!
//! The routine inlines the same clamp twice: `param_2` (r5, signed) is pinned
//! to `0 ..= count-1` in single precision, truncated with `ftrc`, and the
//! first copy turns `selected - old_selected` into a float that is added to
//! the accumulator at +0x58; the second copy is stored at +0x48.
//!
//! ```text
//! lds r5,fpul ; fldi0 fr2 ; float fpul,fr0 ; fcmp/gt fr0,fr2
//! bf.s ...    ; float fpul,fr1            ; delay slot (fr1 = (count-1)f)
//! ```
//!
//! `fcmp/gt FRm,FRn` sets T when `FRn > FRm`, so the first `bf.s` takes the
//! "not negative" arm. The `bra`/delay pairs make the clamp `x < 0 ? 0 :
//! x > count-1 ? count-1 : x`; `count-1` is computed with a wrapping 32-bit
//! subtract and then converted, so a zero/negative count pins high values at a
//! negative bound.
//!
//! If +0x44 is 1 the routine stops after the two stores. Otherwise it walks
//! the item array and calls 0x8c05d274 twice per item — mask 2, then mask 1 —
//! with `r6 = (i == selected)`, which sets the two highlight bits on the
//! selected item and clears them everywhere else.
//!
//! The prologue pushes r10-r14 and pr; the epilogue pops all six, so every
//! pushed register (and r15) ends at its entry value. Those pushes are ignored
//! stack traffic in the verifier, and the callee is a leaf that never reads
//! the caller's frame, so the port leaves r10-r15 alone instead of emulating
//! the stack.
//!
//! Registers left behind outside the loop: r0 = +0x44 byte (early exit) or
//! +0x4c address (empty exit), r1 = +0x44 address, r3 = the truncated index.
//! Inside the loop r0/r1 are the callee's last results, r4 the last item
//! pointer, r5 = 1 and r6 the last `i == selected` flag. The final T comes
//! from `cmp/eq #1` (set) or from `cmp/pl`/`cmp/gt` falling through (clear).

use sh4_oracle::{Cpu, Memory};

/// One inlined clamp block. Loads `param_2` (r5) into `fr0`, `count-1` into
/// `fr1` and leaves the pinned value in `fr2`; `fpul` ends holding `count-1`.
fn clamp_into_fr(cpu: &mut Cpu, count: u32) {
    cpu.fpul = cpu.r[5]; // lds r5,fpul
    cpu.fr[2] = 0.0; // fldi0 fr2
    cpu.fr[0] = cpu.fpul as i32 as f32; // float fpul,fr0
    let negative = 0.0f32 > cpu.fr[0]; // fcmp/gt fr0,fr2
    cpu.set_t(negative);
    cpu.fpul = count.wrapping_sub(1); // mov #-1,r1 ; add r1,r0 ; lds r0,fpul
    cpu.fr[1] = cpu.fpul as i32 as f32; // delay slot: float fpul,fr1
    if !negative {
        let over = cpu.fr[0] > cpu.fr[1]; // fcmp/gt fr1,fr0
        cpu.set_t(over);
        cpu.fr[2] = if over { cpu.fr[1] } else { cpu.fr[0] };
    }
}

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let p = cpu.r[4];

    // mov.l @(76,r4),r0 ; mov #-1,r1 ; add r1,r0
    let count = mem.read_u32(p.wrapping_add(76)).unwrap_or(0);
    cpu.r[0] = count.wrapping_sub(1);

    // First clamp, then `selected - old` added to the +0x58 accumulator.
    clamp_into_fr(cpu, count);
    cpu.r[0] = mem.read_u32(p.wrapping_add(72)).unwrap_or(0); // mov.l @(72,r4),r0
    cpu.fpul = cpu.fr[2] as i32 as u32; // ftrc fr2,fpul
    cpu.fr[0] = mem.read_f32(p.wrapping_add(88)).unwrap_or(0.0); // fmov @r3,fr0
    cpu.r[1] = cpu.fpul; // sts fpul,r1
    cpu.r[1] = cpu.r[1].wrapping_sub(cpu.r[0]); // sub r0,r1
    cpu.fpul = cpu.r[1]; // lds r1,fpul
    cpu.fr[1] = cpu.fpul as i32 as f32; // float fpul,fr1
    cpu.fr[1] = cpu.fr[1] + cpu.fr[0]; // fadd fr0,fr1
    let _ = mem.write_f32(p.wrapping_add(88), cpu.fr[1]); // fmov fr1,@r3

    // Second clamp: the same value recomputed from the (unchanged) count and
    // stored as the new selection.
    let count = mem.read_u32(p.wrapping_add(76)).unwrap_or(0);
    cpu.r[0] = count.wrapping_sub(1);
    clamp_into_fr(cpu, count);
    cpu.fpul = cpu.fr[2] as i32 as u32; // ftrc fr2,fpul
    cpu.r[3] = cpu.fpul; // sts fpul,r3
    let _ = mem.write_u32(p.wrapping_add(72), cpu.r[3]); // mov.l r3,@(72,r10)

    // mov r10,r1 ; add #68,r1 ; mov.b @r1,r0 ; extu.b r0,r0 (twice)
    cpu.r[1] = p.wrapping_add(68);
    cpu.r[0] = mem.read_u8(p.wrapping_add(68)).unwrap_or(0) as u32;
    let locked = cpu.r[0] == 1; // cmp/eq #1,r0
    cpu.set_t(locked);
    if locked {
        return; // bt 0x8c0d92b4
    }

    // mov.l @(76,r4),r11 ; cmp/pl r11 ; bf.s with `mov #0,r12` in the delay
    let count = mem.read_u32(p.wrapping_add(76)).unwrap_or(0);
    cpu.r[0] = p.wrapping_add(76);
    let n = count as i32;
    cpu.set_t(n > 0);
    if n <= 0 {
        return;
    }

    let mut i: u32 = 0;
    loop {
        // mov.l @(64,r4),r0 ; mov r12,r14 ; shll2 r14 ; add r14,r0 ; mov.l @r0,r4
        let base = mem.read_u32(p.wrapping_add(64)).unwrap_or(0);
        let off = i << 2;
        cpu.r[4] = mem.read_u32(base.wrapping_add(off)).unwrap_or(0);
        cpu.r[5] = 2; // mov #2,r5
        cpu.r[1] = mem.read_u32(p.wrapping_add(72)).unwrap_or(0); // mov.l @(72,r4),r1
        cpu.r[0] = mem.read_u32(0x8c0d92c4).unwrap_or(0x8c05d274);
        let hit = i == cpu.r[1]; // cmp/eq r1,r12 ; movt r13
        cpu.r[6] = hit as u32; // delay slot: mov r13,r6
        crate::registry::p_8c05d274::run(cpu, mem);

        // Second call, mask 1: mov.l @(64,r4),r0 ; add r14,r0 ; mov.l @r0,r4
        let base = mem.read_u32(p.wrapping_add(64)).unwrap_or(0);
        cpu.r[4] = mem.read_u32(base.wrapping_add(off)).unwrap_or(0);
        cpu.r[6] = hit as u32; // mov r13,r6
        cpu.r[5] = 1; // delay slot: mov #1,r5
        cpu.r[0] = mem.read_u32(0x8c0d92c4).unwrap_or(0x8c05d274);
        crate::registry::p_8c05d274::run(cpu, mem);

        i = i.wrapping_add(1); // add #1,r12
        if !(n > i as i32) {
            // cmp/gt r12,r11 ; bt 0x8c0d927c
            break;
        }
    }
    // The `cmp/gt` that ends the loop leaves T clear.
    cpu.set_t(false);
}
