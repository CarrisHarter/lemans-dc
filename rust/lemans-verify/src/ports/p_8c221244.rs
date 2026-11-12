//! 0x8c221244 - reject a live index, otherwise tail-call the admit dispatcher.
//!
//! Saves the object, the low half of r5 and the low byte of r6. If
//! `[[0x8c2bde98] + 128*index + 4]` is nonzero, returns -1. Otherwise restores
//! pr in the `jmp` delay slot and continues at 0x8c223812, so that callee's
//! `rts` returns to our caller.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let saved_pr = cpu.pr;
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr);
    cpu.r[15] = cpu.r[15].wrapping_sub(12);
    cpu.r[0] = cpu.r[5];
    let _ = mem.write_u32(cpu.r[15], cpu.r[4]);
    let _ = mem.write_u16(cpu.r[15].wrapping_add(4), cpu.r[0] as u16);
    cpu.r[0] = cpu.r[6];
    let _ = mem.write_u8(cpu.r[15].wrapping_add(8), cpu.r[0] as u8);
    cpu.r[4] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[4] = mem.read_u32(cpu.r[4].wrapping_add(40)).unwrap_or(0); // jsr delay
    cpu.pr = 0x8c22125a;
    crate::registry::p_8c221da8::run(cpu, mem);
    cpu.set_t(cpu.r[0] == 0); // tst r0,r0
    if cpu.t() {
        cpu.r[0] = mem.read_u8(cpu.r[15].wrapping_add(8)).unwrap_or(0) as i8 as i32 as u32;
        cpu.r[4] = mem.read_u32(cpu.r[15]).unwrap_or(0);
        cpu.r[6] = cpu.r[0];
        cpu.r[0] = mem.read_u16(cpu.r[15].wrapping_add(4)).unwrap_or(0) as i16 as i32 as u32;
        cpu.r[5] = cpu.r[0];
        cpu.r[15] = cpu.r[15].wrapping_add(12);
        cpu.pr = saved_pr; // jmp delay: lds.l @r15+,pr
        cpu.r[15] = cpu.r[15].wrapping_add(4);
        crate::registry::p_8c223812::run(cpu, mem);
    } else {
        cpu.r[15] = cpu.r[15].wrapping_add(12);
        cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0);
        cpu.r[15] = cpu.r[15].wrapping_add(4);
        cpu.r[0] = 0xffffffff; // rts delay: mov #-1,r0
    }
}
