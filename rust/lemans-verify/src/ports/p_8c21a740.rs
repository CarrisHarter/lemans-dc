//! 0x8c21a740 - 1 when slot `r4` is kind 14 and its 4-byte record unpacks to
//! a fixed pattern.
//!
//! 0x8c2199ca yields the slot. 0x8c219de6 unpacks the record into the frame.
//! A negative unpack, or any mismatched field, returns 0. The 1 is written
//! before the epilogue, not in the delay slot.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[14]);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr);
    cpu.r[15] = cpu.r[15].wrapping_sub(12); // add #-12,r15
    cpu.r[3] = mem.read_u32(0x8c21a7ac).unwrap_or(0x8c2199ca);
    let _ = mem.write_u32(cpu.r[15], cpu.r[4]); // jsr delay
    cpu.pr = 0x8c21a74c;
    crate::registry::p_8c2199ca::run(cpu, mem);
    cpu.r[2] = mem.read_u32(0x8c21a7b0).unwrap_or(0x8c219de6);
    cpu.r[4] = cpu.r[15];
    cpu.r[14] = cpu.r[0];
    cpu.r[5] = cpu.r[0];
    cpu.r[4] = cpu.r[4].wrapping_add(4); // jsr delay
    cpu.pr = 0x8c21a758;
    crate::registry::p_8c219de6::run(cpu, mem);
    cpu.r[4] = cpu.r[0];
    cpu.set_t((cpu.r[4] as i32) >= 0); // cmp/pz r4
    let mut hit = false;
    if cpu.t() {
        cpu.r[0] = mem.read_u32(cpu.r[14]).unwrap_or(0);
        cpu.set_t(cpu.r[0] == 14);
        if cpu.t() {
            cpu.r[0] = mem.read_u16(cpu.r[15].wrapping_add(4)).unwrap_or(0) as i16 as i32 as u32;
            cpu.r[0] &= 0xffff;
            cpu.set_t(cpu.r[0] == 1);
            if cpu.t() {
                cpu.r[0] = mem.read_u16(cpu.r[15].wrapping_add(6)).unwrap_or(0) as i16 as i32 as u32;
                cpu.r[3] = mem.read_u16(0x8c21a79e).unwrap_or(0x00c0) as i16 as i32 as u32;
                cpu.r[0] &= 0xffff;
                cpu.set_t(cpu.r[3] == cpu.r[0]);
                if cpu.t() {
                    cpu.r[0] = mem.read_u16(cpu.r[15].wrapping_add(8)).unwrap_or(0) as i16 as i32 as u32;
                    cpu.r[0] &= 0xffff;
                    cpu.set_t(cpu.r[0] == 1);
                    if cpu.t() {
                        cpu.r[0] = mem.read_u8(cpu.r[15].wrapping_add(10)).unwrap_or(0) as i8 as i32 as u32;
                        cpu.set_t(cpu.r[0] == 0);
                        if cpu.t() {
                            cpu.r[0] = mem.read_u8(cpu.r[15].wrapping_add(11)).unwrap_or(0) as i8 as i32 as u32;
                            cpu.set_t(cpu.r[0] == 0);
                            hit = cpu.t();
                        }
                    }
                }
            }
        }
    }
    cpu.r[0] = u32::from(hit);
    cpu.r[15] = cpu.r[15].wrapping_add(12);
    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[14] = mem.read_u32(cpu.r[15]).unwrap_or(0); // rts delay
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}
