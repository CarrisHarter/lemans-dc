//! 0x8c05ad54 — initialise a block with the 0x8c1e9120 sentinel, then free
//! it when the signed halfword argument is positive.
//!
//! ```text
//! mov.l r14,@-r15 ; mov.l r13,@-r15
//! mov   r5,r14                      ; r14 = arg2
//! sts.l pr,@-r15 ; mov r4,r13       ; r13 = arg1
//! tst   r13,r13 ; bt 0x8c05ad78     ; null: return 0, T=1
//! mov.l 0x8c05ad84,r1               ; r1 = 0x8c1e9120
//! mov   #0,r5
//! mov.l 0x8c05ad88,r0 ; jsr @r0
//! mov.l r1,@r13                     ; delay slot: store runs before the callee
//!                                   ; callee 0x8c056bcc stores 0x8c1e8f88 over it
//! exts.w r14,r0 ; cmp/pl r0 ; bf 0x8c05ad78
//! mov.l 0x8c05ad8c,r0 ; jsr @r0
//! mov   r13,r4                      ; delay slot: free(r13) at 0x8c01032c
//! mov   r13,r0
//! lds.l @r15+,pr ; mov.l @r15+,r13 ; rts ; mov.l @r15+,r14 (delay)
//! ```
//!
//! The first jsr runs with `r5 = 0`, so the callee only writes its own
//! 0x8c1e8f88 sentinel and never frees; the 0x8c1e9120 store in the delay slot
//! is visible in the write log but is immediately overwritten. The second call
//! is gated on the `exts.w` halfword of `r14`, so the upper 16 bits of the
//! argument never matter, and on that path the exit state (r0, T, side effects)
//! is whatever the free implementation leaves. The null path leaves the entry
//! T-below-`tst` value: T=1 from the taken `bt`. Every other register keeps
//! its entry value except r0/r1/r4/r5 and the callee clobbers.
//!
//! Literals are read from the image rather than hardcoded, as the original does.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    // 0x8c05ad54: mov.l r14,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[14]);
    // 0x8c05ad56: mov.l r13,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[13]);
    // 0x8c05ad58: mov r5,r14
    cpu.r[14] = cpu.r[5];
    // 0x8c05ad5a: sts.l pr,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr);
    // 0x8c05ad5c: mov r4,r13
    cpu.r[13] = cpu.r[4];
    // 0x8c05ad5e: tst r13,r13
    cpu.set_t(cpu.r[13] == 0);
    // 0x8c05ad60: bt 0x8c05ad78
    if !cpu.t() {
        // 0x8c05ad62: mov.l 0x8c05ad84,r1
        cpu.r[1] = mem.read_u32(0x8c05ad84).unwrap_or(0x8c1e9120);
        // 0x8c05ad64: mov #0,r5
        cpu.r[5] = 0;
        // 0x8c05ad66: mov.l 0x8c05ad88,r0
        cpu.r[0] = mem.read_u32(0x8c05ad88).unwrap_or(0x8c056bcc);
        // 0x8c05ad68: jsr @r0; 0x8c05ad6a: mov.l r1,@r13 (delay slot)
        cpu.pr = 0x8c05ad6c;
        let _ = mem.write_u32(cpu.r[13], cpu.r[1]);
        crate::registry::p_8c056bcc::run(cpu, mem);
        // 0x8c05ad6c: exts.w r14,r0
        cpu.r[0] = cpu.r[14] as u16 as i16 as i32 as u32;
        // 0x8c05ad6e: cmp/pl r0
        cpu.set_t((cpu.r[0] as i32) > 0);
        // 0x8c05ad70: bf 0x8c05ad78
        if cpu.t() {
            // 0x8c05ad72: mov.l 0x8c05ad8c,r0
            cpu.r[0] = mem.read_u32(0x8c05ad8c).unwrap_or(0x8c01032c);
            // 0x8c05ad74: jsr @r0; 0x8c05ad76: mov r13,r4 (delay slot)
            cpu.pr = 0x8c05ad78;
            cpu.r[4] = cpu.r[13];
            crate::registry::p_8c01032c::run(cpu, mem);
        }
    }
    // 0x8c05ad78: mov r13,r0
    cpu.r[0] = cpu.r[13];
    // 0x8c05ad7a: lds.l @r15+,pr
    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    // 0x8c05ad7c: mov.l @r15+,r13
    cpu.r[13] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    // 0x8c05ad7e: rts; 0x8c05ad80: mov.l @r15+,r14 (delay slot)
    cpu.r[14] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}
