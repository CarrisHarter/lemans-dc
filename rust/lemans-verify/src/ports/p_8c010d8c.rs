//! 0x8c010d8c — classify a computed float (twin of 0x8c010b38).
//!
//! Calls 0x8c0148d0, then when the result's exponent is all ones it records
//! 0x44d (NaN) or 0x44c (infinity) in the global at 0x8c23d8b8, and returns
//! the float unchanged.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    cpu.r[3] = mem.read_u32(0x8c010dcc).unwrap_or(0);
    crate::registry::p_8c0148d0::run(cpu, mem);
    cpu.r[3] = cpu.r[15];
    cpu.r[4] = mem.read_u32(0x8c010dd0).unwrap_or(0);
    let _ = mem.write_f32(cpu.r[15], cpu.fr[0]);
    cpu.r[2] = mem.read_u32(cpu.r[3]).unwrap_or(0);
    cpu.r[2] = cpu.r[2] & cpu.r[4];
    let eq = cpu.r[2] == cpu.r[4];
    cpu.set_t(eq);
    if eq {
        cpu.r[2] = cpu.r[15];
        cpu.r[1] = mem.read_u32(cpu.r[2]).unwrap_or(0);
        cpu.r[3] = mem.read_u32(0x8c010dd4).unwrap_or(0);
        let t = (cpu.r[3] & cpu.r[1]) == 0;
        cpu.set_t(t);
        if t {
            cpu.r[1] = mem.read_u32(0x8c010dd8).unwrap_or(0);
            cpu.r[2] = mem.read_u16(0x8c010dc8).unwrap_or(0) as i16 as i32 as u32;
            let _ = mem.write_u32(cpu.r[1], cpu.r[2]);
        } else {
            cpu.r[2] = mem.read_u32(0x8c010dd8).unwrap_or(0);
            cpu.r[0] = mem.read_u16(0x8c010dc6).unwrap_or(0) as i16 as i32 as u32;
            let _ = mem.write_u32(cpu.r[2], cpu.r[0]); // delay slot of `bra`
        }
    }
    cpu.fr[0] = mem.read_f32(cpu.r[15]).unwrap_or(0.0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}
