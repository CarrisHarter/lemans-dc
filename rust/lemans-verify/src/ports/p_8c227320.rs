//! 0x8c227320 — insert_bits_179f8.
//!
//! Gated on bit mask `0x179f8` of `[r4+4]`; clears `0xffefffff` in the
//! target word and inserts `r5 & 1` at a fixed shift.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[2] = mem.read_u32(cpu.r[4] + 4).unwrap_or(0);
    cpu.r[3] = 96760;
    let t = (cpu.r[3] & cpu.r[2]) == 0;
    cpu.set_t(t);
    if t {
        cpu.r[0] = 10; // delay slot of `rts`
        return;
    }
    cpu.r[3] = 1;
    cpu.r[6] = cpu.r[4] + 4;
    cpu.r[5] = cpu.r[5] & cpu.r[3];
    cpu.r[2] = 4293918719;
    cpu.r[1] = mem.read_u32(cpu.r[6]).unwrap_or(0);
    cpu.set_t((cpu.r[5] >> 31) & 1 != 0);
    cpu.r[5] = cpu.r[5] << 16;
    cpu.r[5] = cpu.r[5] << 2;
    cpu.r[5] = cpu.r[5] << 2;
    cpu.r[1] = cpu.r[1] & cpu.r[2];
    cpu.r[5] = cpu.r[5] | cpu.r[1];
    cpu.r[0] = 0;
    let _ = mem.write_u32(cpu.r[6], cpu.r[5]);

}
