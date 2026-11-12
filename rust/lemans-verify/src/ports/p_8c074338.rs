//! 0x8c074338 — free a non-null block when the count argument is positive.
//!
//! ```text
//! mov.l r14,@-r15 ; sts.l pr,@-r15 ; mov r4,r14
//! tst r14,r14 ; bt out
//! exts.w r5,r0 ; cmp/pl r0 ; bf out
//! mov.l 0x8c074358,r0 (0x8c01032c) ; jsr @r0 ; nop
//! out: mov r14,r0 ; lds.l @r15+,pr ; rts ; mov.l @r15+,r14
//! ```
//!
//! `r4` is a block pointer and the low 16 bits of `r5`, sign-extended, are a
//! count. A null pointer or a non-positive count skips the call entirely; the
//! `bt` leaves T set, while the `bf` arm leaves T clear from `cmp/pl`. The call
//! goes through the `free` thunk at 0x8c01032c, so every register and store it
//! makes is the callee's; the delay slots are `nop` and the final register pop.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    // 0x8c074338: mov.l r14,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[14]);
    // 0x8c07433a: sts.l pr,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr);
    // 0x8c07433c: mov r4,r14
    cpu.r[14] = cpu.r[4];
    // 0x8c07433e: tst r14,r14 ; 0x8c074340: bt 0x8c07434e
    cpu.set_t(cpu.r[14] == 0);
    if !cpu.t() {
        // 0x8c074342: exts.w r5,r0
        cpu.r[0] = cpu.r[5] as i16 as i32 as u32;
        // 0x8c074344: cmp/pl r0 ; 0x8c074346: bf 0x8c07434e
        let positive = (cpu.r[0] as i32) > 0;
        cpu.set_t(positive);
        if positive {
            // 0x8c074348: mov.l 0x8c074358,r0
            cpu.r[0] = mem.read_u32(0x8c074358).unwrap_or(0x8c01032c);
            // 0x8c07434a: jsr @r0 ; 0x8c07434c: nop (delay slot)
            cpu.pr = 0x8c07434e;
            crate::registry::p_8c01032c::run(cpu, mem);
        }
    }
    // 0x8c07434e: mov r14,r0
    cpu.r[0] = cpu.r[14];
    // 0x8c074350: lds.l @r15+,pr
    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    // 0x8c074352: rts ; 0x8c074354: mov.l @r15+,r14 (delay slot)
    cpu.r[14] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}
