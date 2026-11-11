//! 0x8c2259d6 — alloc_slot.
//!
//! Scans 72-byte slots from `0x8c2c4f78` up to `[0x8c2c51b8]` slots. The first
//! with a zero flag byte gets flag 1, `[slot+12] = index`, and is returned;
//! otherwise `0x8c2c51bc` is set to `0x7ff` and 0 is returned.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[7] = mem.read_u32(0x8c225a24).unwrap_or(0);
    cpu.r[6] = 1;
    cpu.r[4] = mem.read_u32(0x8c225a20).unwrap_or(0);
    cpu.r[5] = 0;
    loop {
        cpu.r[3] = mem.read_u32(cpu.r[7]).unwrap_or(0);
        let ge = (cpu.r[5] as i32) >= (cpu.r[3] as i32);
        cpu.set_t(ge);
        if ge {
            break;
        }
        cpu.r[2] = mem.read_u8(cpu.r[4]).unwrap_or(0) as u32;
        let t = cpu.r[2] == 0;
        cpu.set_t(t);
        if t {
            let _ = mem.write_u8(cpu.r[4], cpu.r[6] as u8);
            let _ = mem.write_u32(cpu.r[4] + 12, cpu.r[5]);
            cpu.r[0] = cpu.r[4]; // delay slot of `rts`
            return;
        }
        cpu.r[4] = cpu.r[4].wrapping_add(72);
        cpu.r[5] = cpu.r[5].wrapping_add(1);
    }
    cpu.r[2] = mem.read_u32(0x8c225a28).unwrap_or(0);
    cpu.r[0] = 0;
    cpu.r[1] = mem.read_u16(0x8c225a12).unwrap_or(0) as i16 as i32 as u32;
    let _ = mem.write_u32(cpu.r[2], cpu.r[1]);
}
