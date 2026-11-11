//! 0x8c23bbc0 — list_max.
//!
//! Walks two lists from `0x8c23d7bc` and `0x8c23d7cc` and returns the largest
//! `[node+16]` of each through `r4` and `r5`.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[6] = mem.read_u32(0x8c23bc00).unwrap_or(0);
    cpu.r[0] = 0;
    cpu.r[6] = mem.read_u32(cpu.r[6] + 8).unwrap_or(0);
    let t = cpu.r[6] == 0;
    cpu.set_t(t);
    cpu.r[7] = cpu.r[0]; // delay slot of `bt.s`
    if !t {
        loop {
            cpu.r[2] = mem.read_u32(cpu.r[6] + 16).unwrap_or(0);
            let hi = cpu.r[2] > cpu.r[7];
            cpu.set_t(hi);
            if hi {
                cpu.r[7] = mem.read_u32(cpu.r[6] + 16).unwrap_or(0);
            }
            cpu.r[6] = mem.read_u32(cpu.r[6] + 8).unwrap_or(0);
            let t2 = cpu.r[6] == 0;
            cpu.set_t(t2);
            if t2 {
                break;
            }
        }
    }
    cpu.r[6] = mem.read_u32(0x8c23bc04).unwrap_or(0);
    cpu.r[6] = mem.read_u32(cpu.r[6] + 8).unwrap_or(0);
    let t3 = cpu.r[6] == 0;
    cpu.set_t(t3);
    if !t3 {
        loop {
            cpu.r[2] = mem.read_u32(cpu.r[6] + 16).unwrap_or(0);
            let hi = cpu.r[2] > cpu.r[0];
            cpu.set_t(hi);
            if hi {
                cpu.r[0] = mem.read_u32(cpu.r[6] + 16).unwrap_or(0);
            }
            cpu.r[6] = mem.read_u32(cpu.r[6] + 8).unwrap_or(0);
            let t4 = cpu.r[6] == 0;
            cpu.set_t(t4);
            if t4 {
                break;
            }
        }
    }
    let _ = mem.write_u32(cpu.r[4], cpu.r[7]);
    let _ = mem.write_u32(cpu.r[5], cpu.r[0]);
    cpu.r[0] = 0; // delay slot of `rts`
}
