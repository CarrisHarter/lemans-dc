//! 0x8c17ba64 - fill an object from three globals and publish it.
//!
//! Stores two pointers that are immediately overwritten, inits `object+28`
//! via 0x8c1261d4, then copies three words from 0x8c20d600 and four halfwords
//! from `*[0x8c2ab468]+32`. `object+24` is set to the object itself.

use sh4_oracle::{Cpu, Memory};

fn sext16(v: u32) -> u32 {
    v as u16 as i16 as i32 as u32
}

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[14]);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[13]);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr);
    cpu.r[15] = cpu.r[15].wrapping_sub(24);
    let _ = mem.write_u32(cpu.r[15], cpu.r[4]);
    cpu.r[3] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[0] = mem.read_u32(0x8c17bb18).unwrap_or(0);
    cpu.r[1] = mem.read_u32(0x8c17bb1c).unwrap_or(0);
    let _ = mem.write_u32(cpu.r[3], cpu.r[0]);
    cpu.r[0] = mem.read_u32(0x8c17bb20).unwrap_or(0x8c1261d4);
    let _ = mem.write_u32(cpu.r[3], cpu.r[1]);
    cpu.r[4] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[4] = cpu.r[4].wrapping_add(28); // jsr delay
    cpu.pr = 0x8c17ba80;
    crate::registry::p_8c1261d4::run(cpu, mem);
    cpu.r[3] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[14] = cpu.r[15].wrapping_add(12);
    cpu.r[1] = mem.read_u32(0x8c17bb24).unwrap_or(0);
    cpu.r[0] = 40;
    let _ = mem.write_u32(cpu.r[3], cpu.r[1]);
    cpu.r[3] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[1] = 0;
    cpu.r[7] = mem.read_u32(0x8c17bb28).unwrap_or(0);
    let _ = mem.write_u16(cpu.r[0].wrapping_add(cpu.r[3]), cpu.r[1] as u16);
    cpu.r[4] = mem.read_u32(0x8c17bb2c).unwrap_or(0);
    cpu.r[0] = 42;
    cpu.r[5] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[3] = cpu.r[15].wrapping_add(4);
    let _ = mem.write_u16(cpu.r[0].wrapping_add(cpu.r[5]), cpu.r[1] as u16);
    cpu.r[5] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[0] = 44;
    let _ = mem.write_u16(cpu.r[0].wrapping_add(cpu.r[5]), cpu.r[1] as u16);
    cpu.r[5] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[0] = 46;
    let _ = mem.write_u16(cpu.r[0].wrapping_add(cpu.r[5]), cpu.r[1] as u16);
    cpu.r[13] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[0] = 20;
    cpu.r[6] = mem.read_u32(cpu.r[7]).unwrap_or(0);
    cpu.r[5] = mem.read_u32(cpu.r[7].wrapping_add(4)).unwrap_or(0);
    let _ = mem.write_u32(cpu.r[14], cpu.r[6]);
    let _ = mem.write_u32(cpu.r[14].wrapping_add(4), cpu.r[5]);
    cpu.r[5] = mem.read_u32(cpu.r[7].wrapping_add(8)).unwrap_or(0);
    let _ = mem.write_u32(cpu.r[14].wrapping_add(8), cpu.r[5]);
    let _ = mem.write_u32(cpu.r[13].wrapping_add(16), cpu.r[1]);
    cpu.r[5] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    let _ = mem.write_u8(cpu.r[0].wrapping_add(cpu.r[5]), cpu.r[1] as u8);
    cpu.r[5] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[1] = mem.read_u32(cpu.r[14]).unwrap_or(0);
    cpu.r[5] = cpu.r[5].wrapping_add(4);
    cpu.r[0] = mem.read_u32(cpu.r[14].wrapping_add(4)).unwrap_or(0);
    let _ = mem.write_u32(cpu.r[5], cpu.r[1]);
    let _ = mem.write_u32(cpu.r[5].wrapping_add(4), cpu.r[0]);
    cpu.r[0] = mem.read_u32(cpu.r[14].wrapping_add(8)).unwrap_or(0);
    let _ = mem.write_u32(cpu.r[5].wrapping_add(8), cpu.r[0]);
    cpu.r[0] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    let _ = mem.write_u32(cpu.r[0].wrapping_add(24), cpu.r[0]);
    cpu.r[6] = mem.read_u32(cpu.r[4]).unwrap_or(0);
    cpu.r[1] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[0] = cpu.r[6].wrapping_add(32);
    cpu.r[0] = sext16(mem.read_u16(cpu.r[0]).unwrap_or(0) as u32);
    cpu.r[5] = cpu.r[6];
    cpu.r[4] = cpu.r[6];
    let _ = mem.write_u16(cpu.r[15].wrapping_add(4), cpu.r[0] as u16);
    cpu.r[5] = cpu.r[5].wrapping_add(34);
    cpu.r[0] = sext16(mem.read_u16(cpu.r[5]).unwrap_or(0) as u32);
    cpu.r[4] = cpu.r[4].wrapping_add(36);
    cpu.r[6] = cpu.r[6].wrapping_add(38);
    let _ = mem.write_u16(cpu.r[15].wrapping_add(6), cpu.r[0] as u16);
    cpu.r[1] = cpu.r[1].wrapping_add(40);
    cpu.r[0] = sext16(mem.read_u16(cpu.r[4]).unwrap_or(0) as u32);
    let _ = mem.write_u16(cpu.r[15].wrapping_add(8), cpu.r[0] as u16);
    cpu.r[0] = sext16(mem.read_u16(cpu.r[6]).unwrap_or(0) as u32);
    let _ = mem.write_u16(cpu.r[15].wrapping_add(10), cpu.r[0] as u16);
    cpu.r[0] = sext16(mem.read_u16(cpu.r[3]).unwrap_or(0) as u32);
    let _ = mem.write_u16(cpu.r[1], cpu.r[0] as u16);
    cpu.r[0] = sext16(mem.read_u16(cpu.r[3].wrapping_add(2)).unwrap_or(0) as u32);
    let _ = mem.write_u16(cpu.r[1].wrapping_add(2), cpu.r[0] as u16);
    cpu.r[0] = sext16(mem.read_u16(cpu.r[3].wrapping_add(4)).unwrap_or(0) as u32);
    let _ = mem.write_u16(cpu.r[1].wrapping_add(4), cpu.r[0] as u16);
    cpu.r[0] = sext16(mem.read_u16(cpu.r[3].wrapping_add(6)).unwrap_or(0) as u32);
    let _ = mem.write_u16(cpu.r[1].wrapping_add(6), cpu.r[0] as u16);
    cpu.r[0] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(24);
    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[13] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[14] = mem.read_u32(cpu.r[15]).unwrap_or(0); // rts delay
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}
