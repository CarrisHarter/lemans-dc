//! 0x8c231f4c - dispatch on the halfword and two flag bits, store the result.
//!
//! Halfword 3 calls 0x8c23213c. Otherwise bit 30 of `[r4+24]` calls
//! 0x8c232014, bit 15 calls 0x8c232096, and the remaining path calls
//! 0x8c231f90. The result is stored at `[r4+20]`.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[14]);
    cpu.r[14] = cpu.r[4];
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr);
    cpu.r[0] = mem.read_u32(cpu.r[14]).unwrap_or(0);
    cpu.r[0] &= 0xffff; // extu.w
    cpu.set_t(cpu.r[0] == 3);
    if cpu.t() {
        cpu.r[4] = cpu.r[14]; // bsr delay
        cpu.pr = 0x8c231f5e;
        crate::registry::p_8c23213c::run(cpu, mem);
    } else {
        cpu.r[2] = mem.read_u32(cpu.r[14].wrapping_add(24)).unwrap_or(0);
        cpu.r[3] = mem.read_u32(0x8c231fc0).unwrap_or(0x4000_0000);
        cpu.set_t((cpu.r[3] & cpu.r[2]) == 0); // tst r3,r2
        if !cpu.t() {
            cpu.r[4] = cpu.r[14];
            cpu.pr = 0x8c231f6e;
            crate::registry::p_8c232014::run(cpu, mem);
        } else {
            cpu.r[1] = mem.read_u32(cpu.r[14].wrapping_add(24)).unwrap_or(0);
            cpu.r[2] = mem.read_u32(0x8c231fc4).unwrap_or(0x8000);
            cpu.set_t((cpu.r[2] & cpu.r[1]) == 0);
            if !cpu.t() {
                cpu.r[4] = cpu.r[14];
                cpu.pr = 0x8c231f7e;
                crate::registry::p_8c232096::run(cpu, mem);
            } else {
                cpu.r[4] = cpu.r[14];
                cpu.pr = 0x8c231f86;
                crate::registry::p_8c231f90::run(cpu, mem);
            }
        }
    }
    cpu.r[4] = cpu.r[0];
    let _ = mem.write_u32(cpu.r[14].wrapping_add(20), cpu.r[4]);
    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[14] = mem.read_u32(cpu.r[15]).unwrap_or(0); // rts delay
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}
