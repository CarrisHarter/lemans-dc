//! 0x8c0ed764 — destroying destructor: reset the object's vtable words, then
//! free the object when asked.
//!
//! ```text
//! mov.l r14,@-r15                      ; sts.l pr,@-r15
//! mov   r4,r14                         ; r14 = object
//! tst   r14,r14 ; bt 0x8c0ed786        ; null → return null
//! mov.l 0x8c0ed790,r0                  ; r0 = vtable word A
//! tst   r14,r14 ; bt.s 0x8c0ed77a
//! mov.l r0,@r14                        ; delay slot: *object = A (always runs)
//! mov.l 0x8c0ed794,r0 ; mov.l r0,@r14  ; *object = vtable word B
//! exts.w r5,r0 ; cmp/pl r0             ; delete flag = sign-extended low half
//! bf    0x8c0ed786
//! mov.l 0x8c0ed798,r0 ; jsr @r0
//! mov   r14,r4                         ; delay slot: free(object)
//! 0x8c0ed786: mov r14,r0 ; lds.l @r15+,pr ; rts ; mov.l @r15+,r14
//! ```
//!
//! The vtable words are read from the literal pool at run time in program
//! order, so even a spec that points `r4` at the pool itself replays the same
//! corruption the original would. T on exit is the last flag-setting
//! instruction: the `tst` on the null path, `cmp/pl` when the delete flag is
//! not positive, or the free() callee's final comparison when it is.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    // Prologue: push r14, push pr.
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[14]);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr);
    cpu.r[14] = cpu.r[4]; // mov r4,r14

    if cpu.r[14] == 0 {
        cpu.set_t(true); // tst r14,r14 ; bt 0x8c0ed786
    } else {
        // 0x8c0ed76e: first vtable word. Its store is the bt.s delay slot, so it
        // executes on this path no matter which way that branch goes.
        cpu.r[0] = mem.read_u32(0x8c0ed790).unwrap_or(0x8c1ec920);
        let _ = mem.write_u32(cpu.r[14], cpu.r[0]);
        // 0x8c0ed776: second vtable word.
        cpu.r[0] = mem.read_u32(0x8c0ed794).unwrap_or(0x8c1ec934);
        let _ = mem.write_u32(cpu.r[14], cpu.r[0]);

        cpu.r[0] = cpu.r[5] as i16 as i32 as u32; // exts.w r5,r0
        let deleting = (cpu.r[0] as i32) > 0; // cmp/pl r0
        cpu.set_t(deleting);
        if deleting {
            cpu.r[0] = mem.read_u32(0x8c0ed798).unwrap_or(0x8c01032c);
            cpu.pr = 0x8c0ed786; // jsr @r0
            cpu.r[4] = cpu.r[14]; // delay slot: mov r14,r4
            crate::registry::p_8c01032c::run(cpu, mem);
        }
    }

    cpu.r[0] = cpu.r[14]; // 0x8c0ed786: mov r14,r0 (T untouched)

    // Epilogue: lds.l @r15+,pr ; rts (delay: mov.l @r15+,r14).
    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[14] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}
