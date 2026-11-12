//! 0x8c018e14 — free an object when a signed 16-bit flag says so, then return it.
//!
//! ```text
//! mov.l  r14,@-r15
//! sts.l  pr,@-r15
//! mov    r4,r14
//! tst    r14,r14
//! bt     0x8c018e2a
//! exts.w r5,r0
//! cmp/pl r0
//! bf     0x8c018e2a
//! mov.l  0x8c018e34,r0   ; r0 = 0x8c01032c, the free thunk
//! jsr    @r0             ; delay slot: nop
//! nop
//! 0x8c018e2a:
//! mov    r14,r0
//! lds.l  @r15+,pr
//! rts
//! mov.l  @r15+,r14       ; delay slot of `rts`
//! ```
//!
//! `(void *)FUN_8c018e14(void *p, short flag)`: frees `p` when it is non-null
//! and the sign-extended low half of `r5` is positive, then returns `p`. The
//! two `bt`/`bf` exits have no delay slot, so `r0` still holds `exts.w`'s
//! result on the `bf` exit until the epilogue overwrites it with `r14`.

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
        cpu.r[0] = cpu.r[5] as i16 as i32 as u32;
        let pl = (cpu.r[0] as i32) > 0;
        cpu.set_t(pl);
        if pl {
            cpu.r[0] = mem.read_u32(0x8c018e34).unwrap_or(0);
            cpu.pr = 0x8c018e2a;
            // delay slot of `jsr` is `nop`
            crate::registry::p_8c01032c::run(cpu, mem);
        }
    }
    cpu.r[0] = cpu.r[14];
    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[14] = mem.read_u32(cpu.r[15]).unwrap_or(0); // delay slot of `rts`
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}
