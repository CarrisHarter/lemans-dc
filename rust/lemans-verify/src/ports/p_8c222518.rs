//! 0x8c222518 — install_mode_handlers.
//!
//! Masks `r4` to one bit, stores it through the pointer at `0x8c2c02b4`, and
//! installs one of two handler pairs at `+12`/`+16` of the object behind it.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[5] = mem.read_u32(0x8c2225cc).unwrap_or(0);
    cpu.r[3] = 1;
    cpu.r[4] = cpu.r[4] & cpu.r[3];
    cpu.r[2] = mem.read_u32(cpu.r[5]).unwrap_or(0);
    cpu.r[0] = cpu.r[4];
    let eq = cpu.r[0] == 1;
    cpu.set_t(eq);
    let _ = mem.write_u32(cpu.r[2], cpu.r[4]);
    cpu.r[6] = mem.read_u32(cpu.r[5]).unwrap_or(0); // delay slot of `bf.s`
    if eq {
        cpu.r[2] = mem.read_u32(0x8c2225d0).unwrap_or(0);
        let _ = mem.write_u32(cpu.r[6] + 12, cpu.r[2]);
        cpu.r[2] = mem.read_u32(0x8c2225d4).unwrap_or(0);
        cpu.r[1] = mem.read_u32(cpu.r[5]).unwrap_or(0);
        let _ = mem.write_u32(cpu.r[1] + 16, cpu.r[2]);
    } else {
        cpu.r[1] = mem.read_u32(0x8c2225dc).unwrap_or(0);
        cpu.r[0] = mem.read_u32(0x8c2225d8).unwrap_or(0);
        let _ = mem.write_u32(cpu.r[6] + 12, cpu.r[0]);
        cpu.r[2] = mem.read_u32(cpu.r[5]).unwrap_or(0);
        let _ = mem.write_u32(cpu.r[2] + 16, cpu.r[1]);
    }
}
