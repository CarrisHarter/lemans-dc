//! 0x8c14b698 — advance the cursor at [r4+8] by 0x570-multiples past [r4+4].
//!
//! `macl = (0x570 * [r4+4])` (signed `mul.l`, low half), `r5 = [r4+8] + macl`,
//! then r3 = [r4+8] grows by 0x570 while `r5 > r3` (unsigned `cmp/hi`). The
//! `rts` delay slot stores 0 at [r4+4]; r0 = 0 and T = 0 on both exits.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[1] = mem.read_u32(cpu.r[4].wrapping_add(4)).unwrap_or(0); // mov.l @(4,r4),r1
    cpu.r[0] = mem.read_u32(0x8c14b6b8).unwrap_or(0x570); // mov.l 0x8c14b6b8,r0
    cpu.r[3] = mem.read_u32(cpu.r[4].wrapping_add(8)).unwrap_or(0); // mov.l @(8,r4),r3
    let product = (cpu.r[0] as i32 as i64) * (cpu.r[1] as i32 as i64); // mul.l r0,r1
    cpu.mach = (product >> 32) as u32;
    cpu.macl = product as u32;
    cpu.r[1] = cpu.macl; // sts macl,r1
    cpu.r[5] = cpu.r[3]; // mov r3,r5
    cpu.r[5] = cpu.r[5].wrapping_add(cpu.r[1]); // add r1,r5
    cpu.set_t(cpu.r[5] > cpu.r[3]); // cmp/hi r3,r5
    if cpu.r[5] > cpu.r[3] {
        loop {
            cpu.r[3] = cpu.r[3].wrapping_add(cpu.r[0]); // add r0,r3
            cpu.set_t(cpu.r[5] > cpu.r[3]); // cmp/hi r3,r5
            if cpu.r[5] > cpu.r[3] {
                continue;
            }
            break;
        }
    }
    cpu.r[0] = 0; // mov #0,r0
    let _ = mem.write_u32(cpu.r[4].wrapping_add(4), cpu.r[0]); // delay: mov.l r0,@(4,r4)
}
