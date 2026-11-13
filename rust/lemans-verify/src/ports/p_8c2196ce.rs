//! 0x8c2196ce - scan four flag bits for a record that just cleared.
//!
//! The byte at `*[0x8c2ba634] + 0xfb0` is tested one bit at a time. A set bit
//! loads the byte at `0x8c23c33c + index`, turns it into a 24-byte record and
//! a 120-byte slot, and copies the paired flag byte. It returns `index + 1`
//! when that slot's bit 7 is set, the record's byte at +4 is 8, byte +2 is
//! 0, the linked byte's bit 2 is clear, and the previous flag byte was not.

use sh4_oracle::{Cpu, Memory};

fn shad(value: u32, amount: i32) -> u32 {
    if amount >= 0 {
        value << (amount as u32 & 31)
    } else {
        ((value as i32) >> ((-amount) as u32 & 31)) as u32
    }
}

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    for reg in [14, 13, 12, 11, 10, 9, 8] {
        cpu.r[15] = cpu.r[15].wrapping_sub(4);
        let _ = mem.write_u32(cpu.r[15], cpu.r[reg]);
    }
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr);
    cpu.r[15] = cpu.r[15].wrapping_sub(8);
    cpu.r[2] = mem.read_u32(0x8c21976c).unwrap_or(0x8c2ba634);
    cpu.r[8] = 4;
    cpu.r[0] = mem.read_u16(0x8c21975a).unwrap_or(0x0fb0) as i16 as i32 as u32;
    cpu.r[10] = cpu.r[8];
    cpu.r[3] = mem.read_u32(cpu.r[2]).unwrap_or(0);
    cpu.r[9] = 1;
    cpu.r[11] = mem.read_u32(0x8c219770).unwrap_or(0x8c2b8e34);
    cpu.r[10] = cpu.r[10].wrapping_add(124);
    cpu.r[1] = mem.read_u8(cpu.r[0].wrapping_add(cpu.r[3])).unwrap_or(0) as i8 as i32 as u32;
    let _ = mem.write_u8(cpu.r[15], cpu.r[1] as u8);
    cpu.r[12] = mem.read_u32(0x8c219774).unwrap_or(0x8c2b8e30);
    cpu.r[14] = 0; // bra delay
    loop {
        cpu.r[3] = mem.read_u8(cpu.r[15]).unwrap_or(0) as i8 as i32 as u32;
        cpu.r[2] = cpu.r[9];
        cpu.r[2] = shad(cpu.r[2], cpu.r[14] as i32);
        cpu.r[3] &= 0xff;
        cpu.set_t(cpu.r[3] & cpu.r[2] == 0);
        if !cpu.t() {
            cpu.r[4] = mem.read_u32(0x8c219778).unwrap_or(0x8c23c33c);
            cpu.r[4] = cpu.r[4].wrapping_add(cpu.r[14]);
            let _ = mem.write_u32(cpu.r[15].wrapping_add(4), cpu.r[4]);
            cpu.r[4] = mem.read_u8(cpu.r[4]).unwrap_or(0) as i8 as i32 as u32;
            cpu.r[4] &= 0xff; // bsr delay
            cpu.pr = 0x8c219712;
            crate::registry::p_8c2199b8::run(cpu, mem);
            cpu.r[4] = mem.read_u32(cpu.r[15].wrapping_add(4)).unwrap_or(0);
            cpu.r[13] = cpu.r[0];
            cpu.r[4] = mem.read_u8(cpu.r[4]).unwrap_or(0) as i8 as i32 as u32;
            cpu.r[4] &= 0xff;
            cpu.pr = 0x8c21971c;
            crate::registry::p_8c2199ca::run(cpu, mem);
            cpu.r[4] = cpu.r[0];
            cpu.r[0] = cpu.r[14];
            cpu.r[3] = mem.read_u8(cpu.r[0].wrapping_add(cpu.r[11])).unwrap_or(0) as i8 as i32 as u32;
            let _ = mem.write_u8(cpu.r[0].wrapping_add(cpu.r[12]), cpu.r[3] as u8);
            cpu.r[2] = mem.read_u32(cpu.r[4]).unwrap_or(0);
            cpu.set_t(cpu.r[10] & cpu.r[2] == 0);
            if !cpu.t() {
                cpu.r[0] = mem.read_u8(cpu.r[13].wrapping_add(2)).unwrap_or(0) as i8 as i32 as u32;
                cpu.set_t(cpu.r[0] == 0);
                if !cpu.t() {
                    // bf: the byte was nonzero
                } else {
                    cpu.r[0] = mem.read_u8(cpu.r[13].wrapping_add(4)).unwrap_or(0) as i8 as i32 as u32;
                    cpu.r[0] &= 0xff;
                    cpu.set_t(cpu.r[0] == 8);
                    if cpu.t() {
                        cpu.r[4] = mem.read_u32(cpu.r[13].wrapping_add(8)).unwrap_or(0);
                        cpu.r[0] = mem.read_u8(cpu.r[4].wrapping_add(8)).unwrap_or(0) as i8 as i32 as u32;
                        cpu.r[0] &= 4;
                        cpu.r[4] = cpu.r[0];
                        cpu.r[0] = cpu.r[14];
                        let _ = mem.write_u8(cpu.r[0].wrapping_add(cpu.r[11]), cpu.r[4] as u8);
                        cpu.r[5] = mem.read_u8(cpu.r[0].wrapping_add(cpu.r[12])).unwrap_or(0) as i8 as i32 as u32;
                        cpu.r[5] ^= cpu.r[4];
                        cpu.r[4] &= 0xff;
                        cpu.set_t(cpu.r[4] != 0);
                        if !cpu.t() {
                            cpu.r[5] &= 0xff;
                            cpu.set_t(cpu.r[5] == 0);
                            if !cpu.t() {
                                cpu.r[0] = cpu.r[14].wrapping_add(1);
                                break;
                            }
                        }
                    }
                }
            }
        }
        cpu.r[14] = cpu.r[14].wrapping_add(1);
        cpu.set_t((cpu.r[14] as i32) >= (cpu.r[8] as i32));
        if cpu.t() {
            cpu.r[0] = 0;
            break;
        }
    }
    cpu.r[15] = cpu.r[15].wrapping_add(8);
    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    for reg in [8, 9, 10, 11, 12, 13, 14] {
        cpu.r[reg] = mem.read_u32(cpu.r[15]).unwrap_or(0);
        cpu.r[15] = cpu.r[15].wrapping_add(4);
    }
}
