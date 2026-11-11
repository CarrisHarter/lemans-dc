//! 0x8c19ae00 — find_entry_by_key.
//!
//! Scans `[r4+8]` records of 12 bytes starting at `[r4+12]` for the one whose
//! first word equals `r5`; returns its address, or 0.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[1] = mem.read_u32(cpu.r[4] + 8).unwrap_or(0);
    cpu.r[0] = mem.read_u32(cpu.r[4] + 12).unwrap_or(0);
    cpu.r[3] = cpu.r[1];
    cpu.r[3] = cpu.r[3] << 1;
    cpu.r[3] = cpu.r[3].wrapping_add(cpu.r[1]);
    cpu.r[3] = cpu.r[3] << 2;
    cpu.r[3] = cpu.r[3].wrapping_add(cpu.r[0]); // delay slot of `bra`
    let mut r1 = cpu.r[1];
    let mut found = false;
    loop {
        let t = cpu.r[3] > cpu.r[0];
        cpu.set_t(t);
        if !t {
            break;
        }
        r1 = mem.read_u32(cpu.r[0]).unwrap_or(0);
        let eq = r1 == cpu.r[5];
        cpu.set_t(eq);
        if eq {
            found = true;
            break;
        }
        cpu.r[0] = cpu.r[0].wrapping_add(12);
    }
    cpu.r[1] = r1;
    if !found {
        cpu.r[0] = 0; // delay slot of `rts`
    }
}
