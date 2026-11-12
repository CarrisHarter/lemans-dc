//! 0x8c19d48c - increment a refcount, initialising bits when it was zero.
//!
//! When `[r14]` is zero the target at `[r14+4]` gets bit 1 set and bit 2 cleared
//! through two 0x8c05d274 calls (both with `r6 = 1`). Then `[r14] += 1` either way.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[14]);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr);
    cpu.r[14] = cpu.r[4]; // mov r4,r14
    cpu.r[0] = mem.read_u32(cpu.r[14]).unwrap_or(0); // mov.l @r14,r0
    cpu.set_t(cpu.r[0] == 0); // tst r0,r0
    if cpu.t() {
        cpu.r[4] = mem.read_u32(cpu.r[14].wrapping_add(4)).unwrap_or(0); // mov.l @(4,r14),r4
        cpu.r[6] = 1; // mov #1,r6
        cpu.r[0] = mem.read_u32(0x8c19d4b8).unwrap_or(0x8c05d274); // mov.l 0x8c19d4b8,r0
        cpu.r[5] = cpu.r[6]; // jsr delay: mov r6,r5
        cpu.pr = 0x8c19d4a2;
        crate::registry::p_8c05d274::run(cpu, mem);
        cpu.r[4] = mem.read_u32(cpu.r[14].wrapping_add(4)).unwrap_or(0); // mov.l @(4,r14),r4
        cpu.r[6] = 1; // mov #1,r6
        cpu.r[0] = mem.read_u32(0x8c19d4b8).unwrap_or(0x8c05d274); // mov.l 0x8c19d4b8,r0
        cpu.r[5] = 2; // jsr delay: mov #2,r5
        cpu.pr = 0x8c19d4ac;
        crate::registry::p_8c05d274::run(cpu, mem);
    }
    cpu.r[0] = mem.read_u32(cpu.r[14]).unwrap_or(0); // mov.l @r14,r0
    cpu.r[0] = cpu.r[0].wrapping_add(1); // add #1,r0
    let _ = mem.write_u32(cpu.r[14], cpu.r[0]); // mov.l r0,@r14
    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[14] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}
