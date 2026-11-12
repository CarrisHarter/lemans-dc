//! 0x8c082fe4 — release a refcounted handle through 0x8c018bd8, then, when
//! the signed low half of the original count argument is positive, free the
//! handle itself through the free thunk 0x8c01032c.
//!
//! ```text
//! mov.l r14,@-r15 ; mov.l r13,@-r15 ; mov r5,r14 ; sts.l pr,@-r15
//! mov r4,r13 ; tst r13,r13 ; bt out
//! r0 = 0x8c018bd8 ; jsr @r0 ; mov #-1,r5 (delay)
//! exts.w r14,r0 ; cmp/pl r0 ; bf out
//! r0 = 0x8c01032c ; jsr @r0 ; mov r13,r4 (delay)
//! out: mov r13,r0 ; lds.l @r15+,pr ; mov.l @r15+,r13 ; rts
//!      mov.l @r15+,r14 (delay)
//! ```
//!
//! The release always gets count -1, so it never frees the handle. The second
//! call is guarded by the *original* count argument, sign-extended from its
//! low half: 0x0000ffff is positive, 0x00008000 and 0x12340000 are not.
//! Plain `bt`/`bf` take no delay slot; the delay slots belong to the two `jsr`s
//! and the `rts`. The returned value is the handle as it was restored, so it is
//! `r13` in every arm. T is left by whichever arm ran last: the first `tst`
//! (set) on the null arm, the free's final `cmp/hi` (clear) on the freeing arm,
//! and `cmp/pl` (clear) when only the release ran.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    // 0x8c082fe4: mov.l r14,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[14]);
    // 0x8c082fe6: mov.l r13,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[13]);
    // 0x8c082fe8: mov r5,r14
    cpu.r[14] = cpu.r[5];
    // 0x8c082fea: sts.l pr,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr);
    // 0x8c082fec: mov r4,r13
    cpu.r[13] = cpu.r[4];
    // 0x8c082fee: tst r13,r13
    cpu.set_t(cpu.r[13] == 0);
    // 0x8c082ff0: bt 0x8c083004
    if cpu.r[13] != 0 {
        // 0x8c082ff2: mov.l 0x8c083010,r0
        cpu.r[0] = mem.read_u32(0x8c083010).unwrap_or(0); // 0x8c018bd8
        // 0x8c082ff4: jsr @r0 ; 0x8c082ff6: mov #-1,r5 (delay slot)
        cpu.pr = 0x8c082ff8;
        cpu.r[5] = 0xffff_ffff;
        crate::registry::p_8c018bd8::run(cpu, mem);
        // 0x8c082ff8: exts.w r14,r0
        cpu.r[0] = cpu.r[14] as u16 as i16 as i32 as u32;
        // 0x8c082ffa: cmp/pl r0
        cpu.set_t((cpu.r[0] as i32) > 0);
        // 0x8c082ffc: bf 0x8c083004
        if cpu.t() {
            // 0x8c082ffe: mov.l 0x8c083014,r0
            cpu.r[0] = mem.read_u32(0x8c083014).unwrap_or(0); // 0x8c01032c
            // 0x8c083000: jsr @r0 ; 0x8c083002: mov r13,r4 (delay slot)
            cpu.pr = 0x8c083004;
            cpu.r[4] = cpu.r[13];
            crate::registry::p_8c01032c::run(cpu, mem);
        }
    }
    // 0x8c083004: mov r13,r0
    cpu.r[0] = cpu.r[13];
    // 0x8c083006: lds.l @r15+,pr
    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    // 0x8c083008: mov.l @r15+,r13
    cpu.r[13] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    // 0x8c08300a: rts ; 0x8c08300c: mov.l @r15+,r14 (delay slot)
    cpu.r[14] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}
