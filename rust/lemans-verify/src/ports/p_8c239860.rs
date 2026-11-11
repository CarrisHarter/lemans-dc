//! 0x8c239860 — find_slot_by_key.
//!
//! Scans 44-byte slots from `0x8c2c9e88+0x254` up to `+0x254+0xa0` for the one
//! equal to `r4`, returning 1 if found and 0 otherwise.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[5] = mem.read_u16(0x8c2398e2).unwrap_or(0) as i16 as i32 as u32;
    cpu.r[1] = 0;
    cpu.r[6] = mem.read_u32(0x8c2398e8).unwrap_or(0);
    cpu.r[0] = 1;
    cpu.r[7] = mem.read_u16(0x8c2398e0).unwrap_or(0) as i16 as i32 as u32;
    cpu.r[5] = cpu.r[5].wrapping_add(cpu.r[6]);
    loop {
        let eq = cpu.r[4] == cpu.r[5];
        cpu.set_t(eq);
        cpu.r[5] = cpu.r[5].wrapping_add(44); // delay slot of `bf.s`
        if eq {
            cpu.r[1] = cpu.r[0];
        }
        cpu.r[2] = mem.read_u16(0x8c2398e2).unwrap_or(0) as i16 as i32 as u32;
        cpu.r[2] = cpu.r[2].wrapping_add(cpu.r[6]);
        cpu.r[2] = cpu.r[2].wrapping_add(cpu.r[7]);
        let hs = cpu.r[5] >= cpu.r[2];
        cpu.set_t(hs);
        if hs {
            break;
        }
    }
    cpu.r[0] = cpu.r[1]; // delay slot of `rts`
}
