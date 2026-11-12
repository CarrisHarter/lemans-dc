//! 0x8c0d8ab8 — store a global into `*r4`, then free `r4` when it is non-null
//! and `sext16(r5) > 0`; returns r4.
//!
//! ```text
//! 8c0d8ab8  mov.l r14,@-r15       push r14
//! 8c0d8aba  sts.l pr,@-r15        push pr
//! 8c0d8abc  mov   r4,r14          r14 = object
//! 8c0d8abe  tst   r14,r14
//! 8c0d8ac0  bt    0x8c0d8ad2      null: skip store and free
//! 8c0d8ac2  mov.l 0x8c0d8adc,r1   r1 = *0x8c0d8adc = 0x8c1ec360
//! 8c0d8ac4  exts.w r5,r0          r0 = sext16(r5)
//! 8c0d8ac6  cmp/pl r0
//! 8c0d8ac8  bf.s  0x8c0d8ad2      sext16(r5) <= 0: keep object alive
//! 8c0d8aca  mov.l r1,@r14         delay: store the global into *object
//! 8c0d8acc  mov.l 0x8c0d8ae0,r0   r0 = *0x8c0d8ae0 = 0x8c01032c (free thunk)
//! 8c0d8ace  jsr   @r0
//! 8c0d8ad0  nop                   delay
//! 8c0d8ad2  mov   r14,r0          return the object
//! 8c0d8ad4  lds.l @r15+,pr
//! 8c0d8ad6  rts
//! 8c0d8ad8  mov.l @r15+,r14       delay
//! ```
//!
//! The store sits in the `bf.s` delay slot, so it happens for every non-null
//! `r4` — including when the guard declines to free. Only `exts.w` feeds the
//! guard: r5 = 0x00010001 frees, r5 = 0xffff0001 does not. The free thunk at
//! 0x8c01032c tail-jumps to free (0x8c159fc0) with r4 unchanged. `pr` is saved
//! on the frame and restored, so it ends at its entry value.

use sh4_oracle::{Cpu, Memory};

const GLOBAL_PTR: u32 = 0x8c0d8adc;
const FREE_THUNK_PTR: u32 = 0x8c0d8ae0;
const GLOBAL: u32 = 0x8c1ec360;
const FREE_THUNK: u32 = 0x8c01032c;

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    // 0x8c0d8ab8: mov.l r14,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[14]);
    // 0x8c0d8aba: sts.l pr,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr);
    // 0x8c0d8abc: mov r4,r14
    cpu.r[14] = cpu.r[4];
    // 0x8c0d8abe: tst r14,r14
    cpu.set_t(cpu.r[14] == 0);
    // 0x8c0d8ac0: bt 0x8c0d8ad2
    if cpu.r[14] != 0 {
        // 0x8c0d8ac2: mov.l 0x8c0d8adc,r1
        let global = mem.read_u32(GLOBAL_PTR).unwrap_or(GLOBAL);
        cpu.r[1] = global;
        // 0x8c0d8ac4: exts.w r5,r0
        cpu.r[0] = cpu.r[5] as i16 as i32 as u32;
        // 0x8c0d8ac6: cmp/pl r0
        cpu.set_t((cpu.r[0] as i32) > 0);
        // 0x8c0d8ac8: bf.s 0x8c0d8ad2 ; 0x8c0d8aca: mov.l r1,@r14 (delay)
        let _ = mem.write_u32(cpu.r[14], global);
        if (cpu.r[0] as i32) > 0 {
            // 0x8c0d8acc: mov.l 0x8c0d8ae0,r0
            cpu.r[0] = mem.read_u32(FREE_THUNK_PTR).unwrap_or(FREE_THUNK);
            // 0x8c0d8ace: jsr @r0 ; 0x8c0d8ad0: nop (delay)
            cpu.pr = 0x8c0d8ad2;
            crate::registry::p_8c01032c::run(cpu, mem);
        }
    }
    // 0x8c0d8ad2: mov r14,r0
    cpu.r[0] = cpu.r[14];
    // 0x8c0d8ad4: lds.l @r15+,pr
    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    // 0x8c0d8ad6: rts ; 0x8c0d8ad8: mov.l @r15+,r14 (delay)
    cpu.r[14] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}
