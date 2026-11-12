//! 0x8c04396c — tear down an object: release its handles, free its arrays and
//! finally free the object itself when the caller's count is positive.
//!
//! ```text
//! 8c04396c  mov.l r14,@-r15
//! 8c04396e  mov.l r13,@-r15
//! 8c043970  mov.l r12,@-r15
//! 8c043972  mov.l r11,@-r15
//! 8c043974  mov.l r10,@-r15
//! 8c043976  mov   r5,r11           r11 = count arg (full word; tested via exts.w)
//! 8c043978  sts.l pr,@-r15
//! 8c04397a  mov   r4,r10           r10 = param_1
//! 8c04397c  tst   r10,r10
//! 8c04397e  bt    0x8c043a1e       NULL: return 0, T stays set
//! 8c043980  mov.l 0x8c043a30,r1     r1 = 0x8c23dbf8 (global slot address)
//! 8c043982  mov.l @r1,r0
//! 8c043984  cmp/eq r10,r0
//! 8c043986  bf    0x8c04398c
//! 8c043988  mov   #0,r0
//! 8c04398a  mov.l r0,@r1           clear the global when it names us
//! 8c04398c  mov.l 0x8c043a34,r0     r0 = 0x1a0
//! 8c04398e  mov.l @(r0,r10),r4
//! 8c043990  tst   r4,r4
//! 8c043992  bt    0x8c04399a       no inner pointer: skip the free
//! 8c043994  mov.l 0x8c043a38,r0     r0 = 0x8c0102cc (free thunk)
//! 8c043996  jsr   @r0              free([param_1+0x1a0])
//! 8c043998  nop
//! 8c04399a  mov.l 0x8c043a3c,r0     r0 = 0x190
//! 8c04399c  mov   r10,r4
//! 8c04399e  add   r0,r4
//! 8c0439a0  mov.l 0x8c043a40,r0     r0 = 0x8c018bd8 (release)
//! 8c0439a2  jsr   @r0              release(param_1+0x190, -1)
//! 8c0439a4  mov   #-1,r5           delay slot
//! 8c0439a6  mov.l 0x8c043a44,r0     r0 = 0x184
//! 8c0439a8  mov   r10,r4
//! 8c0439aa  add   r0,r4
//! 8c0439ac  tst   r4,r4
//! 8c0439ae  bt    0x8c0439c0       only reachable for an unmapped param_1
//! 8c0439b0  mov.l 0x8c043a48,r0     r0 = 0x8c043a80 (destroy circular list)
//! 8c0439b2  jsr   @r0              destroy(param_1+0x184)
//! 8c0439b4  nop
//! 8c0439b6  mov.l 0x8c043a44,r0     r0 = 0x184
//! 8c0439b8  mov.l @(r0,r10),r4      reload: the destroy rewrote the head word
//! 8c0439ba  mov.l 0x8c043a4c,r0     r0 = 0x8c01032c (free thunk)
//! 8c0439bc  jsr   @r0              free([param_1+0x184])
//! 8c0439be  nop
//! 8c0439c0  mov.l 0x8c043a50,r0     r0 = 0x180
//! 8c0439c2  mov   r10,r4
//! 8c0439c4  add   r0,r4
//! 8c0439c6  mov.l 0x8c043a40,r0     r0 = 0x8c018bd8 (release)
//! 8c0439c8  jsr   @r0              release(param_1+0x180, -1)
//! 8c0439ca  mov   #-1,r5           delay slot
//! 8c0439cc  tst   r10,r10
//! 8c0439ce  bt    0x8c043a12       dead: param_1 is known non-NULL here
//! 8c0439d0  mov.l 0x8c043a54,r0     r0 = 0x17c (array base field)
//! 8c0439d2  mov.l 0x8c043a58,r13    r13 = 0x178 (array count field)
//! 8c0439d4  mov   r0,r12           r12 = 0x17c
//! 8c0439d6  bra   0x8c0439e2
//! 8c0439d8  mov.l @(r0,r10),r14     delay: r14 = [param_1+0x17c]
//! 8c0439da  mov.l 0x8c043a4c,r0     loop body: r0 = 0x8c01032c
//! 8c0439dc  jsr   @r0              free(*r14)
//! 8c0439de  mov.l @r14,r4          delay slot
//! 8c0439e0  add   #4,r14           next element
//! 8c0439e2  mov   r12,r0           test: r0 = 0x17c
//! 8c0439e4  mov.l @(r0,r10),r1      r1 = [param_1+0x17c] (reloaded every pass)
//! 8c0439e6  mov   r13,r0
//! 8c0439e8  mov.l @(r0,r10),r0      r0 = [param_1+0x178]
//! 8c0439ea  shll2 r0
//! 8c0439ec  add   r0,r1            bound = base + count*4
//! 8c0439ee  cmp/hi r14,r1
//! 8c0439f0  bt    0x8c0439da      more elements: keep freeing
//! 8c0439f2  mov.l 0x8c043a5c,r0     r0 = 0x174
//! 8c0439f4  mov   r10,r4
//! 8c0439f6  add   r0,r4
//! 8c0439f8  tst   r4,r4
//! 8c0439fa  bt    0x8c043a12       only reachable for an unmapped param_1
//! 8c0439fc  tst   r4,r4
//! 8c0439fe  bt    0x8c043a12       duplicate guard
//! 8c043a00  mov.l 0x8c043a60,r0     r0 = 0x8c043a68 (release block)
//! 8c043a02  jsr   @r0              release_block(param_1+0x174)
//! 8c043a04  nop
//! 8c043a06  mov.l 0x8c043a5c,r0     r0 = 0x174
//! 8c043a08  mov   r10,r4
//! 8c043a0a  add   r0,r4
//! 8c043a0c  mov.l 0x8c043a64,r0     r0 = 0x8c01e460 (free-if-positive)
//! 8c043a0e  jsr   @r0              free_if_positive(param_1+0x174, -1)
//! 8c043a10  mov   #-1,r5           delay slot
//! 8c043a12  exts.w r11,r0           r0 = sext16(count arg)
//! 8c043a14  cmp/pl r0
//! 8c043a16  bf    0x8c043a1e       count low half <= 0: keep the object
//! 8c043a18  mov.l 0x8c043a4c,r0     r0 = 0x8c01032c (free thunk)
//! 8c043a1a  jsr   @r0              free(param_1)
//! 8c043a1c  mov   r10,r4           delay slot
//! 8c043a1e  mov   r10,r0           return param_1 (0 on the NULL path)
//! 8c043a20  lds.l @r15+,pr
//! 8c043a22  mov.l @r15+,r10
//! 8c043a24  mov.l @r15+,r11
//! 8c043a26  mov.l @r15+,r12
//! 8c043a28  mov.l @r15+,r13
//! 8c043a2a  rts
//! 8c043a2c  mov.l @r15+,r14       delay slot
//! ```
//!
//! The null test at 0x8c04397e jumps straight past the `exts.w` block, so a
//! NULL argument leaves `T` set by that `tst` and never touches the count.
//! The two "field address is zero" branches (0x8c0439ae and 0x8c0439fa) can
//! only be taken when `param_1` is one of the two wrapping pointers
//! 0xfffffe7c / 0xfffffe8c, which fault on the earlier field loads, so they are
//! kept for fidelity but unreachable with mapped memory.
//!
//! The element walk reloads both the base pointer (0x17c) and the count (0x178)
//! on every pass, and compares the recomputed bound against the running cursor
//! with the *unsigned* `cmp/hi`; `shll2` leaves `T` alone. Each element is
//! handed to the free thunk through the delay-slot load, so the cursor only
//! advances after the call returns. The destroy call at 0x8c0439b2 rewrites the
//! head word, which is why the address for the following free is reloaded from
//! the field rather than reused.
//!
//! All six callees are `jsr` targets; the delay slots load `r5`/`r4`/nothing.
//! `r10`–`r14` and `pr` are restored from the frame, so they end at their entry
//! values; `r0` ends as `param_1` and `T` is the last flag-setting instruction
//! on the path taken (the final `cmp/pl` when no free runs, otherwise the
//! callee's).

use sh4_oracle::{Cpu, Memory};

/// Literal addresses in this function's own pool.
const GLOBAL_SLOT_LITERAL: u32 = 0x8c043a30; // holds 0x8c23dbf8
const FREE_THUNK_1: u32 = 0x8c043a38; // holds 0x8c0102cc
const RELEASE: u32 = 0x8c043a40; // holds 0x8c018bd8
const DESTROY_LIST: u32 = 0x8c043a48; // holds 0x8c043a80
const FREE_THUNK_2: u32 = 0x8c043a4c; // holds 0x8c01032c
const RELEASE_BLOCK: u32 = 0x8c043a60; // holds 0x8c043a68
const FREE_IF_POSITIVE: u32 = 0x8c043a64; // holds 0x8c01e460

/// Field offsets inside the object, from the constant pool.
const OFF_INNER: u32 = 0x1a0;
const OFF_HANDLE_A: u32 = 0x190;
const OFF_LIST: u32 = 0x184;
const OFF_HANDLE_B: u32 = 0x180;
const OFF_ARRAY_BASE: u32 = 0x17c;
const OFF_ARRAY_COUNT: u32 = 0x178;
const OFF_HANDLE_C: u32 = 0x174;

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    // 0x8c04396c: mov.l r14,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[14]);
    // 0x8c04396e: mov.l r13,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[13]);
    // 0x8c043970: mov.l r12,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[12]);
    // 0x8c043972: mov.l r11,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[11]);
    // 0x8c043974: mov.l r10,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[10]);
    // 0x8c043976: mov r5,r11
    cpu.r[11] = cpu.r[5];
    // 0x8c043978: sts.l pr,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr);
    // 0x8c04397a: mov r4,r10
    cpu.r[10] = cpu.r[4];
    // 0x8c04397c: tst r10,r10 ; 0x8c04397e: bt 0x8c043a1e
    cpu.set_t(cpu.r[10] == 0);
    if cpu.r[10] != 0 {
        // 0x8c043980: mov.l 0x8c043a30,r1
        cpu.r[1] = mem.read_u32(GLOBAL_SLOT_LITERAL).unwrap_or(0x8c23dbf8);
        // 0x8c043982: mov.l @r1,r0
        cpu.r[0] = mem.read_u32(cpu.r[1]).unwrap_or(0);
        // 0x8c043984: cmp/eq r10,r0 ; 0x8c043986: bf 0x8c04398c
        cpu.set_t(cpu.r[0] == cpu.r[10]);
        if cpu.r[0] == cpu.r[10] {
            // 0x8c043988: mov #0,r0
            cpu.r[0] = 0;
            // 0x8c04398a: mov.l r0,@r1
            let _ = mem.write_u32(cpu.r[1], cpu.r[0]);
        }
        // 0x8c04398c: mov.l 0x8c043a34,r0
        cpu.r[0] = mem.read_u32(0x8c043a34).unwrap_or(OFF_INNER);
        // 0x8c04398e: mov.l @(r0,r10),r4
        cpu.r[4] = mem.read_u32(cpu.r[0].wrapping_add(cpu.r[10])).unwrap_or(0);
        // 0x8c043990: tst r4,r4 ; 0x8c043992: bt 0x8c04399a
        cpu.set_t(cpu.r[4] == 0);
        if cpu.r[4] != 0 {
            // 0x8c043994: mov.l 0x8c043a38,r0
            cpu.r[0] = mem.read_u32(FREE_THUNK_1).unwrap_or(0x8c0102cc);
            // 0x8c043996: jsr @r0 ; 0x8c043998: nop
            cpu.pr = 0x8c04399a;
            crate::registry::p_8c0102cc::run(cpu, mem);
        }
        // 0x8c04399a: mov.l 0x8c043a3c,r0
        cpu.r[0] = mem.read_u32(0x8c043a3c).unwrap_or(OFF_HANDLE_A);
        // 0x8c04399c: mov r10,r4 ; 0x8c04399e: add r0,r4
        cpu.r[4] = cpu.r[10].wrapping_add(cpu.r[0]);
        // 0x8c0439a0: mov.l 0x8c043a40,r0
        cpu.r[0] = mem.read_u32(RELEASE).unwrap_or(0x8c018bd8);
        // 0x8c0439a2: jsr @r0 ; 0x8c0439a4: mov #-1,r5 (delay slot)
        cpu.pr = 0x8c0439a6;
        cpu.r[5] = 0xffff_ffff;
        crate::registry::p_8c018bd8::run(cpu, mem);
        // 0x8c0439a6: mov.l 0x8c043a44,r0
        cpu.r[0] = mem.read_u32(0x8c043a44).unwrap_or(OFF_LIST);
        // 0x8c0439a8: mov r10,r4 ; 0x8c0439aa: add r0,r4
        cpu.r[4] = cpu.r[10].wrapping_add(cpu.r[0]);
        // 0x8c0439ac: tst r4,r4 ; 0x8c0439ae: bt 0x8c0439c0
        cpu.set_t(cpu.r[4] == 0);
        if cpu.r[4] != 0 {
            // 0x8c0439b0: mov.l 0x8c043a48,r0
            cpu.r[0] = mem.read_u32(DESTROY_LIST).unwrap_or(0x8c043a80);
            // 0x8c0439b2: jsr @r0 ; 0x8c0439b4: nop
            cpu.pr = 0x8c0439b6;
            crate::registry::p_8c043a80::run(cpu, mem);
            // 0x8c0439b6: mov.l 0x8c043a44,r0
            cpu.r[0] = mem.read_u32(0x8c043a44).unwrap_or(OFF_LIST);
            // 0x8c0439b8: mov.l @(r0,r10),r4
            cpu.r[4] = mem.read_u32(cpu.r[0].wrapping_add(cpu.r[10])).unwrap_or(0);
            // 0x8c0439ba: mov.l 0x8c043a4c,r0
            cpu.r[0] = mem.read_u32(FREE_THUNK_2).unwrap_or(0x8c01032c);
            // 0x8c0439bc: jsr @r0 ; 0x8c0439be: nop
            cpu.pr = 0x8c0439be;
            crate::registry::p_8c01032c::run(cpu, mem);
        }
        // 0x8c0439c0: mov.l 0x8c043a50,r0
        cpu.r[0] = mem.read_u32(0x8c043a50).unwrap_or(OFF_HANDLE_B);
        // 0x8c0439c2: mov r10,r4 ; 0x8c0439c4: add r0,r4
        cpu.r[4] = cpu.r[10].wrapping_add(cpu.r[0]);
        // 0x8c0439c6: mov.l 0x8c043a40,r0
        cpu.r[0] = mem.read_u32(RELEASE).unwrap_or(0x8c018bd8);
        // 0x8c0439c8: jsr @r0 ; 0x8c0439ca: mov #-1,r5 (delay slot)
        cpu.pr = 0x8c0439cc;
        cpu.r[5] = 0xffff_ffff;
        crate::registry::p_8c018bd8::run(cpu, mem);
        // 0x8c0439cc: tst r10,r10 ; 0x8c0439ce: bt 0x8c043a12
        cpu.set_t(cpu.r[10] == 0);
        if cpu.r[10] != 0 {
            // 0x8c0439d0: mov.l 0x8c043a54,r0
            cpu.r[0] = mem.read_u32(0x8c043a54).unwrap_or(OFF_ARRAY_BASE);
            // 0x8c0439d2: mov.l 0x8c043a58,r13
            cpu.r[13] = mem.read_u32(0x8c043a58).unwrap_or(OFF_ARRAY_COUNT);
            // 0x8c0439d4: mov r0,r12
            cpu.r[12] = cpu.r[0];
            // 0x8c0439d6: bra 0x8c0439e2 ; 0x8c0439d8: mov.l @(r0,r10),r14 (delay)
            cpu.r[14] = mem.read_u32(cpu.r[0].wrapping_add(cpu.r[10])).unwrap_or(0);
            loop {
                // 0x8c0439e2: mov r12,r0
                cpu.r[0] = cpu.r[12];
                // 0x8c0439e4: mov.l @(r0,r10),r1
                cpu.r[1] = mem.read_u32(cpu.r[0].wrapping_add(cpu.r[10])).unwrap_or(0);
                // 0x8c0439e6: mov r13,r0
                cpu.r[0] = cpu.r[13];
                // 0x8c0439e8: mov.l @(r0,r10),r0
                cpu.r[0] = mem.read_u32(cpu.r[0].wrapping_add(cpu.r[10])).unwrap_or(0);
                // 0x8c0439ea: shll2 r0 (does not touch T)
                cpu.r[0] <<= 2;
                // 0x8c0439ec: add r0,r1
                cpu.r[1] = cpu.r[1].wrapping_add(cpu.r[0]);
                // 0x8c0439ee: cmp/hi r14,r1 ; 0x8c0439f0: bt 0x8c0439da
                cpu.set_t(cpu.r[1] > cpu.r[14]);
                if cpu.r[1] <= cpu.r[14] {
                    break;
                }
                // 0x8c0439da: mov.l 0x8c043a4c,r0
                cpu.r[0] = mem.read_u32(FREE_THUNK_2).unwrap_or(0x8c01032c);
                // 0x8c0439dc: jsr @r0 ; 0x8c0439de: mov.l @r14,r4 (delay slot)
                cpu.pr = 0x8c0439e0;
                cpu.r[4] = mem.read_u32(cpu.r[14]).unwrap_or(0);
                crate::registry::p_8c01032c::run(cpu, mem);
                // 0x8c0439e0: add #4,r14
                cpu.r[14] = cpu.r[14].wrapping_add(4);
            }
            // 0x8c0439f2: mov.l 0x8c043a5c,r0
            cpu.r[0] = mem.read_u32(0x8c043a5c).unwrap_or(OFF_HANDLE_C);
            // 0x8c0439f4: mov r10,r4 ; 0x8c0439f6: add r0,r4
            cpu.r[4] = cpu.r[10].wrapping_add(cpu.r[0]);
            // 0x8c0439f8: tst r4,r4 ; 0x8c0439fa: bt 0x8c043a12
            cpu.set_t(cpu.r[4] == 0);
            if cpu.r[4] != 0 {
                // 0x8c0439fc: tst r4,r4 ; 0x8c0439fe: bt 0x8c043a12
                cpu.set_t(cpu.r[4] == 0);
                if cpu.r[4] != 0 {
                    // 0x8c043a00: mov.l 0x8c043a60,r0
                    cpu.r[0] = mem.read_u32(RELEASE_BLOCK).unwrap_or(0x8c043a68);
                    // 0x8c043a02: jsr @r0 ; 0x8c043a04: nop
                    cpu.pr = 0x8c043a06;
                    crate::registry::p_8c043a68::run(cpu, mem);
                    // 0x8c043a06: mov.l 0x8c043a5c,r0
                    cpu.r[0] = mem.read_u32(0x8c043a5c).unwrap_or(OFF_HANDLE_C);
                    // 0x8c043a08: mov r10,r4 ; 0x8c043a0a: add r0,r4
                    cpu.r[4] = cpu.r[10].wrapping_add(cpu.r[0]);
                    // 0x8c043a0c: mov.l 0x8c043a64,r0
                    cpu.r[0] = mem.read_u32(FREE_IF_POSITIVE).unwrap_or(0x8c01e460);
                    // 0x8c043a0e: jsr @r0 ; 0x8c043a10: mov #-1,r5 (delay slot)
                    cpu.pr = 0x8c043a12;
                    cpu.r[5] = 0xffff_ffff;
                    crate::registry::p_8c01e460::run(cpu, mem);
                }
            }
        }
        // 0x8c043a12: exts.w r11,r0
        cpu.r[0] = cpu.r[11] as i16 as i32 as u32;
        // 0x8c043a14: cmp/pl r0 ; 0x8c043a16: bf 0x8c043a1e
        cpu.set_t((cpu.r[0] as i32) > 0);
        if (cpu.r[0] as i32) > 0 {
            // 0x8c043a18: mov.l 0x8c043a4c,r0
            cpu.r[0] = mem.read_u32(FREE_THUNK_2).unwrap_or(0x8c01032c);
            // 0x8c043a1a: jsr @r0 ; 0x8c043a1c: mov r10,r4 (delay slot)
            cpu.pr = 0x8c043a1e;
            cpu.r[4] = cpu.r[10];
            crate::registry::p_8c01032c::run(cpu, mem);
        }
    }
    // 0x8c043a1e: mov r10,r0
    cpu.r[0] = cpu.r[10];
    // 0x8c043a20: lds.l @r15+,pr
    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    // 0x8c043a22: mov.l @r15+,r10
    cpu.r[10] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    // 0x8c043a24: mov.l @r15+,r11
    cpu.r[11] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    // 0x8c043a26: mov.l @r15+,r12
    cpu.r[12] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    // 0x8c043a28: mov.l @r15+,r13
    cpu.r[13] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    // 0x8c043a2a: rts ; 0x8c043a2c: mov.l @r15+,r14 (delay slot)
    cpu.r[14] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}
