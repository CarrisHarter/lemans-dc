//! 0x8c224be2 - validate record `index` and scan 16 bytes for 0x55.
//!
//! Byte +2 clear, a failed 0x8c222430 (which then runs 0x8c222c3a), a failed
//! 0x8c222470, or byte +8 clear returns 0xffffff01. A byte other than 0x55
//! writes zeros at +11 and +12 and returns 0xffffff03. Sixteen 0x55 bytes
//! then return 0 only when `[0x8c2c0cf8 + 4*index]` is zero, else 0xffffff03.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[14]);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[13]);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    cpu.r[14] = mem.read_u16(0x8c224c6e).unwrap_or(0x0148) as i16 as i32 as u32;
    cpu.r[13] = cpu.r[4];
    cpu.r[3] = mem.read_u32(0x8c224c78).unwrap_or(0x8c2c02b8);
    let product = (cpu.r[14] as i32 as i64) * (cpu.r[13] as i32 as i64);
    cpu.mach = (product >> 32) as u32;
    cpu.macl = product as u32;
    cpu.r[14] = cpu.macl;
    let _ = mem.write_u32(cpu.r[15], cpu.r[14]);
    cpu.r[14] = cpu.r[14].wrapping_add(cpu.r[3]);
    cpu.r[0] = mem.read_u8(cpu.r[14].wrapping_add(2)).unwrap_or(0) as i8 as i32 as u32;
    cpu.set_t(cpu.r[0] == 0);
    let mut result: Option<u32> = None;
    if !cpu.t() {
        cpu.r[3] = mem.read_u32(0x8c224c7c).unwrap_or(0x8c222430);
        cpu.r[4] = cpu.r[13]; // jsr delay
        cpu.pr = 0x8c224c04;
        crate::registry::p_8c222430::run(cpu, mem);
        cpu.set_t(cpu.r[0] == 0);
        if cpu.t() {
            cpu.r[3] = mem.read_u32(0x8c224c80).unwrap_or(0x8c222c3a);
            cpu.r[4] = cpu.r[13];
            cpu.pr = 0x8c224c0e;
            crate::registry::p_8c222c3a::run(cpu, mem);
            result = Some(mem.read_u16(0x8c224c70).unwrap_or(0xff01) as i16 as i32 as u32);
        } else {
            cpu.r[2] = mem.read_u32(0x8c224c84).unwrap_or(0x8c222470);
            cpu.r[4] = cpu.r[13];
            cpu.pr = 0x8c224c18;
            crate::registry::p_8c222470::run(cpu, mem);
            cpu.set_t(cpu.r[0] == 0);
            if !cpu.t() {
                cpu.r[2] = mem.read_u32(cpu.r[15]).unwrap_or(0);
                cpu.r[3] = mem.read_u32(0x8c224c78).unwrap_or(0x8c2c02b8);
                cpu.r[2] = cpu.r[2].wrapping_add(cpu.r[3]);
                cpu.r[0] = mem.read_u8(cpu.r[2].wrapping_add(8)).unwrap_or(0) as i8 as i32 as u32;
                cpu.set_t(cpu.r[0] == 0);
                if !cpu.t() {
                    cpu.r[5] = mem.read_u32(cpu.r[14].wrapping_add(32)).unwrap_or(0);
                    cpu.r[4] = 0;
                    cpu.r[7] = 16;
                    cpu.r[6] = cpu.r[4];
                    let mut all_u = true;
                    loop {
                        cpu.r[0] = mem.read_u8(cpu.r[5]).unwrap_or(0) as i8 as i32 as u32;
                        cpu.r[0] &= 0xff;
                        cpu.set_t(cpu.r[0] == 85);
                        if !cpu.t() {
                            cpu.r[0] = cpu.r[4];
                            let _ = mem.write_u8(cpu.r[14].wrapping_add(11), cpu.r[0] as u8);
                            let _ = mem.write_u8(cpu.r[14].wrapping_add(12), cpu.r[0] as u8);
                            all_u = false;
                            break;
                        }
                        cpu.r[6] = cpu.r[6].wrapping_add(1);
                        cpu.set_t((cpu.r[6] as i32) >= (cpu.r[7] as i32));
                        cpu.r[5] = cpu.r[5].wrapping_add(1); // bf.s delay
                        if cpu.t() {
                            break;
                        }
                    }
                    if all_u {
                        cpu.r[0] = mem.read_u32(0x8c224c88).unwrap_or(0x8c2c0cf8);
                        cpu.r[13] <<= 2;
                        cpu.r[3] = mem.read_u32(cpu.r[0].wrapping_add(cpu.r[13])).unwrap_or(0);
                        cpu.set_t(cpu.r[3] == 0);
                        result = Some(if cpu.t() {
                            0
                        } else {
                            mem.read_u16(0x8c224c6a).unwrap_or(0xff03) as i16 as i32 as u32
                        });
                    } else {
                        result = Some(mem.read_u16(0x8c224c6a).unwrap_or(0xff03) as i16 as i32 as u32);
                    }
                }
            }
        }
    }
    if result.is_none() {
        result = Some(mem.read_u16(0x8c224c70).unwrap_or(0xff01) as i16 as i32 as u32);
    }
    cpu.r[0] = result.unwrap();
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[13] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[14] = mem.read_u32(cpu.r[15]).unwrap_or(0); // rts delay
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}
