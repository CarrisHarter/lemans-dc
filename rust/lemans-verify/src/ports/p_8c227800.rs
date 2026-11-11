//! 0x8c227800 — insert_bits_20000.
//!
//! Gated on bit mask `0x20000` of `[r4+4]`; clears `0xfbffffff` in the
//! target word and inserts `r5 & 1` at a fixed shift.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[2] = mem.read_u32(cpu.r[4] + 4).unwrap_or(0);
    cpu.r[3] = 131072;
    let t = (cpu.r[3] & cpu.r[2]) == 0;
    cpu.set_t(t);
    if !t {
        cpu.r[0] = 10; // delay slot of `rts`
        return;
    }
    cpu.r[3] = 1;
    cpu.r[6] = cpu.r[4] + 4;
    cpu.r[5] = cpu.r[5] & cpu.r[3];
    cpu.r[2] = 4227858431;
    cpu.r[1] = mem.read_u32(cpu.r[6]).unwrap_or(0);
    cpu.r[5] = cpu.r[5] << 16; // shll16 does not touch T
    cpu.r[5] = cpu.r[5] << 8; // shll8 does not touch T
    cpu.r[5] = cpu.r[5] << 2;
    cpu.r[1] = cpu.r[1] & cpu.r[2];
    cpu.r[5] = cpu.r[5] | cpu.r[1];
    cpu.r[0] = 0;
    let _ = mem.write_u32(cpu.r[6], cpu.r[5]);

}
