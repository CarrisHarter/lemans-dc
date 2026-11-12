//! 0x8c14b664 - free the array at `[r4+8]` after walking 0x570-byte records.
//!
//! `r3 = base + low32(0x570 * [r4+4])` (signed `mul.l`, MACH/MACL written);
//! r4 starts at the base and grows by 0x570 while `r3 > r4`. The `bf.s` delay
//! slot sets r4 = base on both paths, and the free call re-sets it.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr);
    cpu.r[5] = mem.read_u32(cpu.r[4].wrapping_add(8)).unwrap_or(0); // mov.l @(8,r4),r5
    cpu.set_t(cpu.r[5] == 0); // tst r5,r5
    if cpu.r[5] != 0 {
        cpu.r[1] = mem.read_u32(cpu.r[4].wrapping_add(4)).unwrap_or(0); // mov.l @(4,r4),r1
        cpu.r[3] = cpu.r[5]; // mov r5,r3
        cpu.r[0] = mem.read_u32(0x8c14b690).unwrap_or(0x570); // mov.l 0x8c14b690,r0
        let product = (cpu.r[0] as i32 as i64) * (cpu.r[1] as i32 as i64); // mul.l r0,r1
        cpu.mach = (product >> 32) as u32;
        cpu.macl = product as u32;
        cpu.r[1] = cpu.macl; // sts macl,r1
        cpu.r[3] = cpu.r[3].wrapping_add(cpu.r[1]); // add r1,r3
        cpu.set_t(cpu.r[3] > cpu.r[5]); // cmp/hi r5,r3
        cpu.r[4] = cpu.r[5]; // bf.s delay slot: mov r5,r4
        if cpu.t() {
            loop {
                cpu.r[4] = cpu.r[4].wrapping_add(cpu.r[0]); // add r0,r4
                cpu.set_t(cpu.r[3] > cpu.r[4]); // cmp/hi r4,r3
                if !cpu.t() {
                    break;
                }
            }
        }
        cpu.r[0] = mem.read_u32(0x8c14b694).unwrap_or(0x8c01032c); // mov.l 0x8c14b694,r0
        cpu.r[4] = cpu.r[5]; // jsr delay: mov r5,r4
        cpu.pr = 0x8c14b68a;
        crate::registry::p_8c01032c::run(cpu, mem);
    }
    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0); // lds.l @r15+,pr
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}
