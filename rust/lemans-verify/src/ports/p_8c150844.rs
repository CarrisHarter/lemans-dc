//! 0x8c150844 - free `[r4+8]` through the free thunk, then free r4 when positive.
//!
//! Null r4 exits after the prologue (T=1). Otherwise 0x8c0102cc frees the inner
//! pointer unconditionally (that thunk `jmp`s to free, so pr passes through),
//! then a positive sign-extended low half of r5 frees the object itself.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[14]);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[13]);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr);

    cpu.r[14] = cpu.r[5]; // mov r5,r14
    cpu.r[13] = cpu.r[4]; // mov r4,r13
    cpu.set_t(cpu.r[13] == 0); // tst r13,r13
    if cpu.r[13] != 0 {
        cpu.r[0] = mem.read_u32(0x8c150870).unwrap_or(0x8c0102cc); // mov.l 0x8c150870,r0
        cpu.r[4] = mem.read_u32(cpu.r[13].wrapping_add(8)).unwrap_or(0); // jsr delay: mov.l @(8,r13),r4
        cpu.pr = 0x8c150858;
        crate::registry::p_8c0102cc::run(cpu, mem);
        cpu.r[0] = cpu.r[14] as i16 as i32 as u32; // exts.w r14,r0
        cpu.set_t((cpu.r[0] as i32) > 0); // cmp/pl r0
        if cpu.t() {
            cpu.r[0] = mem.read_u32(0x8c150874).unwrap_or(0x8c01032c); // mov.l 0x8c150874,r0
            cpu.r[4] = cpu.r[13]; // jsr delay: mov r13,r4
            cpu.pr = 0x8c150864;
            crate::registry::p_8c01032c::run(cpu, mem);
        }
    }
    cpu.r[0] = cpu.r[13]; // mov r13,r0
    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[13] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[14] = mem.read_u32(cpu.r[15]).unwrap_or(0); // rts delay
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}
