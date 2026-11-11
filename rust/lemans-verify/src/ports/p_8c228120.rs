//! 0x8c228120 — clear_bit_and_test.
//!
//! Clears the pending word at `0x8c2c6f90`, sets `0x8c2c6f9c` to 1, clears the
//! bit in `0x8c2c6f84`, and returns `([0x8c2c6f84] >> r4) & 1`.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[3] = mem.read_u32(0x8c228220).unwrap_or(0);
    cpu.r[5] = mem.read_u32(cpu.r[3]).unwrap_or(0);
    let t = cpu.r[5] == 0;
    cpu.set_t(t);
    cpu.r[6] = 1; // delay slot of `bt.s`
    if !t {
        cpu.r[1] = mem.read_u32(0x8c228220).unwrap_or(0);
        cpu.r[7] = 0;
        let _ = mem.write_u32(cpu.r[1], cpu.r[7]);
        cpu.r[2] = mem.read_u32(0x8c228224).unwrap_or(0);
        let _ = mem.write_u32(cpu.r[2], cpu.r[6]);
        cpu.r[3] = mem.read_u32(0x8c228228).unwrap_or(0);
        cpu.r[5] = !cpu.r[5];
        cpu.r[0] = mem.read_u32(cpu.r[3]).unwrap_or(0);
        cpu.r[0] = cpu.r[0] & cpu.r[5];
        let _ = mem.write_u32(cpu.r[3], cpu.r[0]);
        cpu.r[3] = cpu.r[3].wrapping_add(24);
        let _ = mem.write_u32(cpu.r[3], cpu.r[7]);
    }
    cpu.r[2] = mem.read_u32(0x8c228228).unwrap_or(0);
    cpu.r[4] = cpu.r[4].wrapping_neg();
    cpu.r[0] = mem.read_u32(cpu.r[2]).unwrap_or(0);
    cpu.r[0] = if (cpu.r[4] as i32) >= 0 { cpu.r[0] << cpu.r[4] } else { cpu.r[0] >> (cpu.r[4].wrapping_neg()) }; // shld
    cpu.r[0] = cpu.r[0] & cpu.r[6]; // delay slot of `rts`
}
