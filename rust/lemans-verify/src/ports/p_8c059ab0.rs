//! 0x8c059ab0 — install a vtable pointer, then free the object if requested.
//!
//! ```text
//! 8c059ab0: mov.l r14,@-r15    ; push r14 (callee-saved)
//! 8c059ab2: sts.l pr,@-r15     ; push pr
//! 8c059ab4: mov   r4,r14       ; r14 = object
//! 8c059ab6: tst   r14,r14
//! 8c059ab8: bt    0x8c059aca   ; object == NULL: nothing else
//! 8c059aba: mov.l 0x8c059ad4,r1 ; r1 = 0x8c1e90cc (vtable)
//! 8c059abc: exts.w r5,r0       ; r0 = sext16(low half of flag)
//! 8c059abe: cmp/pl r0
//! 8c059ac0: bf.s  0x8c059aca   ; signed 16-bit flag <= 0: skip the free
//! 8c059ac2: mov.l r1,@r14      ; delay slot: *object = vtable (ALWAYS runs)
//! 8c059ac4: mov.l 0x8c059ad8,r0 ; r0 = 0x8c01032c (free thunk)
//! 8c059ac6: jsr   @r0 ; 8c059ac8: nop
//! 8c059aca: mov   r14,r0       ; return object
//! 8c059acc: lds.l @r15+,pr
//! 8c059ace: rts
//! 8c059ad0: mov.l @r15+,r14    ; delay slot
//! ```
//!
//! Only the low 16 bits of r5 count (`exts.w`): r5=0xffff0001 calls free,
//! r5=0x00010000 does not. `bf.s` is a delayed branch, so the vtable store in
//! its delay slot executes on both outcomes: every non-NULL call stores
//! 0x8c1e90cc at *object and leaves r1 holding it, while the NULL path leaves
//! r1 alone. T is `tst`'s set flag on the NULL path, `cmp/pl`'s clear flag on
//! the non-positive path, and the callee's T after a free call.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    // 0x8c059ab0: mov.l r14,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[14]);
    // 0x8c059ab2: sts.l pr,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr);
    // 0x8c059ab4: mov r4,r14
    cpu.r[14] = cpu.r[4];
    // 0x8c059ab6: tst r14,r14
    cpu.set_t(cpu.r[14] == 0);
    // 0x8c059ab8: bt 0x8c059aca
    if cpu.r[14] != 0 {
        // 0x8c059aba: mov.l 0x8c059ad4,r1 — the vtable pointer
        let r1 = mem.read_u32(0x8c059ad4).unwrap_or(0x8c1e90cc);
        // 0x8c059abc: exts.w r5,r0 — sign-extend the low 16 bits only
        cpu.r[0] = cpu.r[5] as u16 as i16 as i32 as u32;
        // 0x8c059abe: cmp/pl r0
        cpu.set_t((cpu.r[0] as i32) > 0);
        // 0x8c059ac0: bf.s 0x8c059aca — delayed, so the store below happens
        // whichever way it goes.
        let skip_free = (cpu.r[0] as i32) <= 0;
        // 0x8c059ac2: mov.l r1,@r14 (delay slot)
        cpu.r[1] = r1;
        let _ = mem.write_u32(cpu.r[14], r1);
        if !skip_free {
            // 0x8c059ac4: mov.l 0x8c059ad8,r0 — the 0x8c01032c free thunk
            cpu.r[0] = mem.read_u32(0x8c059ad8).unwrap_or(0x8c01032c);
            // 0x8c059ac6: jsr @r0 ; 0x8c059ac8: nop (delay slot)
            cpu.pr = 0x8c059aca;
            crate::registry::p_8c01032c::run(cpu, mem);
        }
    }
    // 0x8c059aca: mov r14,r0
    cpu.r[0] = cpu.r[14];
    // 0x8c059acc: lds.l @r15+,pr
    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    // 0x8c059ace: rts ; 0x8c059ad0: mov.l @r15+,r14 (delay slot)
    cpu.r[14] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}
