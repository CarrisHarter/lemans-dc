//! 0x8c10d5f0 — insertion sort over `[r4, r5)` stepping by 4.
//!
//! `r13` is the key element; the inner loop shifts elements right while the
//! signed key of the key element exceeds the signed key of `[r12]`. Keys come
//! from 0x8c10610c (`r0 = **r4`, double deref); the shift test is
//! `cmp/gt r0,r14` (`T = r14 > r0` signed). Outer test (`cmp/hi r8,r1`) and
//! inner test (`cmp/hi r9,r11`) are unsigned. `r5` (end) is spilled at `[r15]`
//! since `r10` is scratch. The shift block clobbers `r0`. Callee-saved
//! `r8`-`r14` and `pr` are restored; final `T` is always 0.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    // 8c10d5f0..8c10d5fc: push r14,r13,r12,r11,r10,r9,r8
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[14]);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[13]);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[12]);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[11]);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[10]);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[9]);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[8]);
    // 8c10d5fe: mov r4,r9 ; 8c10d600: sts.l pr,@-r15
    cpu.r[9] = cpu.r[4];
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr);
    // 8c10d602: add #-4,r15 ; 8c10d604: mov.l r5,@r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[5]);
    // 8c10d606: mov r9,r8 ; 8c10d608: add #4,r8
    cpu.r[8] = cpu.r[9];
    cpu.r[8] = cpu.r[8].wrapping_add(4);
    // 8c10d60a: mov r5,r1 ; 8c10d60c: cmp/hi r8,r1 ; 8c10d60e: bf 0x8c10d644
    cpu.r[1] = cpu.r[5];
    cpu.set_t(cpu.r[1] > cpu.r[8]);
    if cpu.t() {
        loop {
            // 8c10d610: mov.l @r8,r13 ; 612: mov r8,r12 ; 614: mov r8,r11
            cpu.r[13] = mem.read_u32(cpu.r[8]).unwrap_or(0);
            cpu.r[12] = cpu.r[8];
            cpu.r[11] = cpu.r[8];
            // 8c10d616: bra 0x8c10d622 ; 618: add #-4,r12 (delay)
            cpu.r[12] = cpu.r[12].wrapping_sub(4);
            loop {
                // 8c10d622: cmp/hi r9,r11 ; 624: bf 0x8c10d63a
                cpu.set_t(cpu.r[11] > cpu.r[9]);
                if !cpu.t() {
                    break;
                }
                // 8c10d626: mov.l 0x8c10d658,r0 — 0x8c10610c
                cpu.r[0] = mem.read_u32(0x8c10d658).unwrap_or(0);
                // 8c10d628: mov r13,r4 ; 62a: jsr @r0 ; 62c: mov.l @r12,r10 (delay)
                cpu.r[4] = cpu.r[13];
                cpu.pr = 0x8c10d62e;
                cpu.r[10] = mem.read_u32(cpu.r[12]).unwrap_or(0);
                crate::registry::p_8c10610c::run(cpu, mem);
                // 8c10d62e: mov r0,r14
                cpu.r[14] = cpu.r[0];
                // 8c10d630: mov.l 0x8c10d658,r0 ; 632: jsr @r0 ; 634: mov r10,r4 (delay)
                cpu.r[0] = mem.read_u32(0x8c10d658).unwrap_or(0);
                cpu.pr = 0x8c10d636;
                cpu.r[4] = cpu.r[10];
                crate::registry::p_8c10610c::run(cpu, mem);
                // 8c10d636: cmp/gt r0,r14 ; 638: bt 0x8c10d61a
                cpu.set_t((cpu.r[14] as i32) > (cpu.r[0] as i32));
                if !cpu.t() {
                    break;
                }
                // 8c10d61a: mov.l @r12,r0 ; 61c: mov.l r0,@r11
                cpu.r[0] = mem.read_u32(cpu.r[12]).unwrap_or(0);
                let _ = mem.write_u32(cpu.r[11], cpu.r[0]);
                // 8c10d61e: add #-4,r12 ; 620: add #-4,r11
                cpu.r[12] = cpu.r[12].wrapping_sub(4);
                cpu.r[11] = cpu.r[11].wrapping_sub(4);
            }
            // 8c10d63a: mov.l r13,@r11 ; 63c: add #4,r8
            let _ = mem.write_u32(cpu.r[11], cpu.r[13]);
            cpu.r[8] = cpu.r[8].wrapping_add(4);
            // 8c10d63e: mov.l @r15,r1 ; 640: cmp/hi r8,r1 ; 642: bt 0x8c10d610
            cpu.r[1] = mem.read_u32(cpu.r[15]).unwrap_or(0);
            cpu.set_t(cpu.r[1] > cpu.r[8]);
            if !cpu.t() {
                break;
            }
        }
    }
    // 8c10d644: add #4,r15 ; 646: lds.l @r15+,pr ; pop r8,r9,r10,r11,r12,r13 ; rts ; @r15+,r14
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[8] = mem.read_u32(cpu.r[15]).unwrap_or(0);
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
    cpu.r[14] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}
