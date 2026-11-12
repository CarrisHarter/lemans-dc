//! 0x8c19ae24 - stamp two vtable words around a sub-array release, then free.
//!
//! `[r14] = 0x8c1f13c0` lands in the `bt.s` delay slot on both paths; when
//! `r14+4` is non-zero, 0x8c19aea0 releases that sub-array. The vtable word is
//! then restamped to 0x8c1e8fec, and a positive `exts.w(r13)` frees r14.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[14]);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[13]);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr);
    cpu.r[14] = cpu.r[4]; // mov r4,r14
    cpu.r[13] = cpu.r[5]; // mov r5,r13
    cpu.set_t(cpu.r[14] == 0); // tst r14,r14
    if cpu.r[14] != 0 {
        cpu.r[0] = mem.read_u32(0x8c19ae68).unwrap_or(0x8c1f13c0); // mov.l 0x8c19ae68,r0
        cpu.r[4] = cpu.r[4].wrapping_add(4); // add #4,r4
        cpu.set_t(cpu.r[4] == 0); // tst r4,r4
        let _ = mem.write_u32(cpu.r[14], cpu.r[0]); // bt.s delay: mov.l r0,@r14
        if !cpu.t() {
            cpu.r[0] = mem.read_u32(0x8c19ae6c).unwrap_or(0x8c19aea0); // mov.l 0x8c19ae6c,r0
            cpu.pr = 0x8c19ae42; // jsr delay is nop
            crate::registry::p_8c19aea0::run(cpu, mem);
            cpu.r[0] = cpu.r[14].wrapping_add(4); // mov r14,r0 ; add #4,r0
            cpu.set_t(cpu.r[0] == 0); // tst r0,r0 (dead)
        }
        cpu.set_t(cpu.r[14] == 0); // tst r14,r14
        if cpu.r[14] != 0 {
            cpu.r[0] = mem.read_u32(0x8c19ae70).unwrap_or(0x8c1e8fec); // mov.l 0x8c19ae70,r0
            let _ = mem.write_u32(cpu.r[14], cpu.r[0]); // mov.l r0,@r14
        }
        cpu.r[0] = cpu.r[13] as i16 as i32 as u32; // exts.w r13,r0
        cpu.set_t((cpu.r[0] as i32) > 0); // cmp/pl r0
        if cpu.t() {
            cpu.r[0] = mem.read_u32(0x8c19ae74).unwrap_or(0x8c01032c); // mov.l 0x8c19ae74,r0
            cpu.r[4] = cpu.r[14]; // jsr delay: mov r14,r4
            cpu.pr = 0x8c19ae5c;
            crate::registry::p_8c01032c::run(cpu, mem);
        }
    }
    cpu.r[0] = cpu.r[14]; // mov r14,r0
    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[13] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[14] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}
