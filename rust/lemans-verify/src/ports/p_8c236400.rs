//! 0x8c236400 - set or clear bit 16 of `[slot+36]`, or return 0xeeee.
//!
//! 0x8c237780(r4) returning 0 takes the 0xeeee path. A nonzero r5 sets the
//! bit; a zero r5 clears it with the mask 0xfffeffff. Success returns 0.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr);
    cpu.r[15] = cpu.r[15].wrapping_sub(8);
    cpu.r[3] = mem.read_u32(0x8c236464).unwrap_or(0x8c237780);
    let _ = mem.write_u32(cpu.r[15], cpu.r[4]);
    let _ = mem.write_u32(cpu.r[15].wrapping_add(4), cpu.r[5]);
    cpu.r[4] = mem.read_u32(cpu.r[15]).unwrap_or(0); // jsr delay
    cpu.pr = 0x8c23640e;
    crate::registry::p_8c237780::run(cpu, mem);
    cpu.set_t(cpu.r[0] == 0); // tst r0,r0
    if cpu.t() {
        cpu.r[0] = mem.read_u32(0x8c236468).unwrap_or(0xeeee);
        cpu.r[15] = cpu.r[15].wrapping_add(8);
        cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0);
        cpu.r[15] = cpu.r[15].wrapping_add(4);
        return;
    }
    cpu.r[0] = mem.read_u16(0x8c236460).unwrap_or(0x0088) as i16 as i32 as u32;
    cpu.r[4] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[3] = mem.read_u32(cpu.r[15].wrapping_add(4)).unwrap_or(0);
    cpu.r[4] = mem.read_u32(cpu.r[4].wrapping_add(cpu.r[0])).unwrap_or(0);
    cpu.set_t(cpu.r[3] == 0); // tst r3,r3
    if !cpu.t() {
        cpu.r[2] = mem.read_u32(cpu.r[4].wrapping_add(36)).unwrap_or(0);
        cpu.r[3] = mem.read_u32(0x8c23646c).unwrap_or(0x10000);
        cpu.r[2] |= cpu.r[3];
        let _ = mem.write_u32(cpu.r[4].wrapping_add(36), cpu.r[2]); // bra delay
    } else {
        cpu.r[1] = mem.read_u32(cpu.r[4].wrapping_add(36)).unwrap_or(0);
        cpu.r[3] = mem.read_u32(0x8c236470).unwrap_or(0xfffeffff);
        cpu.r[1] &= cpu.r[3];
        let _ = mem.write_u32(cpu.r[4].wrapping_add(36), cpu.r[1]);
    }
    cpu.r[0] = 0;
    cpu.r[15] = cpu.r[15].wrapping_add(8);
    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}
