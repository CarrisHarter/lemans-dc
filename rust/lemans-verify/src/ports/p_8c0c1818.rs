//! 0x8c0c1818 — tear down a heap-backed object and optionally free it.
//!
//! ```text
//! mov.l r14,@-r15        ; save caller r14 (ENTRY value, before r14 is reused)
//! mov.l r13,@-r15        ; save caller r13
//! mov   r5,r14           ; r14 = second argument (interpreted as i16 later)
//! mov.l r12,@-r15        ; save caller r12
//! mov   r4,r13           ; r13 = object
//! sts.l pr,@-r15
//! tst   r13,r13 ; bt  0x8c0c1862            ; NULL object: just return it
//! mov.l 0x8c0c1870,r1    ; 0x8c27a238
//! mov   #0,r0
//! mov.l r0,@r1           ; *0x8c27a238 = 0
//! mov.l @(36,r13),r12    ; r12 = object->field_0x24
//! tst   r12,r12 ; bt  0x8c0c1856            ; no block: skip to the size test
//! mov   r12,r4 ; add #44,r4
//! tst   r4,r4 ; bt 0x8c0c1850
//! tst   r4,r4 ; bt 0x8c0c1850               ; (the twin test the compiler kept)
//! mov.l 0x8c0c1874,r0 ; jsr @r0             ; 0x8c043a68(r12+0x2c)
//! mov.l 0x8c0c1878,r0 ; mov r12,r4 ; add #44,r4 ; jsr @r0 ; mov #-1,r5
//!                                              ; 0x8c01e460(r12+0x2c, -1)
//! mov.l 0x8c0c187c,r0 ; jsr @r0 ; mov r12,r4  ; free(r12)
//! 0x8c0c1856: exts.w r14,r0 ; cmp/pl r0 ; bf 0x8c0c1862
//! mov.l 0x8c0c187c,r0 ; jsr @r0 ; mov r13,r4  ; if (sext16(r5) > 0) free(r13)
//! 0x8c0c1862: mov r13,r0
//! lds.l @r15+,pr ; mov.l @r15+,r12 ; mov.l @r15+,r13
//! rts ; mov.l @r15+,r14 (delay slot)
//! ```
//!
//! The three pushes happen *before* the corresponding registers are reused, so
//! the epilogue restores the entry values of r12, r13 and r14 — r14's body copy
//! of r5 is discarded. r0 always ends as the object pointer (r13), overwriting
//! whatever the callees returned. T is left by the last flag-setting
//! instruction: `tst r13,r13` on the NULL-object path, `cmp/pl` when the outer
//! free is skipped, and whichever callee ran last on the other paths.
//!
//! `mov #-1,r5` sits in the delay slot of the 0x8c01e460 call, so that callee
//! always sees -1 and its inner `exts.w` test never frees. The last free is
//! guarded by `exts.w r14`, i.e. only the low 16 bits of the caller's r5 matter.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    // 0x8c0c1818: mov.l r14,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[14]);
    // 0x8c0c181a: mov.l r13,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[13]);
    // 0x8c0c181c: mov r5,r14
    cpu.r[14] = cpu.r[5];
    // 0x8c0c181e: mov.l r12,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[12]);
    // 0x8c0c1820: mov r4,r13
    cpu.r[13] = cpu.r[4];
    // 0x8c0c1822: sts.l pr,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr);
    // 0x8c0c1824: tst r13,r13 ; 0x8c0c1826: bt 0x8c0c1862
    cpu.set_t(cpu.r[13] == 0);
    if cpu.r[13] != 0 {
        // 0x8c0c1828: mov.l 0x8c0c1870,r1 — global pointer 0x8c27a238
        cpu.r[1] = mem.read_u32(0x8c0c1870).unwrap_or(0x8c27a238);
        // 0x8c0c182a: mov #0,r0 ; 0x8c0c182c: mov.l r0,@r1
        cpu.r[0] = 0;
        let _ = mem.write_u32(cpu.r[1], cpu.r[0]);
        // 0x8c0c182e: mov.l @(36,r13),r12
        cpu.r[12] = mem.read_u32(cpu.r[13].wrapping_add(36)).unwrap_or(0);
        // 0x8c0c1830: tst r12,r12 ; 0x8c0c1832: bt 0x8c0c1856
        cpu.set_t(cpu.r[12] == 0);
        if cpu.r[12] != 0 {
            // 0x8c0c1834: mov r12,r4 ; 0x8c0c1836: add #44,r4
            cpu.r[4] = cpu.r[12].wrapping_add(44);
            // 0x8c0c1838: tst r4,r4 ; 0x8c0c183a: bt 0x8c0c1850
            cpu.set_t(cpu.r[4] == 0);
            if cpu.r[4] != 0 {
                // 0x8c0c183c: tst r4,r4 ; 0x8c0c183e: bt 0x8c0c1850
                cpu.set_t(cpu.r[4] == 0);
                if cpu.r[4] != 0 {
                    // 0x8c0c1840: mov.l 0x8c0c1874,r0 — 0x8c043a68
                    cpu.r[0] = mem.read_u32(0x8c0c1874).unwrap_or(0x8c043a68);
                    // 0x8c0c1842: jsr @r0 ; 0x8c0c1844: nop
                    cpu.pr = 0x8c0c1846;
                    crate::registry::p_8c043a68::run(cpu, mem);
                    // 0x8c0c1846: mov.l 0x8c0c1878,r0 — 0x8c01e460
                    cpu.r[0] = mem.read_u32(0x8c0c1878).unwrap_or(0x8c01e460);
                    // 0x8c0c1848: mov r12,r4 ; 0x8c0c184a: add #44,r4
                    cpu.r[4] = cpu.r[12].wrapping_add(44);
                    // 0x8c0c184c: jsr @r0 ; 0x8c0c184e: mov #-1,r5 (delay slot)
                    cpu.r[5] = 0xffff_ffff;
                    cpu.pr = 0x8c0c1850;
                    crate::registry::p_8c01e460::run(cpu, mem);
                }
                // 0x8c0c1850: mov.l 0x8c0c187c,r0 — 0x8c01032c (free thunk)
                cpu.r[0] = mem.read_u32(0x8c0c187c).unwrap_or(0x8c01032c);
                // 0x8c0c1852: jsr @r0 ; 0x8c0c1854: mov r12,r4 (delay slot)
                cpu.r[4] = cpu.r[12];
                cpu.pr = 0x8c0c1856;
                crate::registry::p_8c01032c::run(cpu, mem);
            }
        }
        // 0x8c0c1856: exts.w r14,r0
        cpu.r[0] = cpu.r[14] as i16 as i32 as u32;
        // 0x8c0c1858: cmp/pl r0 ; 0x8c0c185a: bf 0x8c0c1862
        cpu.set_t((cpu.r[0] as i32) > 0);
        if (cpu.r[0] as i32) > 0 {
            // 0x8c0c185c: mov.l 0x8c0c187c,r0 — 0x8c01032c (free thunk)
            cpu.r[0] = mem.read_u32(0x8c0c187c).unwrap_or(0x8c01032c);
            // 0x8c0c185e: jsr @r0 ; 0x8c0c1860: mov r13,r4 (delay slot)
            cpu.r[4] = cpu.r[13];
            cpu.pr = 0x8c0c1862;
            crate::registry::p_8c01032c::run(cpu, mem);
        }
    }
    // 0x8c0c1862: mov r13,r0
    cpu.r[0] = cpu.r[13];
    // 0x8c0c1864: lds.l @r15+,pr
    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    // 0x8c0c1866: mov.l @r15+,r12
    cpu.r[12] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    // 0x8c0c1868: mov.l @r15+,r13
    cpu.r[13] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    // 0x8c0c186a: rts ; 0x8c0c186c: mov.l @r15+,r14 (delay slot)
    cpu.r[14] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}
