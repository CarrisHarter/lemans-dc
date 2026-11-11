//! 0x8c23a8a0 — insert_into_control_word.
//!
//! Clears `0xffa0ffff` in `[0x8c2c8698]`, inserts `(r5>>5)&31` at bit 16 and
//! `r4&1` at bit 22, then stores `r4` and `r5` at `0x8c2c8730`/`0x8c2c8734`.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[7] = cpu.r[5];
    let pz = (cpu.r[7] as i32) >= 0;
    cpu.set_t(pz);
    if !pz {
        cpu.r[7] = cpu.r[7].wrapping_add(31);
    }
    cpu.r[3] = (-5i32) as u32;
    cpu.r[2] = mem.read_u32(0x8c23a8e4).unwrap_or(0);
    cpu.r[7] = cpu.r[7] >> 5; // shad with a negative count
    cpu.r[3] = mem.read_u32(0x8c23a8e0).unwrap_or(0);
    cpu.r[6] = mem.read_u32(cpu.r[2]).unwrap_or(0);
    cpu.r[2] = 1;
    cpu.r[1] = 31;
    cpu.r[6] = cpu.r[6] & cpu.r[3];
    cpu.r[2] = cpu.r[2] & cpu.r[4];
    cpu.r[3] = 22;
    cpu.r[7] = cpu.r[7] & cpu.r[1];
    cpu.r[2] = cpu.r[2] << cpu.r[3]; // shad
    cpu.r[7] = cpu.r[7] << 16;
    cpu.r[7] = cpu.r[7] | cpu.r[2];
    cpu.r[2] = mem.read_u32(0x8c23a8e4).unwrap_or(0);
    cpu.r[6] = cpu.r[6] | cpu.r[7];
    let _ = mem.write_u32(cpu.r[2], cpu.r[6]);
    cpu.r[3] = mem.read_u32(0x8c23a8e8).unwrap_or(0);
    let _ = mem.write_u32(cpu.r[3], cpu.r[4]);
    cpu.r[1] = mem.read_u32(0x8c23a8ec).unwrap_or(0);
    let _ = mem.write_u32(cpu.r[1], cpu.r[5]);
    cpu.r[0] = 0; // delay slot of `rts`
}
