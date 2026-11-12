//! 0x8c023be0 — destroy an array of 0x20-byte handle containers.
//!
//! `param_1` (r4) points at `{ u32 pad; u32 count; u32 base; }`. When `base`
//! (offset +8) is non-NULL it calls the container destructor 0x8c021d70 on
//! each of `count` consecutive 0x20-byte records, then frees the array base
//! through the free thunk 0x8c01032c.
//!
//! ```text
//! 8c023be0  mov.l r14,@-r15     push r14
//! 8c023be2  mov.l r13,@-r15     push r13
//! 8c023be4  mov.l r12,@-r15     push r12
//! 8c023be6  mov.l r11,@-r15     push r11
//! 8c023be8  sts.l pr,@-r15      push pr
//! 8c023bea  mov   r4,r11        r11 = descriptor
//! 8c023bec  mov.l @(8,r11),r13  r13 = base
//! 8c023bee  tst   r13,r13
//! 8c023bf0  bt    0x8c023c18    base == NULL: return, T stays set
//! 8c023bf2  mov.l @(4,r11),r0   r0 = count
//! 8c023bf4  mov   r13,r14
//! 8c023bf6  shll2 r0
//! 8c023bf8  shll2 r0
//! 8c023bfa  shll  r0            r0 = count * 0x20
//! 8c023bfc  add   r0,r14        r14 = base + count*0x20
//! 8c023bfe  cmp/hi r13,r14      T = (end > base), unsigned
//! 8c023c00  bf    0x8c023c12    empty window: just free the base
//! 8c023c02  mov   #-1,r12
//! 8c023c04  mov.l 0x8c023c24,r0 r0 = 0x8c021d70 (container destructor)
//! 8c023c06  mov   r12,r5
//! 8c023c08  jsr   @r0
//! 8c023c0a  mov   r13,r4        delay: destroy(r13, -1)
//! 8c023c0c  add   #32,r13
//! 8c023c0e  cmp/hi r13,r14
//! 8c023c10  bt    0x8c023c04
//! 8c023c12  mov.l 0x8c023c28,r0 r0 = 0x8c01032c (free thunk)
//! 8c023c14  jsr   @r0
//! 8c023c16  mov.l @(8,r11),r4   delay: free(base), re-read
//! 8c023c18  lds.l @r15+,pr
//! 8c023c1a  mov.l @r15+,r11
//! 8c023c1c  mov.l @r15+,r12
//! 8c023c1e  mov.l @r15+,r13
//! 8c023c20  rts
//! 8c023c22  mov.l @r15+,r14     delay
//! ```
//!
//! The end pointer is computed with wrapping 32-bit arithmetic: a count whose
//! `count*0x20` wraps can leave `end <= base`, in which case the loop is
//! skipped and only the free runs. `cmp/hi` is unsigned (`>`, not `>=`), so
//! `end == base` (count 0 modulo 2^27) also skips the loop.
//!
//! Every path with a non-NULL base ends in the free call, whose result supplies
//! r0..r7 and T. The NULL early exit touches none of r0..r10 and leaves T set
//! by `tst`. r11..r14 and pr end at their entry values (restored from the
//! frame), and r15 is back to its entry value.

use sh4_oracle::{Cpu, Memory};

/// 0x8c023c24 holds 0x8c021d70, 0x8c023c28 holds 0x8c01032c.
const DESTROY_EACH_LITERAL: u32 = 0x8c023c24;
const FREE_THUNK_LITERAL: u32 = 0x8c023c28;

#[inline]
fn rd(mem: &Memory, addr: u32, fallback: u32) -> u32 {
    mem.read_u32(addr).unwrap_or(fallback)
}

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    // 0x8c023be0..0x8c023be8: push r14, r13, r12, r11, pr.
    for reg in [14usize, 13, 12, 11] {
        cpu.r[15] = cpu.r[15].wrapping_sub(4);
        let _ = mem.write_u32(cpu.r[15], cpu.r[reg]);
    }
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr);

    // 0x8c023bea: mov r4,r11
    cpu.r[11] = cpu.r[4];
    // 0x8c023bec: mov.l @(8,r11),r13
    cpu.r[13] = rd(mem, cpu.r[11].wrapping_add(8), 0);
    // 0x8c023bee: tst r13,r13
    cpu.set_t(cpu.r[13] == 0);
    // 0x8c023bf0: bt 0x8c023c18
    if cpu.r[13] != 0 {
        // 0x8c023bf2: mov.l @(4,r11),r0
        cpu.r[0] = rd(mem, cpu.r[11].wrapping_add(4), 0);
        // 0x8c023bf4: mov r13,r14
        cpu.r[14] = cpu.r[13];
        // 0x8c023bf6/0x8c023bf8/0x8c023bfa: shll2 ; shll2 ; shll
        cpu.r[0] = cpu.r[0].wrapping_shl(2).wrapping_shl(2).wrapping_shl(1);
        // 0x8c023bfc: add r0,r14
        cpu.r[14] = cpu.r[14].wrapping_add(cpu.r[0]);
        // 0x8c023bfe: cmp/hi r13,r14
        cpu.set_t(cpu.r[14] > cpu.r[13]);
        // 0x8c023c00: bf 0x8c023c12
        if cpu.r[14] > cpu.r[13] {
            // 0x8c023c02: mov #-1,r12
            cpu.r[12] = 0xffff_ffff;
            loop {
                // 0x8c023c04: mov.l 0x8c023c24,r0
                cpu.r[0] = rd(mem, DESTROY_EACH_LITERAL, 0x8c021d70);
                // 0x8c023c06: mov r12,r5
                cpu.r[5] = cpu.r[12];
                // 0x8c023c08: jsr @r0 ; 0x8c023c0a: mov r13,r4 (delay)
                cpu.pr = 0x8c023c0c;
                cpu.r[4] = cpu.r[13];
                crate::registry::p_8c021d70::run(cpu, mem);
                // 0x8c023c0c: add #32,r13
                cpu.r[13] = cpu.r[13].wrapping_add(32);
                // 0x8c023c0e: cmp/hi r13,r14
                cpu.set_t(cpu.r[14] > cpu.r[13]);
                // 0x8c023c10: bt 0x8c023c04
                if !cpu.t() {
                    break;
                }
            }
        }
        // 0x8c023c12: mov.l 0x8c023c28,r0
        cpu.r[0] = rd(mem, FREE_THUNK_LITERAL, 0x8c01032c);
        // 0x8c023c14: jsr @r0 ; 0x8c023c16: mov.l @(8,r11),r4 (delay)
        cpu.pr = 0x8c023c18;
        cpu.r[4] = rd(mem, cpu.r[11].wrapping_add(8), 0);
        crate::registry::p_8c01032c::run(cpu, mem);
    }

    // 0x8c023c18..0x8c023c22: pop pr, r11, r12, r13, r14.
    cpu.pr = rd(mem, cpu.r[15], 0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    for reg in [11usize, 12, 13, 14] {
        cpu.r[reg] = rd(mem, cpu.r[15], 0);
        cpu.r[15] = cpu.r[15].wrapping_add(4);
    }
}
