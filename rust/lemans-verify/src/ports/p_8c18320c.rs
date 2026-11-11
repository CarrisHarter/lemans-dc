//! 0x8c18320c — compare_table_prefix.
//!
//! Compares `[r4+20]` words of the tables at `[r4+12]` and `[r4+16]`; returns 0
//! on the first mismatch, otherwise 1 (the `rts` delay slot is `mov #1,r0`).

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[6] = mem.read_u32(cpu.r[4] + 20).unwrap_or(0);
    cpu.r[5] = 0;
    cpu.r[0] = cpu.r[5];
    let mut ok = true;
    while cpu.r[6] > cpu.r[5] {
        cpu.r[3] = mem.read_u32(cpu.r[4] + 16).unwrap_or(0);
        cpu.r[1] = mem.read_u32(cpu.r[4] + 12).unwrap_or(0);
        cpu.r[3] = mem.read_u32(cpu.r[0].wrapping_add(cpu.r[3])).unwrap_or(0);
        cpu.r[1] = mem.read_u32(cpu.r[0].wrapping_add(cpu.r[1])).unwrap_or(0);
        let t = cpu.r[1] == cpu.r[3];
        cpu.set_t(t);
        if !t {
            cpu.r[0] = 0; // delay slot of `rts`
            ok = false;
            break;
        }
        cpu.r[0] = cpu.r[0].wrapping_add(4);
        cpu.r[5] = cpu.r[5].wrapping_add(1);
        cpu.set_t(cpu.r[6] > cpu.r[5]);
    }
    if ok {
        cpu.set_t(cpu.r[6] > cpu.r[5]);
        cpu.r[0] = 1; // delay slot of `rts`
    }
}
