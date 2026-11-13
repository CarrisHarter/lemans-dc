//! 0x8c23a640 - fill a table entry after a stride search hits.
//!
//! 0x8c237780 returns 0 when `r4` is not an entry of the table at
//! `[0x8c2377c4]`; this returns the pool constant 0xeeee and leaves that
//! residue. On a hit it stores r7/r5/r6 into the entry, packs `(r6-1)<<16 |
//! (r5-1)` at +56, and derives the word at +36 from the two dwords at +24
//! minus `24 * r5 * r6 * r7`. Bit 7 of `[0x8c2c9da4]` subtracts another 24.
//! The pointer at +0x88 then receives four globals.

use sh4_oracle::{Cpu, Memory};

fn mul(cpu: &mut Cpu, a: u32, b: u32) -> u32 {
    let product = (a as i32 as i64).wrapping_mul(b as i32 as i64);
    cpu.macl = product as u32;
    cpu.mach = (product >> 32) as u32;
    cpu.macl
}

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr);
    cpu.r[15] = cpu.r[15].wrapping_sub(16);
    cpu.r[3] = mem.read_u32(0x8c23a704).unwrap_or(0x8c237780);
    let _ = mem.write_u32(cpu.r[15].wrapping_add(8), cpu.r[4]);
    let _ = mem.write_u32(cpu.r[15].wrapping_add(4), cpu.r[5]);
    let _ = mem.write_u32(cpu.r[15], cpu.r[6]);
    let _ = mem.write_u32(cpu.r[15].wrapping_add(12), cpu.r[7]);
    cpu.r[4] = mem.read_u32(cpu.r[15].wrapping_add(8)).unwrap_or(0);
    cpu.pr = 0x8c23a652;
    crate::registry::p_8c237780::run(cpu, mem);
    cpu.set_t(cpu.r[0] == 0);
    if cpu.r[0] == 0 {
        cpu.r[0] = mem.read_u32(0x8c23a708).unwrap_or(0xeeee);
        cpu.r[15] = cpu.r[15].wrapping_add(16);
        cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0);
        cpu.r[15] = cpu.r[15].wrapping_add(4);
        return;
    }
    cpu.r[4] = mem.read_u32(cpu.r[15].wrapping_add(8)).unwrap_or(0);
    cpu.r[0] = mem.read_u16(0x8c23a700).unwrap_or(0x88) as i16 as i32 as u32;
    cpu.r[5] = cpu.r[4].wrapping_add(24);
    cpu.r[6] = mem.read_u32(cpu.r[4].wrapping_add(cpu.r[0])).unwrap_or(0);
    cpu.r[3] = mem.read_u32(cpu.r[15].wrapping_add(12)).unwrap_or(0);
    let _ = mem.write_u32(cpu.r[4].wrapping_add(12), cpu.r[3]);
    cpu.r[2] = mem.read_u32(cpu.r[15].wrapping_add(4)).unwrap_or(0);
    let _ = mem.write_u32(cpu.r[4].wrapping_add(4), cpu.r[2]);
    cpu.r[3] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    let _ = mem.write_u32(cpu.r[4].wrapping_add(8), cpu.r[3]);
    cpu.r[2] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[3] = mem.read_u32(cpu.r[15].wrapping_add(4)).unwrap_or(0);
    cpu.r[2] = cpu.r[2].wrapping_add((-1i32) as u32);
    cpu.r[2] = cpu.r[2].wrapping_shl(16);
    cpu.r[3] = cpu.r[3].wrapping_add((-1i32) as u32);
    cpu.r[2] |= cpu.r[3];
    let _ = mem.write_u32(cpu.r[4].wrapping_add(56), cpu.r[2]);
    cpu.r[3] = mem.read_u32(cpu.r[5]).unwrap_or(0);
    let _ = mem.write_u32(cpu.r[5].wrapping_add(8), cpu.r[3]);
    cpu.r[3] = mem.read_u32(cpu.r[5].wrapping_add(4)).unwrap_or(0);
    cpu.r[2] = mem.read_u32(cpu.r[5]).unwrap_or(0);
    cpu.r[1] = mem.read_u32(cpu.r[4].wrapping_add(4)).unwrap_or(0);
    cpu.r[2] = cpu.r[2].wrapping_add(cpu.r[3]);
    cpu.r[3] = mem.read_u32(cpu.r[4].wrapping_add(8)).unwrap_or(0);
    let low = mul(cpu, cpu.r[3], cpu.r[1]);
    cpu.r[3] = mem.read_u32(cpu.r[4].wrapping_add(12)).unwrap_or(0);
    cpu.r[0] = mul(cpu, cpu.r[3], low);
    cpu.r[3] = cpu.r[0];
    cpu.r[0] = cpu.r[0].wrapping_shl(1);
    cpu.r[1] = mem.read_u32(0x8c23a710).unwrap_or(0);
    cpu.r[0] = cpu.r[0].wrapping_add(cpu.r[3]);
    cpu.r[0] = cpu.r[0].wrapping_shl(2);
    cpu.r[0] = cpu.r[0].wrapping_shl(1);
    cpu.r[2] = cpu.r[2].wrapping_sub(cpu.r[0]);
    let _ = mem.write_u32(cpu.r[5].wrapping_add(12), cpu.r[2]);
    cpu.r[0] = mem.read_u32(cpu.r[1]).unwrap_or(0);
    cpu.set_t(cpu.r[0] & 128 == 0);
    if !cpu.t() {
        cpu.r[2] = mem.read_u32(cpu.r[5].wrapping_add(12)).unwrap_or(0);
        cpu.r[2] = cpu.r[2].wrapping_add((-24i32) as u32);
        let _ = mem.write_u32(cpu.r[5].wrapping_add(12), cpu.r[2]);
    }
    cpu.r[1] = mem.read_u32(cpu.r[5].wrapping_add(12)).unwrap_or(0);
    cpu.r[3] = mem.read_u16(0x8c23a702).unwrap_or(0xff00) as i16 as i32 as u32;
    cpu.r[0] = mem.read_u32(0x8c23a714).unwrap_or(0);
    cpu.r[1] = cpu.r[1].wrapping_add(cpu.r[3]);
    let _ = mem.write_u32(cpu.r[5].wrapping_add(36), cpu.r[1]);
    cpu.r[2] = mem.read_u32(cpu.r[0]).unwrap_or(0);
    let _ = mem.write_u32(cpu.r[6].wrapping_add(20), cpu.r[2]);
    cpu.r[2] = mem.read_u32(0x8c23a718).unwrap_or(0);
    cpu.r[1] = mem.read_u32(cpu.r[2]).unwrap_or(0);
    let _ = mem.write_u32(cpu.r[6].wrapping_add(24), cpu.r[1]);
    cpu.r[1] = mem.read_u32(0x8c23a71c).unwrap_or(0);
    cpu.r[3] = mem.read_u32(cpu.r[1]).unwrap_or(0);
    let _ = mem.write_u32(cpu.r[6].wrapping_add(16), cpu.r[3]);
    cpu.r[3] = mem.read_u32(0x8c23a720).unwrap_or(0);
    cpu.r[0] = mem.read_u32(cpu.r[3]).unwrap_or(0);
    let _ = mem.write_u32(cpu.r[6].wrapping_add(4), cpu.r[0]);
    cpu.r[0] = 0;
    cpu.r[15] = cpu.r[15].wrapping_add(16);
    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}
