//! 0x8c082c74 — initialise an object header and optionally free it.
//!
//! ```text
//! mov.l r14,@-r15 ; sts.l pr,@-r15 ; mov r4,r14 ; tst r14,r14 ; bt 0x8c082c8e
//! mov.l 0x8c082c98,r1 (0x8c1e9b50) ; exts.w r5,r0 ; cmp/pl r0
//! bf.s 0x8c082c8e ; mov.l r1,@r14     <- store runs on BOTH branch outcomes
//! mov.l 0x8c082c9c,r0 (0x8c01032c) ; jsr @r0 ; nop
//! 0x8c082c8e: mov r14,r0 ; lds.l @r15+,pr ; rts ; mov.l @r15+,r14
//! ```
//!
//! Returns `r4` (zero when `r4` was null). When `r4` is non-null the vtable
//! word 0x8c1e9b50 is always stored at `*r4` — the store is the delay slot of
//! `bf.s`, so it is not conditioned on the sign test. Only a positive
//! sign-extended low half of `r5` reaches the tail `jsr` into 0x8c01032c
//! (`free(r4)`, with `r5` passed through untouched). `pr` is saved on entry and
//! restored at exit, so the `jsr` cannot leak into the caller's `pr`; `r14` is
//! saved/restored too. The residual `T` is `tst`'s (null path), `cmp/pl`'s
//! (skipped call), or the callee's (call path).

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    // mov.l r14,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[14]);
    // sts.l pr,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr);
    // mov r4,r14 ; tst r14,r14 ; bt 0x8c082c8e
    cpu.r[14] = cpu.r[4];
    let null = cpu.r[14] == 0;
    cpu.set_t(null);
    if !null {
        // mov.l 0x8c082c98,r1
        cpu.r[1] = mem.read_u32(0x8c082c98).unwrap_or(0x8c1e9b50);
        // exts.w r5,r0 ; cmp/pl r0
        cpu.r[0] = cpu.r[5] as i16 as i32 as u32;
        let positive = (cpu.r[0] as i32) > 0;
        cpu.set_t(positive);
        // bf.s 0x8c082c8e ; mov.l r1,@r14 (delay slot: runs either way)
        let _ = mem.write_u32(cpu.r[14], cpu.r[1]);
        if positive {
            // mov.l 0x8c082c9c,r0 ; jsr @r0 ; nop
            cpu.r[0] = mem.read_u32(0x8c082c9c).unwrap_or(0x8c01032c);
            crate::registry::p_8c01032c::run(cpu, mem);
        }
    }
    // 0x8c082c8e: mov r14,r0 ; lds.l @r15+,pr ; rts ; mov.l @r15+,r14
    cpu.r[0] = cpu.r[14];
    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[14] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}
