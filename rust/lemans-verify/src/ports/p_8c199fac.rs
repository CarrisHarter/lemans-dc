//! 0x8c199fac - round `[r4+28]` up to the next even-then-unit boundary.
//!
//! `n = [r4+28]`; when n is zero the function returns immediately with r0 and r1
//! untouched. Otherwise r0 = 2, r1 = 0xfffffffe and r4 = n - 2; a first loop adds
//! 2 to r3 while `n-2 > r3`, then a second adds 1 while `n > r3`. T ends clear.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[5] = mem.read_u32(cpu.r[4].wrapping_add(28)).unwrap_or(0); // mov.l @(28,r4),r5
    cpu.r[3] = 0; // mov #0,r3
    cpu.set_t(cpu.r[5] > cpu.r[3]); // cmp/hi r3,r5
    if cpu.t() {
        cpu.r[1] = mem.read_u32(0x8c199fdc).unwrap_or(0xffff_fffe); // mov.l 0x8c199fdc,r1
        cpu.r[0] = 2; // mov #2,r0
        cpu.r[4] = cpu.r[5]; // mov r5,r4
        cpu.set_t(cpu.r[5] > cpu.r[0]); // cmp/hi r0,r5
        cpu.r[4] = cpu.r[4].wrapping_add(cpu.r[1]); // bf.s delay: add r1,r4
        if cpu.t() {
            loop {
                cpu.r[3] = cpu.r[3].wrapping_add(2); // add #2,r3
                cpu.set_t(cpu.r[4] > cpu.r[3]); // cmp/hi r3,r4
                if !cpu.t() {
                    break;
                }
            }
        }
        loop {
            cpu.set_t(cpu.r[5] > cpu.r[3]); // cmp/hi r3,r5
            if !cpu.t() {
                break;
            }
            cpu.r[3] = cpu.r[3].wrapping_add(1); // add #1,r3
        }
    }
}
