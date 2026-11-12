//! 0x8c2259bc - store 1 at `record + 72 + (r5 >> 4)`.
//!
//! `shad` with -4 is an arithmetic right shift. The store is the `rts` delay
//! slot. `r0` ends as `record+72`.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[3] = mem.read_u16(0x8c225a10).unwrap_or(0x0148) as i16 as i32 as u32;
    cpu.r[1] = (-4i32) as u32; // mov #-4,r1
    cpu.r[2] = mem.read_u32(0x8c225a14).unwrap_or(0x8c2c02b8);
    let amount = cpu.r[1] as i32; // shad r1,r5
    if amount >= 0 {
        cpu.r[5] <<= (amount as u32) & 31;
    } else {
        cpu.r[5] = ((cpu.r[5] as i32) >> (((-amount) as u32) & 31)) as u32;
    }
    let product = (cpu.r[3] as i32 as i64) * (cpu.r[4] as i32 as i64); // mul.l r3,r4
    cpu.mach = (product >> 32) as u32;
    cpu.macl = product as u32;
    cpu.r[1] = 1; // mov #1,r1
    cpu.r[4] = cpu.macl;
    cpu.r[4] = cpu.r[4].wrapping_add(cpu.r[2]);
    cpu.r[0] = cpu.r[4];
    cpu.r[0] = cpu.r[0].wrapping_add(72); // add #72,r0
    cpu.r[5] = cpu.r[5].wrapping_add(cpu.r[0]); // add r0,r5
    let _ = mem.write_u8(cpu.r[5], cpu.r[1] as u8); // rts delay: mov.b r1,@r5
}
