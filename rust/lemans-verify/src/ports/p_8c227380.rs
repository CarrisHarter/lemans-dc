//! 0x8c227380 — insert_bits_20000_shift29.
//!
//! Gated on `0x20000` of `[r4+4]`; inserts the low 3 bits of `r5` at bit 29 of
//! `[r4+4]` via `shad`.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[2] = mem.read_u32(cpu.r[4] + 4).unwrap_or(0);
    cpu.r[3] = 0x20000;
    let t = (cpu.r[3] & cpu.r[2]) == 0;
    cpu.set_t(t);
    if !t {
        cpu.r[0] = 10; // delay slot of `rts`
        return;
    }
    cpu.r[6] = cpu.r[4].wrapping_add(4);
    cpu.r[1] = 0x1fffffff;
    cpu.r[3] = 7;
    cpu.r[0] = mem.read_u32(cpu.r[6]).unwrap_or(0);
    cpu.r[5] = cpu.r[5] & cpu.r[3];
    cpu.r[2] = 29;
    cpu.r[0] = cpu.r[0] & cpu.r[1];
    cpu.r[5] = cpu.r[5] << 29; // shad with a positive count
    cpu.r[5] = cpu.r[5] | cpu.r[0];
    cpu.r[0] = 0;
    let _ = mem.write_u32(cpu.r[6], cpu.r[5]);
}
