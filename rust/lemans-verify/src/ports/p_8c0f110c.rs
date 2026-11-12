//! 0x8c0f110c — free the array at `[r4+8]`, aligning a cursor up to `[r4+4]`.
//!
//! `r1 = [r4+8] + ([r4+4] << 2)` is the array end; `r3` walks from the base in
//! 4-byte steps until it is not below that end (the `bt` loop). Both `cmp/hi`
//! forms compute `T = r1 > r3`, so the exit flag is 0 on every path that frees.
//! A null `[r4+8]` exits with T=1 and no call.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let base = cpu.r[4];
    cpu.r[5] = mem.read_u32(base.wrapping_add(8)).unwrap_or(0); // mov.l @(8,r4),r5
    cpu.set_t(cpu.r[5] == 0); // tst r5,r5
    if cpu.r[5] == 0 {
        return; // bt 0x8c0f112e
    }
    let count = mem.read_u32(base.wrapping_add(4)).unwrap_or(0); // mov.l @(4,r4),r0
    cpu.r[1] = cpu.r[5]; // mov r5,r1
    cpu.r[0] = count.wrapping_shl(2); // shll2 r0
    cpu.r[1] = cpu.r[1].wrapping_add(cpu.r[0]); // add r0,r1
    cpu.set_t(cpu.r[1] > cpu.r[5]); // cmp/hi r5,r1
    cpu.r[3] = cpu.r[5]; // bf.s delay slot
    if !cpu.t() {
        free(cpu, mem);
        return;
    }
    loop {
        cpu.r[3] = cpu.r[3].wrapping_add(4); // add #4,r3
        cpu.set_t(cpu.r[1] > cpu.r[3]); // cmp/hi r3,r1
        if !cpu.t() {
            break; // bt 0x8c0f1122
        }
    }
    free(cpu, mem);
}

fn free(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[0] = mem.read_u32(0x8c0f1134).unwrap_or(0x8c01032c); // mov.l 0x8c0f1134,r0
    cpu.r[4] = cpu.r[5]; // jsr delay slot
    cpu.pr = 0x8c0f112e;
    crate::registry::p_8c01032c::run(cpu, mem);
}
