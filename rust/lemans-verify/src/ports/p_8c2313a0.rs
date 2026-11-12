//! 0x8c2313a0 - clamp r4 into 0..255 and poke the global at 0x8c2c65d0.
//!
//! `[global+24]` is 1 when r4 >= 0. `[global+28]` is r4 clamped to 255.
//! Bit 2 is set in `[global+4]` and cleared in `[global]`. The `bra` skips
//! the loop at 0x8c2313bc. `rts` delay zeroes r0.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[5] = mem.read_u16(0x8c231400).unwrap_or(0x00ff) as i16 as i32 as u32;
    cpu.set_t((cpu.r[4] as i32) >= 0); // cmp/pz r4
    cpu.r[6] = u32::from(cpu.t()); // movt r6
    cpu.set_t((cpu.r[4] as i32) > (cpu.r[5] as i32)); // cmp/gt r5,r4
    cpu.r[7] = 4; // bt.s delay
    if !cpu.t() {
        cpu.r[5] = cpu.r[4];
    }
    cpu.r[4] = mem.read_u32(0x8c231404).unwrap_or(0x8c2c65d0);
    cpu.r[2] = mem.read_u32(cpu.r[4]).unwrap_or(0);
    cpu.r[2] |= cpu.r[7];
    let _ = mem.write_u32(cpu.r[4], cpu.r[2]);
    let _ = mem.write_u32(cpu.r[4].wrapping_add(24), cpu.r[6]);
    let _ = mem.write_u32(cpu.r[4].wrapping_add(28), cpu.r[5]); // bra delay
    cpu.r[3] = mem.read_u32(cpu.r[4].wrapping_add(4)).unwrap_or(0);
    cpu.r[3] |= cpu.r[7];
    let _ = mem.write_u32(cpu.r[4].wrapping_add(4), cpu.r[3]);
    cpu.r[2] = mem.read_u32(cpu.r[4]).unwrap_or(0);
    cpu.r[3] = (-5i32) as u32;
    cpu.r[2] &= cpu.r[3];
    let _ = mem.write_u32(cpu.r[4], cpu.r[2]);
    cpu.r[0] = 0; // rts delay
}
