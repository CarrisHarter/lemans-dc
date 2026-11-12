//! 0x8c05adf0 — reinitialise a structure field: store the 0x8c1e9130
//! sentinel, clear the pointer at 0x8c056bcc, and free it when the signed
//! halfword argument is positive.
//!
//! ```text
//! mov.l r14,@-r15 ; mov.l r13,@-r15
//! mov   r5,r14
//! sts.l pr,@-r15
//! mov   r4,r13
//! tst   r13,r13 ; bt 0x8c05ae14        ; r4 == 0: nothing to do
//! mov.l 0x8c05ae20,r1                  ; r1 = 0x8c1e9130
//! mov   #0,r5
//! mov.l 0x8c05ae24,r0 ; jsr @r0        ; call 0x8c056bcc
//! mov.l r1,@r13                        ; delay slot: sentinel store on BOTH outcomes
//! exts.w r14,r0 ; cmp/pl r0 ; bf 0x8c05ae14 ; r5 low half <= 0: no free
//! mov.l 0x8c05ae28,r0 ; jsr @r0        ; free(r4) at 0x8c01032c
//! mov   r13,r4                         ; delay slot
//! mov r13,r0 ; lds.l @r15+,pr ; mov.l @r15+,r13
//! rts ; mov.l @r15+,r14 (delay)
//! ```
//!
//! The delay slot of the first `jsr` stores 0x8c1e9130 at *r4 before
//! 0x8c056bcc overwrites it with the 0x8c1e8f88 sentinel, so both stores are
//! observable. 0x8c056bcc is called with r5 = 0, so its own free branch is
//! never taken; only the target's `cmp/pl` on the sign-extended low half of the
//! caller's r5 decides whether 0x8c01032c frees the pointer. The epilogue
//! restores r13/r14/pr from the frame, so the return value is r13 = the
//! original r4 and the final T is the last flag-setting instruction on the
//! taken path (the entry `tst` when null, `cmp/pl` when no free, and free's
//! bin search `cmp/hi` when the pointer is freed).
//!
//! Literals are read from the image rather than hardcoded, as the original does.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    // 0x8c05adf0: mov.l r14,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[14]);
    // 0x8c05adf2: mov.l r13,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[13]);
    // 0x8c05adf4: mov r5,r14
    cpu.r[14] = cpu.r[5];
    // 0x8c05adf6: sts.l pr,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr);
    // 0x8c05adf8: mov r4,r13
    cpu.r[13] = cpu.r[4];
    // 0x8c05adfa: tst r13,r13
    cpu.set_t(cpu.r[13] == 0);
    // 0x8c05adfc: bt 0x8c05ae14
    if !cpu.t() {
        // 0x8c05adfe: mov.l 0x8c05ae20,r1
        cpu.r[1] = mem.read_u32(0x8c05ae20).unwrap_or(0); // 0x8c1e9130
        // 0x8c05ae00: mov #0,r5
        cpu.r[5] = 0;
        // 0x8c05ae02: mov.l 0x8c05ae24,r0
        cpu.r[0] = mem.read_u32(0x8c05ae24).unwrap_or(0); // 0x8c056bcc
        // 0x8c05ae04: jsr @r0; 0x8c05ae06: mov.l r1,@r13 (delay slot)
        cpu.pr = 0x8c05ae08;
        let _ = mem.write_u32(cpu.r[13], cpu.r[1]);
        crate::registry::p_8c056bcc::run(cpu, mem);
        // 0x8c05ae08: exts.w r14,r0
        cpu.r[0] = cpu.r[14] as u16 as i16 as i32 as u32;
        // 0x8c05ae0a: cmp/pl r0
        cpu.set_t((cpu.r[0] as i32) > 0);
        // 0x8c05ae0c: bf 0x8c05ae14
        if cpu.t() {
            // 0x8c05ae0e: mov.l 0x8c05ae28,r0
            cpu.r[0] = mem.read_u32(0x8c05ae28).unwrap_or(0); // 0x8c01032c
            // 0x8c05ae10: jsr @r0; 0x8c05ae12: mov r13,r4 (delay slot)
            cpu.pr = 0x8c05ae14;
            cpu.r[4] = cpu.r[13];
            crate::registry::p_8c01032c::run(cpu, mem);
        }
    }
    // 0x8c05ae14: mov r13,r0
    cpu.r[0] = cpu.r[13];
    // 0x8c05ae16: lds.l @r15+,pr
    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    // 0x8c05ae18: mov.l @r15+,r13
    cpu.r[13] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    // 0x8c05ae1a: rts; 0x8c05ae1c: mov.l @r15+,r14 (delay slot)
    cpu.r[14] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}
