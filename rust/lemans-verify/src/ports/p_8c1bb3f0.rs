//! 0x8c1bb3f0 — release a handle when asked.
//!
//! If `r4` is non-null and the sign-extended low half of `r5` is positive,
//! call the free thunk at 0x8c01032c with `r4` unchanged. Returns `r4`.
//!
//! ```text
//! mov.l r14,@-r15 ; sts.l pr,@-r15 ; mov r4,r14
//! tst   r14,r14   ; bt skip
//! exts.w r5,r0    ; cmp/pl r0 ; bf skip
//! mov.l 0x8c1bb410,r0  ; jsr @r0 ; nop   (thunk -> 0x8c159fc0)
//! skip: mov r14,r0 ; lds.l @r15+,pr ; rts ; mov.l @r15+,r14
//! ```
//!
//! The call leaves `T` as the callee left it; the other exits leave the flags
//! of `tst` / `cmp/pl`.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[14]);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr);
    cpu.r[14] = cpu.r[4];
    let t = cpu.r[14] == 0;
    cpu.set_t(t);
    if !t {
        cpu.r[0] = cpu.r[5] as i16 as i32 as u32; // exts.w r5,r0
        let pl = (cpu.r[0] as i32) > 0;
        cpu.set_t(pl);
        if pl {
            cpu.r[0] = mem.read_u32(0x8c1bb410).unwrap_or(0);
            cpu.pr = 0x8c1bb406; // jsr sets pr to the return site
            crate::registry::p_8c01032c::run(cpu, mem);
        }
    }
    cpu.r[0] = cpu.r[14];
    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[14] = mem.read_u32(cpu.r[15]).unwrap_or(0); // delay slot of `rts`
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}
