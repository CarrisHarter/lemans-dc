//! 0x8c232dde - same as 0x8c232da0, storing the packed pair at `[slot+24]`.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr);
    cpu.r[15] = cpu.r[15].wrapping_sub(12);
    cpu.r[3] = mem.read_u32(0x8c232e24).unwrap_or(0x8c237780);
    let _ = mem.write_u32(cpu.r[15], cpu.r[4]);
    let _ = mem.write_u32(cpu.r[15].wrapping_add(4), cpu.r[5]);
    let _ = mem.write_u32(cpu.r[15].wrapping_add(8), cpu.r[6]);
    cpu.r[4] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.pr = 0x8c232dee;
    crate::registry::p_8c237780::run(cpu, mem);
    cpu.set_t(cpu.r[0] == 0);
    if cpu.t() {
        cpu.r[0] = mem.read_u32(0x8c232e28).unwrap_or(0xeeee);
        cpu.r[15] = cpu.r[15].wrapping_add(12);
        cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0);
        cpu.r[15] = cpu.r[15].wrapping_add(4);
        return;
    }
    cpu.r[5] = mem.read_u16(0x8c232e22).unwrap_or(0x07ff) as i16 as i32 as u32;
    cpu.r[3] = mem.read_u32(cpu.r[15].wrapping_add(8)).unwrap_or(0);
    cpu.r[2] = mem.read_u32(cpu.r[15].wrapping_add(4)).unwrap_or(0);
    cpu.r[4] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[3] &= cpu.r[5];
    cpu.r[0] = mem.read_u16(0x8c232e20).unwrap_or(0x0088) as i16 as i32 as u32;
    cpu.r[2] &= cpu.r[5];
    cpu.r[3] <<= 16;
    cpu.r[4] = mem.read_u32(cpu.r[4].wrapping_add(cpu.r[0])).unwrap_or(0);
    cpu.r[3] |= cpu.r[2];
    cpu.r[0] = 0;
    let _ = mem.write_u32(cpu.r[4].wrapping_add(24), cpu.r[3]);
    cpu.r[15] = cpu.r[15].wrapping_add(12);
    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}
