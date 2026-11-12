//! 0x8c223812 - admit a record or return a negative status.
//!
//! A null object returns `0xffffff00`. A zero byte in the `0x148`-stride table
//! at `0x8c2c02ba`, a failed slot test, or a negative validator result returns
//! `0xffffff01` or `0xffffff03`. Success copies the caller's halfword and byte
//! into the object and returns 0.

use sh4_oracle::{Cpu, Memory};

fn sext16(mem: &Memory, addr: u32) -> u32 {
    mem.read_u16(addr).unwrap_or(0) as i16 as i32 as u32
}

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[14]);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr);
    cpu.r[15] = cpu.r[15].wrapping_sub(8);
    cpu.r[0] = cpu.r[5];
    cpu.r[14] = cpu.r[4];
    cpu.set_t(cpu.r[14] == 0); // tst r14,r14
    let _ = mem.write_u16(cpu.r[15].wrapping_add(4), cpu.r[0] as u16);
    let _ = mem.write_u8(cpu.r[15], cpu.r[6] as u8); // bf.s delay
    if cpu.t() {
        cpu.r[0] = sext16(mem, 0x8c223856);
    } else {
        cpu.r[2] = mem.read_u32(cpu.r[14].wrapping_add(40)).unwrap_or(0);
        cpu.r[3] = sext16(mem, 0x8c223858);
        cpu.r[0] = mem.read_u32(0x8c22387c).unwrap_or(0);
        let p = (cpu.r[3] as i32 as i64) * (cpu.r[2] as i32 as i64);
        cpu.mach = (p >> 32) as u32;
        cpu.macl = p as u32;
        cpu.r[3] = cpu.macl;
        cpu.r[1] = mem.read_u8(cpu.r[0].wrapping_add(cpu.r[3])).unwrap_or(0) as i8 as i32 as u32;
        cpu.set_t(cpu.r[1] == 0);
        if !cpu.t() {
            cpu.r[4] = mem.read_u32(cpu.r[14].wrapping_add(40)).unwrap_or(0); // jsr delay
            cpu.pr = 0x8c223844;
            crate::registry::p_8c222430::run(cpu, mem);
            cpu.set_t(cpu.r[0] == 0); // tst r0,r0
            if cpu.t() {
                cpu.r[4] = mem.read_u32(cpu.r[14].wrapping_add(40)).unwrap_or(0); // bsr delay
                cpu.pr = 0x8c22384c;
                crate::registry::p_8c222c3a::run(cpu, mem);
                cpu.r[0] = sext16(mem, 0x8c223980);
            } else {
                cpu.r[4] = mem.read_u32(cpu.r[14].wrapping_add(40)).unwrap_or(0); // jsr delay
                cpu.pr = 0x8c22388a;
                crate::registry::p_8c222470::run(cpu, mem);
                cpu.set_t(cpu.r[0] == 0);
                if cpu.t() {
                    cpu.r[0] = sext16(mem, 0x8c223980);
                } else {
                    cpu.r[2] = mem.read_u32(cpu.r[14].wrapping_add(40)).unwrap_or(0);
                    cpu.r[3] = sext16(mem, 0x8c22397e);
                    cpu.r[0] = mem.read_u32(0x8c22398c).unwrap_or(0);
                    let p = (cpu.r[3] as i32 as i64) * (cpu.r[2] as i32 as i64);
                    cpu.mach = (p >> 32) as u32;
                    cpu.macl = p as u32;
                    cpu.r[3] = cpu.macl;
                    cpu.r[1] = mem.read_u8(cpu.r[0].wrapping_add(cpu.r[3])).unwrap_or(0) as i8 as i32 as u32;
                    cpu.set_t(cpu.r[1] == 0);
                    if cpu.t() {
                        cpu.r[0] = sext16(mem, 0x8c223980);
                    } else {
                        cpu.r[4] = mem.read_u32(cpu.r[14].wrapping_add(40)).unwrap_or(0); // jsr delay
                        cpu.pr = 0x8c2238ae;
                        crate::registry::p_8c220f96::run(cpu, mem);
                        cpu.set_t((cpu.r[0] as i32) >= 0); // cmp/pz r0
                        if !cpu.t() {
                            cpu.r[0] = sext16(mem, 0x8c223982);
                        } else {
                            cpu.r[0] = mem.read_u16(cpu.r[15].wrapping_add(4)).unwrap_or(0) as i16 as i32 as u32;
                            let _ = mem.write_u16(cpu.r[14].wrapping_add(2), cpu.r[0] as u16);
                            cpu.r[0] = mem.read_u8(cpu.r[15]).unwrap_or(0) as i8 as i32 as u32;
                            let _ = mem.write_u8(cpu.r[14].wrapping_add(1), cpu.r[0] as u8);
                            cpu.r[0] = 0;
                        }
                    }
                }
            }
        } else {
            cpu.r[0] = sext16(mem, 0x8c223980);
        }
    }
    cpu.r[15] = cpu.r[15].wrapping_add(8);
    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[14] = mem.read_u32(cpu.r[15]).unwrap_or(0); // rts delay
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}
