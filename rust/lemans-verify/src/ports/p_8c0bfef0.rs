//! 0x8c0bfef0 — build a 10-word structure at `r4`.
//!
//! Frame: `r14`, `pr`, then a 4-byte scratch slot. The slot receives a copy of
//! the sign-extended byte at scratch+1; that byte is forwarded (through `r5`)
//! to 0x8c0c079c, which initialises the seven-word tail at `r4+28` and reads a
//! global word from 0x8c1fd8c0. The caller then zeroes words 0..6 of `r4` and
//! returns `r4`.
//!
//! `add #imm` does not touch T in the oracle, and no other instruction here
//! sets it, so T/SR is left exactly as entered.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[14]);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    cpu.r[14] = cpu.r[4];
    cpu.r[0] = cpu.r[15].wrapping_add(1);
    cpu.r[0] = mem.read_u8(cpu.r[0]).unwrap_or(0) as i8 as i32 as u32;
    cpu.r[5] = cpu.r[15];
    cpu.r[6] = cpu.r[15];
    let _ = mem.write_u8(cpu.r[15], cpu.r[0] as u8);
    cpu.r[4] = cpu.r[4].wrapping_add(28);
    cpu.r[0] = mem.read_u32(0x8c0bff28).unwrap_or(0);
    cpu.r[6] = cpu.r[6].wrapping_add(2); // delay slot of `jsr`
    crate::registry::p_8c0c079c::run(cpu, mem);
    cpu.r[1] = 0;
    let _ = mem.write_u32(cpu.r[14], cpu.r[1]);
    cpu.r[0] = cpu.r[14];
    let _ = mem.write_u32(cpu.r[14].wrapping_add(4), cpu.r[1]);
    let _ = mem.write_u32(cpu.r[14].wrapping_add(8), cpu.r[1]);
    let _ = mem.write_u32(cpu.r[14].wrapping_add(12), cpu.r[1]);
    let _ = mem.write_u32(cpu.r[14].wrapping_add(16), cpu.r[1]);
    let _ = mem.write_u32(cpu.r[14].wrapping_add(20), cpu.r[1]);
    let _ = mem.write_u32(cpu.r[14].wrapping_add(24), cpu.r[1]);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[14] = mem.read_u32(cpu.r[15]).unwrap_or(0); // delay slot of `rts`
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}
