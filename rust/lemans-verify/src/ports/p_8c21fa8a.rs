//! 0x8c21fa8a — mode_code.
//!
//! Switches on the global at `0x8c2bdcb8`: 0 scans `[0x8c2bdcb4]` for a non-zero
//! byte at `+28` and returns `byte[obj+20]`; 1 returns 13 when bit 2 of
//! `[[0x8c2bdcb0]+28]` is set; anything else returns `r5` unchanged.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[3] = mem.read_u32(0x8c21fb38).unwrap_or(0);
    cpu.r[0] = mem.read_u32(cpu.r[3]).unwrap_or(0);
    let eq0 = cpu.r[0] == 0;
    cpu.set_t(eq0);
    if eq0 {
        cpu.r[2] = mem.read_u32(0x8c21fb28).unwrap_or(0);
        cpu.r[4] = mem.read_u32(cpu.r[2]).unwrap_or(0);
        loop {
            cpu.r[0] = mem.read_u16(cpu.r[4] + 28).unwrap_or(0) as i16 as i32 as u32;
            cpu.r[3] = cpu.r[0] & 0xff;
            let t = cpu.r[3] == 0;
            cpu.set_t(t);
            if !t {
                break;
            }
        }
        cpu.r[5] = cpu.r[4];
        cpu.r[5] = cpu.r[5].wrapping_add(20);
        cpu.r[5] = mem.read_u8(cpu.r[5]).unwrap_or(0) as u32; // delay slot of `bra`
    } else {
        let eq1 = cpu.r[0] == 1;
        cpu.set_t(eq1);
        if eq1 {
            cpu.r[2] = mem.read_u32(0x8c21fb34).unwrap_or(0);
            cpu.r[0] = mem.read_u32(cpu.r[2]).unwrap_or(0);
            cpu.r[0] = cpu.r[0].wrapping_add(28);
            cpu.r[0] = mem.read_u8(cpu.r[0]).unwrap_or(0) as u32;
            cpu.r[0] = cpu.r[0] & 0xff;
            cpu.r[0] = cpu.r[0] & 4;
            let eq4 = cpu.r[0] == 4;
            cpu.set_t(eq4);
            if eq4 {
                cpu.r[5] = 13;
            }
        }
    }
    cpu.r[0] = cpu.r[5]; // delay slot of `rts`
}
