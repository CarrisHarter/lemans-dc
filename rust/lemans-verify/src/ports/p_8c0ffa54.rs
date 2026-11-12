//! 0x8c0ffa54 — scan a counted byte buffer backwards for `(char)r5`,
//! starting at index `r6`, and return the index or -1.
//!
//! `p = *(u32 *)r4` is the object; `count = *(u32 *)p` and
//! `base = *(u32 *)(p + 0xc)` describe the buffer. The callee 0x8c01ec2c is
//! just `return *(u32 *)(arg + 0xc);`, so every call yields the same `base`.
//! The search limit is clamped to `count - 1` with an unsigned `cmp/hi`, and
//! the needle is re-sign-extended from the low byte of `r12` (`exts.b`).
//!
//! ```text
//! mov.l @r11,r4 ; mov.l @r4,r1    ; p, count
//! tst r1,r1 ; bf 0x8c0ffa70       ; count == 0 -> return -1 (T = 1 from tst)
//! 0x8c0ffa70: r1 = count-1
//! r14 = min(r14, r1)              ; cmp/hi r1,r14 clamps the start index
//! r13 = call(p)                   ; base
//! r14 = r14 + 1 ; r13 += r14      ; pcVar5 = base + index + 1
//! r0 = call(p) ; cmp/eq r0,r13    ; never true: index+1 cannot be 0
//! 0x8c0ffa90: r14 = r13 - 1       ; candidate position
//! r1 = (i8)byte[r14] ; r0 = (i8)r12 ; cmp/eq r0,r1
//! bt hit: r0 = base again; r13 -= r0 + 1; return r13   (index)
//! otherwise r13 = r14; r0 = call(p); cmp/eq r0,r14
//! bf 0x8c0ffa90                   ; keep scanning while r14 != base
//! 0x8c0ffab6: return -1           ; byte at base was checked first
//! ```
//! All exits leave T = 1: the count == 0 path from `tst`, the pre-check from
//! its true `cmp/eq`, hits from the `cmp/eq` that matched, and the exhausted
//! loop from the true `cmp/eq` against `base`. The last checked byte stays in
//! `r1` (sign-extended) on every exit, `r4 = p` on every exit, and the
//! callee-saved `r11`-`r14` are restored from the prologue's stack slots.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    // 0x8c0ffa54..0x8c0ffa62: prologue
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[14]); // mov.l r14,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[13]); // mov.l r13,@-r15
    cpu.r[14] = cpu.r[6]; // mov r6,r14
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[12]); // mov.l r12,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[11]); // mov.l r11,@-r15
    cpu.r[12] = cpu.r[5]; // mov r5,r12
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr); // sts.l pr,@-r15
    cpu.r[11] = cpu.r[4]; // mov r4,r11

    cpu.r[4] = mem.read_u32(cpu.r[11]).unwrap_or(0); // mov.l @r11,r4 (p)
    cpu.r[1] = mem.read_u32(cpu.r[4]).unwrap_or(0); // mov.l @r4,r1 (count)
    cpu.set_t(cpu.r[1] == 0); // tst r1,r1
    if cpu.r[1] == 0 {
        cpu.r[0] = 0xFFFF_FFFF; // bra 0x8c0ffab8; delay slot: mov #-1,r0
        epilogue(cpu, mem);
        return;
    }

    // 0x8c0ffa70..0x8c0ffa78: clamp the start index to count - 1
    cpu.r[0] = 0xFFFF_FFFF; // mov #-1,r0
    cpu.r[1] = cpu.r[1].wrapping_add(cpu.r[0]); // add r0,r1
    cpu.set_t(cpu.r[14] > cpu.r[1]); // cmp/hi r1,r14 (unsigned)
    if cpu.r[14] > cpu.r[1] {
        cpu.r[14] = cpu.r[1]; // mov r1,r14
    }

    // 0x8c0ffa7a..0x8c0ffa80: first 0x8c01ec2c call
    cpu.r[0] = 0x8c01_ec2c; // mov.l 0x8c0ffac4,r0
    cpu.pr = 0x8c0f_fa80; // jsr @r0 (delay slot: nop)
    crate::registry::p_8c01ec2c::run(cpu, mem);
    cpu.r[13] = cpu.r[0]; // mov r0,r13 (base)

    // 0x8c0ffa82..0x8c0ffa8a: second call, delay slot adds the index
    cpu.r[4] = mem.read_u32(cpu.r[11]).unwrap_or(0); // mov.l @r11,r4
    cpu.r[14] = cpu.r[14].wrapping_add(1); // add #1,r14
    cpu.r[0] = 0x8c01_ec2c; // mov.l 0x8c0ffac4,r0
    cpu.pr = 0x8c0f_fa8c; // jsr @r0
    cpu.r[13] = cpu.r[13].wrapping_add(cpu.r[14]); // delay slot: add r14,r13
    crate::registry::p_8c01ec2c::run(cpu, mem);
    cpu.set_t(cpu.r[13] == cpu.r[0]); // cmp/eq r0,r13
    if cpu.r[13] != cpu.r[0] {
        // 0x8c0ffa90..0x8c0ffab4: backward scan
        loop {
            cpu.r[14] = cpu.r[13]; // mov r13,r14
            cpu.r[14] = cpu.r[14].wrapping_sub(1); // add #-1,r14
            cpu.r[1] = mem.read_u8(cpu.r[14]).unwrap_or(0) as i8 as i32 as u32; // mov.b @r14,r1
            cpu.r[0] = cpu.r[12] as u8 as i8 as i32 as u32; // exts.b r12,r0
            cpu.r[1] = cpu.r[1] as u8 as i8 as i32 as u32; // exts.b r1,r1
            cpu.set_t(cpu.r[1] == cpu.r[0]); // cmp/eq r0,r1
            if cpu.r[1] == cpu.r[0] {
                // 0x8c0ffa9e..0x8c0ffaa8: hit, return position - base
                cpu.r[0] = mem.read_u32(cpu.r[11]).unwrap_or(0); // mov.l @r11,r0
                cpu.r[0] = mem.read_u32(cpu.r[0].wrapping_add(12)).unwrap_or(0); // mov.l @(12,r0),r0
                cpu.r[13] = cpu.r[13].wrapping_sub(cpu.r[0]); // sub r0,r13
                cpu.r[13] = cpu.r[13].wrapping_sub(1); // add #-1,r13
                cpu.r[0] = cpu.r[13]; // bra 0x8c0ffab8; delay slot: mov r13,r0
                epilogue(cpu, mem);
                return;
            }
            // 0x8c0ffaaa..0x8c0ffab0: third call; delay slot records the
            // candidate as the next pcVar5
            cpu.r[4] = mem.read_u32(cpu.r[11]).unwrap_or(0); // mov.l @r11,r4
            cpu.r[0] = 0x8c01_ec2c; // mov.l 0x8c0ffac4,r0
            cpu.pr = 0x8c0f_fab2; // jsr @r0
            cpu.r[13] = cpu.r[14]; // delay slot: mov r14,r13
            crate::registry::p_8c01ec2c::run(cpu, mem);
            cpu.set_t(cpu.r[14] == cpu.r[0]); // cmp/eq r0,r14
            if cpu.r[14] == cpu.r[0] {
                break; // bf not taken: exhausted at base
            }
        }
    }

    cpu.r[0] = 0xFFFF_FFFF; // 0x8c0ffab6: mov #-1,r0
    epilogue(cpu, mem);
}

fn epilogue(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0); // lds.l @r15+,pr
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[11] = mem.read_u32(cpu.r[15]).unwrap_or(0); // mov.l @r15+,r11
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[12] = mem.read_u32(cpu.r[15]).unwrap_or(0); // mov.l @r15+,r12
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[13] = mem.read_u32(cpu.r[15]).unwrap_or(0); // mov.l @r15+,r13
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[14] = mem.read_u32(cpu.r[15]).unwrap_or(0); // delay slot of rts
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}
