//! 0x8c221d54 — fill_stride_116.
//!
//! Stores `r4` at `[[0x8c2bde98] + 0x80*i + 116]` for i in `[0, 8)`, i.e.
//! until the pointer reaches base+0x400.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    cpu.r[5] = mem.read_u32(0x8c221e14).unwrap_or(0);
    cpu.r[3] = mem.read_u16(0x8c221e12).unwrap_or(0) as i16 as i32 as u32;
    cpu.r[7] = cpu.r[5];
    cpu.r[6] = mem.read_u16(0x8c221e10).unwrap_or(0) as i16 as i32 as u32;
    cpu.r[7] = cpu.r[7].wrapping_add(cpu.r[3]);
    let _ = mem.write_u32(cpu.r[15], cpu.r[5]);
    loop {
        cpu.r[0] = 116;
        let _ = mem.write_u32(cpu.r[0].wrapping_add(cpu.r[5]), cpu.r[4]);
        cpu.r[5] = cpu.r[5].wrapping_add(cpu.r[6]);
        let hs = cpu.r[5] >= cpu.r[7];
        cpu.set_t(hs);
        if hs {
            break;
        }
    }
    cpu.r[15] = cpu.r[15].wrapping_add(4); // delay slot of `rts`
}
