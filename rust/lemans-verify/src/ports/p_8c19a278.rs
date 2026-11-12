//! 0x8c19a278 - drain the sub-array at `r4+24`, then free the container.
//!
//! Null r13 exits with T=1. Otherwise 0x8c19a45c releases the records at
//! `r13+24`, then a positive `exts.w(r14)` frees r13.

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
        cpu.r[4] = cpu.r[4].wrapping_add(24); // add #24,r4
        cpu.set_t(cpu.r[4] == 0); // tst r4,r4
        if !cpu.t() {
            cpu.r[0] = mem.read_u32(0x8c19a2b0).unwrap_or(0x8c19a45c); // mov.l 0x8c19a2b0,r0
            cpu.pr = 0x8c19a292; // jsr delay is nop
            crate::registry::p_8c19a45c::run(cpu, mem);
            cpu.r[0] = cpu.r[13].wrapping_add(24); // mov r13,r0 ; add #24,r0
            cpu.set_t(cpu.r[0] == 0); // tst r0,r0 (dead)
        }
        cpu.r[0] = cpu.r[14] as i16 as i32 as u32; // exts.w r14,r0
        cpu.set_t((cpu.r[0] as i32) > 0); // cmp/pl r0
        if cpu.t() {
            cpu.r[0] = mem.read_u32(0x8c19a2b4).unwrap_or(0x8c01032c); // mov.l 0x8c19a2b4,r0
            cpu.r[4] = cpu.r[13]; // jsr delay: mov r13,r4
            cpu.pr = 0x8c19a2a4;
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
