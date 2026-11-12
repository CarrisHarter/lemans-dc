//! 0x8c22299e - 1 when 0x8c222470(r4) is nonzero and byte +8 of the record is.
//!
//! The record is `0x8c2c02b8 + 0x148*r4`. The call keeps the entry `r4`; the
//! record pointer is only saved for the byte test.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr);
    cpu.r[15] = cpu.r[15].wrapping_sub(4); // add #-4,r15
    cpu.r[3] = mem.read_u16(0x8c222a90).unwrap_or(0x0148) as i16 as i32 as u32;
    cpu.r[2] = mem.read_u32(0x8c222a98).unwrap_or(0x8c2c02b8);
    let product = (cpu.r[3] as i32 as i64) * (cpu.r[4] as i32 as i64); // mul.l r3,r4
    cpu.mach = (product >> 32) as u32;
    cpu.macl = product as u32;
    cpu.r[1] = mem.read_u32(0x8c222a9c).unwrap_or(0x8c222470);
    cpu.r[3] = cpu.macl;
    cpu.r[3] = cpu.r[3].wrapping_add(cpu.r[2]);
    let _ = mem.write_u32(cpu.r[15], cpu.r[3]); // jsr delay
    cpu.pr = 0x8c2229b2;
    crate::registry::p_8c222470::run(cpu, mem);
    cpu.set_t(cpu.r[0] == 0); // tst r0,r0
    let mut hit = false;
    if !cpu.t() {
        cpu.r[2] = mem.read_u32(cpu.r[15]).unwrap_or(0);
        cpu.r[0] = mem.read_u8(cpu.r[2].wrapping_add(8)).unwrap_or(0) as i8 as i32 as u32;
        cpu.set_t(cpu.r[0] == 0); // tst r0,r0
        hit = !cpu.t();
    }
    if hit {
        cpu.r[15] = cpu.r[15].wrapping_add(4);
        cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0);
        cpu.r[15] = cpu.r[15].wrapping_add(4);
        cpu.r[0] = 1; // rts delay
    } else {
        cpu.r[0] = 0;
        cpu.r[15] = cpu.r[15].wrapping_add(4);
        cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0);
        cpu.r[15] = cpu.r[15].wrapping_add(4);
    }
}
