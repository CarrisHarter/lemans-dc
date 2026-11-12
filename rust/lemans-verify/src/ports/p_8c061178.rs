//! 0x8c061178 — release an object: clear the registered instance, dispose of
//! the embedded block, then optionally free the object itself.
//!
//! ```text
//! mov.l r14,@-r15 ; mov.l r13,@-r15 ; mov r4,r14 ; sts.l pr,@-r15
//! mov   r5,r13
//! tst   r14,r14 ; bt 0x8c0611a8
//! mov.l 0x8c0611b4,r1 (0x8c23dea0)   ; r1 = &registered instance
//! mov.l @r1,r0 ; cmp/eq r14,r0 ; bf 0x8c061192
//! mov   #0,r0 ; mov.l r0,@r1          ; clear the registry when it holds us
//! 0x8c061192: mov.l 0x8c0611b8,r0 (0x8c061064)
//! mov   r14,r4 ; add #16,r4
//! jsr   @r0 ; mov #-1,r5              ; delay slot
//! exts.w r13,r0 ; cmp/pl r0 ; bf 0x8c0611a8
//! mov.l 0x8c0611bc,r0 (0x8c01032c, free thunk)
//! jsr   @r0 ; mov r14,r4              ; delay slot
//! 0x8c0611a8: mov r14,r0
//! lds.l @r15+,pr ; mov.l @r15+,r13 ; rts ; mov.l @r15+,r14 (delay slot)
//! ```
//!
//! Both calls share the preserved `r13` as the free decision: the original
//! halfword is sign-extended (`exts.w`), so `0x0000ffff` is non-positive and
//! skips the free while `0xffff0001` is positive. The inner 0x8c061064 call
//! always receives `r4+16` and `-1`, so its own free branch is never taken; its
//! only visible work here is the stack frame and the possible `r4+16 == 0`
//! (r14 = 0xfffffff0) early exit.
//!
//! T ends as the last flag writer on the executed path: the `tst` set flag on
//! the `r4 == 0` early exit, otherwise `cmp/pl`'s clear flag when the free is
//! skipped or the free callee's residue when it runs. `r0` is always `r14` at
//! the exit; `r14`, `r13` and `pr` come back from the frame, and the only
//! non-stack store owned by this routine is the zero written to the registry
//! word at 0x8c23dea0 when it still equals `r4`.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    // 0x8c061178: mov.l r14,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[14]);
    // 0x8c06117a: mov.l r13,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[13]);
    // 0x8c06117c: mov r4,r14
    cpu.r[14] = cpu.r[4];
    // 0x8c06117e: sts.l pr,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr);
    // 0x8c061180: mov r5,r13
    cpu.r[13] = cpu.r[5];
    // 0x8c061182: tst r14,r14
    let null = cpu.r[14] == 0;
    cpu.set_t(null);
    // 0x8c061184: bt 0x8c0611a8
    if !null {
        // 0x8c061186: mov.l 0x8c0611b4,r1
        cpu.r[1] = mem.read_u32(0x8c0611b4).unwrap_or(0x8c23dea0);
        // 0x8c061188: mov.l @r1,r0
        cpu.r[0] = mem.read_u32(cpu.r[1]).unwrap_or(0);
        // 0x8c06118a: cmp/eq r14,r0
        let registered = cpu.r[0] == cpu.r[14];
        cpu.set_t(registered);
        // 0x8c06118c: bf 0x8c061192
        if registered {
            // 0x8c06118e: mov #0,r0
            cpu.r[0] = 0;
            // 0x8c061190: mov.l r0,@r1
            let _ = mem.write_u32(cpu.r[1], cpu.r[0]);
        }
        // 0x8c061192: mov.l 0x8c0611b8,r0
        cpu.r[0] = mem.read_u32(0x8c0611b8).unwrap_or(0x8c061064);
        // 0x8c061194: mov r14,r4
        cpu.r[4] = cpu.r[14];
        // 0x8c061196: add #16,r4
        cpu.r[4] = cpu.r[4].wrapping_add(16);
        // 0x8c061198: jsr @r0 ; 0x8c06119a: mov #-1,r5 (delay slot)
        cpu.r[5] = 0xffffffff;
        cpu.pr = 0x8c06119c;
        crate::registry::p_8c061064::run(cpu, mem);
        // 0x8c06119c: exts.w r13,r0
        cpu.r[0] = cpu.r[13] as i16 as i32 as u32;
        // 0x8c06119e: cmp/pl r0
        let positive = (cpu.r[0] as i32) > 0;
        cpu.set_t(positive);
        // 0x8c0611a0: bf 0x8c0611a8
        if positive {
            // 0x8c0611a2: mov.l 0x8c0611bc,r0
            cpu.r[0] = mem.read_u32(0x8c0611bc).unwrap_or(0x8c01032c);
            // 0x8c0611a4: jsr @r0 ; 0x8c0611a6: mov r14,r4 (delay slot)
            cpu.r[4] = cpu.r[14];
            cpu.pr = 0x8c0611a8;
            crate::registry::p_8c01032c::run(cpu, mem); // thunk to free (0x8c159fc0)
        }
    }
    // 0x8c0611a8: mov r14,r0
    cpu.r[0] = cpu.r[14];
    // 0x8c0611aa: lds.l @r15+,pr
    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    // 0x8c0611ac: mov.l @r15+,r13
    cpu.r[13] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    // 0x8c0611ae: rts ; 0x8c0611b0: mov.l @r15+,r14 (delay slot)
    cpu.r[14] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}
