//! 0x8c237240 — size class.
//!
//! Derives a size class from bits of `r4` and the null-ness of `r7`, writing the
//! class to `[r5]` and a granularity to `[r6]` when those pointers are non-null.
//! The `r4 & 32` test only runs on the `r7 == 0` path; the other path sets the
//! granularity to 15 directly.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let t = cpu.r[7] == 0;
    cpu.set_t(t);
    cpu.r[0] = 16;
    cpu.r[0] = cpu.r[0] & cpu.r[4]; // delay slot of `bt.s`
    if t {
        let t2 = cpu.r[0] == 0;
        cpu.set_t(t2);
        if t2 {
            cpu.r[7] = 10;
        } else {
            cpu.r[7] = 20;
        }
        cpu.r[2] = 32;
        let t3 = (cpu.r[2] & cpu.r[4]) == 0;
        cpu.set_t(t3);
        if t3 {
            cpu.r[4] = 8;
        } else {
            cpu.r[4] = 15;
        }
    } else {
        let t2 = cpu.r[0] == 0;
        cpu.set_t(t2);
        cpu.r[7] = if t2 { 20 } else { 40 };
        cpu.r[4] = 15;
    }
    let t4 = cpu.r[5] == 0;
    cpu.set_t(t4);
    if !t4 {
        let _ = mem.write_u32(cpu.r[5], cpu.r[7]);
    }
    let t5 = cpu.r[6] == 0;
    cpu.set_t(t5);
    cpu.r[0] = 0; // delay slot of `bt.s`
    if !t5 {
        let _ = mem.write_u32(cpu.r[6], cpu.r[4]);
    }
}
