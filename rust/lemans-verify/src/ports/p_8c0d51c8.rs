//! 0x8c0d51c8 — dispose of an object: free its two owned sub-objects, then
//! optionally the object itself.
//!
//! ```text
//! mov.l r14,@-r15 ; mov.l r13,@-r15 ; mov r5,r14
//! sts.l pr,@-r15 ; mov r4,r13
//! tst r13,r13 ; bt ret                     ; null object: nothing to do
//! mov.l @(4,r13),r4 ; jsr 0x8c06cbc0       ; destroy sub-object at +4
//! mov.l @(8,r13),r4 ; jsr 0x8c06cbc0       ; destroy sub-object at +8
//! exts.w r14,r0 ; cmp/pl r0 ; bf ret       ; only the low half of arg2 counts
//! mov.l 0x8c0d5200,r0 ; jsr @r0            ; free the object itself
//! ret: mov r13,r0 ; pop pr, r13, r14 ; rts
//! ```
//!
//! 0x8c06cbc0 stamps its argument with the type word and then frees it, so a
//! null sub-object is a no-op there but still leaves T set. `bf`/`bt` here have
//! no delay slots; the `jsr`s take `mov #1,r5` / `mov r13,r4` in theirs. The
//! pushed r13/r14 come back from the stack, so they end at their entry values.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    // mov.l r14,@-r15 ; mov.l r13,@-r15 ; mov r5,r14 ; sts.l pr,@-r15 ; mov r4,r13
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[14]);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[13]);
    cpu.r[14] = cpu.r[5];
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr);
    cpu.r[13] = cpu.r[4];

    let is_null = cpu.r[13] == 0;
    cpu.set_t(is_null); // tst r13,r13
    if !is_null {
        // Sub-object at +4. The delay slot of `jsr` is `mov #1,r5`.
        cpu.r[4] = mem.read_u32(cpu.r[13].wrapping_add(4)).unwrap_or(0);
        cpu.r[0] = mem.read_u32(0x8c0d51fc).unwrap_or(0x8c06cbc0);
        cpu.r[5] = 1;
        cpu.pr = 0x8c0d51de;
        crate::registry::p_8c06cbc0::run(cpu, mem);

        // Sub-object at +8.
        cpu.r[4] = mem.read_u32(cpu.r[13].wrapping_add(8)).unwrap_or(0);
        cpu.r[0] = mem.read_u32(0x8c0d51fc).unwrap_or(0x8c06cbc0);
        cpu.r[5] = 1;
        cpu.pr = 0x8c0d51e6;
        crate::registry::p_8c06cbc0::run(cpu, mem);

        // exts.w r14,r0 ; cmp/pl r0
        cpu.r[0] = cpu.r[14] as i16 as i32 as u32;
        let positive = (cpu.r[0] as i32) > 0;
        cpu.set_t(positive);
        if positive {
            cpu.r[0] = mem.read_u32(0x8c0d5200).unwrap_or(0x8c01032c);
            cpu.r[4] = cpu.r[13]; // delay slot of `jsr`
            cpu.pr = 0x8c0d51f2;
            crate::registry::p_8c01032c::run(cpu, mem);
        }
    }

    // ret: mov r13,r0 ; lds.l pr,@r15+ ; mov.l @r15+,r13 ; rts ; mov.l @r15+,r14
    cpu.r[0] = cpu.r[13];
    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[13] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[14] = mem.read_u32(cpu.r[15]).unwrap_or(0); // delay slot
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}
