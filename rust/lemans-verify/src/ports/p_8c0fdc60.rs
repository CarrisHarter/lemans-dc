//! 0x8c0fdc60 — clear a global slot, then free the object when the count is positive.
//!
//! Null r4 exits after the prologue (T=1 from `tst`). Otherwise `[0x8c2ab268]`
//! is zeroed in the `bf.s` delay slot — always, whatever the branch taken — and
//! when `(i16)r5 > 0` the object is freed through 0x8c01032c with the argument
//! still in r4 (the `jsr` delay slot is a nop). r0 returns the argument.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let saved14 = cpu.r[14]; // mov.l r14,@-r15 ; sts.l pr,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(8);
    cpu.r[14] = cpu.r[4]; // mov r4,r14
    cpu.set_t(cpu.r[14] == 0); // tst r14,r14
    if cpu.r[14] != 0 {
        cpu.r[3] = mem.read_u32(0x8c0fdc84).unwrap_or(0x8c2ab268); // mov.l 0x8c0fdc84,r3
        cpu.r[1] = 0; // mov #0,r1
        let half = cpu.r[5] as u16 as i16; // exts.w r5,r0
        cpu.r[0] = half as i32 as u32;
        cpu.set_t(half > 0); // cmp/pl r0
        let _ = mem.write_u32(cpu.r[3], cpu.r[1]); // bf.s delay slot: mov.l r1,@r3
        if half > 0 {
            cpu.r[0] = mem.read_u32(0x8c0fdc88).unwrap_or(0x8c01032c); // mov.l 0x8c0fdc88,r0
            cpu.pr = 0x8c0fdc7c;
            crate::registry::p_8c01032c::run(cpu, mem); // jsr @r0
        }
    }
    cpu.r[0] = cpu.r[14]; // mov r14,r0
    cpu.r[15] = cpu.r[15].wrapping_add(8); // lds.l @r15+,pr ; rts delay: @r15+,r14
    cpu.r[14] = saved14;
}
