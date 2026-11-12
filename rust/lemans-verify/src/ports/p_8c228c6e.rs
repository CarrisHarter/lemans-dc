//! 0x8c228c6e - 0 only when `[0x8c2c7dc0]` is nonzero and 0x8c22828e returns -1.
//!
//! Every other path returns 1. The 0 is written in the `rts` delay slot.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr);
    cpu.r[2] = mem.read_u32(0x8c228cb8).unwrap_or(0x8c2c7dc0);
    cpu.r[3] = mem.read_u32(cpu.r[2]).unwrap_or(0);
    cpu.set_t(cpu.r[3] == 0); // tst r3,r3
    let mut zero = false;
    if !cpu.t() {
        cpu.pr = 0x8c228c7c;
        crate::registry::p_8c22828e::run(cpu, mem);
        cpu.set_t(cpu.r[0] == 0xffff_ffff); // cmp/eq #-1,r0
        zero = cpu.t();
    }
    if zero {
        cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0);
        cpu.r[15] = cpu.r[15].wrapping_add(4);
        cpu.r[0] = 0; // rts delay
    } else {
        cpu.r[0] = 1;
        cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0);
        cpu.r[15] = cpu.r[15].wrapping_add(4);
    }
}
