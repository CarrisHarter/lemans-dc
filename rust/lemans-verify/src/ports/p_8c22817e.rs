//! 0x8c22817e — set_bit_and_bump.
//!
//! Sets bit `r4` in `0x8c2c6f80`, clears `0x8c2c6f98`, and when `0x8c2c7e24` is
//! non-zero increments and masks the counter at `0x8c2c7e20`.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[3] = mem.read_u32(0x8c228234).unwrap_or(0);
    cpu.r[5] = 1;
    let _ = mem.write_u32(cpu.r[3], cpu.r[5]);
    cpu.r[2] = mem.read_u32(0x8c228238).unwrap_or(0);
    cpu.r[5] = cpu.r[5] << cpu.r[4]; // shad
    cpu.r[1] = mem.read_u32(cpu.r[2]).unwrap_or(0);
    cpu.r[1] = cpu.r[1] | cpu.r[5];
    let _ = mem.write_u32(cpu.r[2], cpu.r[1]);
    cpu.r[0] = cpu.r[3];
    cpu.r[2] = 0;
    let _ = mem.write_u32(cpu.r[0], cpu.r[2]);
    cpu.r[2] = mem.read_u32(0x8c22823c).unwrap_or(0);
    cpu.r[1] = mem.read_u32(cpu.r[2]).unwrap_or(0);
    let t = cpu.r[1] == 0;
    cpu.set_t(t);
    if !t {
        cpu.r[3] = mem.read_u32(0x8c228240).unwrap_or(0);
        cpu.r[0] = mem.read_u32(cpu.r[3]).unwrap_or(0);
        cpu.r[0] = cpu.r[0].wrapping_add(1);
        cpu.r[1] = cpu.r[3];
        cpu.r[0] = cpu.r[0] & 1;
        let _ = mem.write_u32(cpu.r[1], cpu.r[0]);
    }
}
