//! 0x8c184c20 - clear a field, release the sub-array at `r4+0x174`, then free.
//!
//! `[r13+0x138] = 0` lands in the `bt.s` delay slot on both paths. When
//! `r13+0x174` is non-zero, 0x8c184e3c releases that sub-array; a positive
//! `exts.w(r14)` then frees r13. Returns r13.

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
        cpu.r[0] = mem.read_u32(0x8c184c60).unwrap_or(0x174); // mov.l 0x8c184c60,r0
        cpu.r[1] = 0; // mov #0,r1
        cpu.r[4] = cpu.r[4].wrapping_add(cpu.r[0]); // add r0,r4
        cpu.r[0] = mem.read_u32(0x8c184c64).unwrap_or(0x138); // mov.l 0x8c184c64,r0
        cpu.set_t(cpu.r[4] == 0); // tst r4,r4
        let _ = mem.write_u32(cpu.r[13].wrapping_add(cpu.r[0]), cpu.r[1]); // bt.s delay: mov.l r1,@(r0,r13)
        if !cpu.t() {
            cpu.r[0] = mem.read_u32(0x8c184c68).unwrap_or(0x8c184e3c); // mov.l 0x8c184c68,r0
            cpu.pr = 0x8c184c42; // jsr delay is nop
            crate::registry::p_8c184e3c::run(cpu, mem);
            cpu.r[1] = mem.read_u32(0x8c184c60).unwrap_or(0x174); // mov.l 0x8c184c60,r1
            cpu.r[0] = cpu.r[13].wrapping_add(cpu.r[1]); // mov r13,r0 ; add r1,r0
            cpu.set_t(cpu.r[0] == 0); // tst r0,r0 (dead)
        }
        cpu.r[0] = cpu.r[14] as i16 as i32 as u32; // exts.w r14,r0
        cpu.set_t((cpu.r[0] as i32) > 0); // cmp/pl r0
        if cpu.t() {
            cpu.r[0] = mem.read_u32(0x8c184c6c).unwrap_or(0x8c01032c); // mov.l 0x8c184c6c,r0
            cpu.r[4] = cpu.r[13]; // jsr delay: mov r13,r4
            cpu.pr = 0x8c184c56;
            crate::registry::p_8c01032c::run(cpu, mem);
        }
    }
    cpu.r[0] = cpu.r[13]; // mov r13,r0
    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[13] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[14] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}
