//! 0x8c2342d8 - write two flag words from bits of r4.
//!
//! `[0x8c2c86ec]` is 0x200 if bit 5 is set and bits 1 and 6 are both set,
//! 0x1e0 if bit 5 is set otherwise, 0x100 if bit 5 is clear and bits 1 and 6
//! are both set, else 0xf0. `[0x8c2c86e8]` is 0x280 if bit 4 is set, else 0x140.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[3] = 32;
    cpu.set_t((cpu.r[4] & cpu.r[3]) == 0); // tst r4,r3
    cpu.r[5] = 66;
    let bit5_clear = cpu.t();
    cpu.r[5] &= cpu.r[4]; // bt.s delay
    if !bit5_clear {
        cpu.r[0] = cpu.r[5];
        cpu.set_t(cpu.r[0] == 66);
        if cpu.t() {
            cpu.r[3] = mem.read_u32(0x8c23433c).unwrap_or(0x8c2c86ec);
            cpu.r[2] = mem.read_u16(0x8c234314).unwrap_or(0x0200) as i16 as i32 as u32;
            let _ = mem.write_u32(cpu.r[3], cpu.r[2]); // bra delay
        } else {
            cpu.r[1] = mem.read_u32(0x8c23433c).unwrap_or(0x8c2c86ec);
            cpu.r[0] = mem.read_u16(0x8c234316).unwrap_or(0x01e0) as i16 as i32 as u32;
            let _ = mem.write_u32(cpu.r[1], cpu.r[0]); // bra delay
        }
    } else {
        cpu.r[0] = cpu.r[5];
        cpu.set_t(cpu.r[0] == 66);
        if cpu.t() {
            cpu.r[2] = mem.read_u32(0x8c234388).unwrap_or(0x8c2c86ec);
            cpu.r[3] = mem.read_u16(0x8c234380).unwrap_or(0x0100) as i16 as i32 as u32;
            let _ = mem.write_u32(cpu.r[2], cpu.r[3]); // bra delay
        } else {
            cpu.r[1] = mem.read_u32(0x8c234388).unwrap_or(0x8c2c86ec);
            cpu.r[0] = mem.read_u16(0x8c234382).unwrap_or(0x00f0) as i16 as i32 as u32;
            let _ = mem.write_u32(cpu.r[1], cpu.r[0]);
        }
    }
    cpu.r[3] = 16;
    cpu.set_t((cpu.r[3] & cpu.r[4]) == 0); // tst r3,r4
    if !cpu.t() {
        cpu.r[2] = mem.read_u32(0x8c23438c).unwrap_or(0x8c2c86e8);
        cpu.r[1] = mem.read_u16(0x8c234384).unwrap_or(0x0280) as i16 as i32 as u32;
        let _ = mem.write_u32(cpu.r[2], cpu.r[1]); // bra delay
    } else {
        cpu.r[0] = mem.read_u32(0x8c23438c).unwrap_or(0x8c2c86e8);
        cpu.r[2] = mem.read_u16(0x8c234386).unwrap_or(0x0140) as i16 as i32 as u32;
        let _ = mem.write_u32(cpu.r[0], cpu.r[2]);
    }
}
