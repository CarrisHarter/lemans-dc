//! 0x8c231420 - set bit 1 of `[global+4]`, clear it in `[global]`, store fr4 at +32.
//!
//! The `fmov` is the `bra` delay slot, so it runs before the second `or`.
//! `rts` delay sets r0 to 120. The loop at 0x8c231430 is skipped.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[5] = mem.read_u32(0x8c231460).unwrap_or(0x8c2c65d0);
    cpu.r[4] = 2;
    cpu.r[0] = 32;
    cpu.r[3] = mem.read_u32(cpu.r[5]).unwrap_or(0);
    cpu.r[3] |= cpu.r[4];
    let _ = mem.write_u32(cpu.r[5], cpu.r[3]);
    let _ = mem.write_f32(cpu.r[0].wrapping_add(cpu.r[5]), cpu.fr[4]); // bra delay
    cpu.r[3] = mem.read_u32(cpu.r[5].wrapping_add(4)).unwrap_or(0);
    cpu.r[3] |= cpu.r[4];
    let _ = mem.write_u32(cpu.r[5].wrapping_add(4), cpu.r[3]);
    cpu.r[2] = mem.read_u32(cpu.r[5]).unwrap_or(0);
    cpu.r[3] = (-3i32) as u32;
    cpu.r[2] &= cpu.r[3];
    let _ = mem.write_u32(cpu.r[5], cpu.r[2]);
    cpu.r[0] = 120; // rts delay
}
