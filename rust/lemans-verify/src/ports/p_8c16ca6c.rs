//! 0x8c16ca6c — increment_until_nonpositive.
//!
//! Increments the counter at `0x8c2b5230` while it stays positive and at most 3.
//! The store is the `bf.s` delay slot, so it runs even on the exit iteration.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let r3 = mem.read_u32(0x8c16ca84).unwrap_or(0);
    let r1: u32 = 3;
    let mut r0: u32;
    loop {
        r0 = mem.read_u32(r3).unwrap_or(0);
        r0 = r0.wrapping_add(1);
        let pz = (r0 as i32) >= 0;
        let _ = mem.write_u32(r3, r0); // delay slot of `bf.s`
        cpu.set_t(pz);
        if !pz {
            break;
        }
        let gt = r1 > r0;
        cpu.set_t(gt);
        if !gt {
            break;
        }
    }
    cpu.r[3] = r3;
    cpu.r[1] = r1;
    cpu.r[0] = r0;
}
