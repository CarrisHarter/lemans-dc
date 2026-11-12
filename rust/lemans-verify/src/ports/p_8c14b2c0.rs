//! 0x8c14b2c0 - free the array at `[r4+8]` after walking `[r4+4]` 4-byte slots.
//!
//! `r1 = base + (count << 2)`; the cursor steps by 4 until it is not below r1.
//! `shll2` leaves T alone, so the exit flag is the `cmp/hi` result (0). A null
//! base exits with T=1 and no call.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr);
    cpu.r[5] = mem.read_u32(cpu.r[4].wrapping_add(8)).unwrap_or(0); // mov.l @(8,r4),r5
    cpu.set_t(cpu.r[5] == 0); // tst r5,r5
    if cpu.r[5] != 0 {
        cpu.r[0] = mem.read_u32(cpu.r[4].wrapping_add(4)).unwrap_or(0); // mov.l @(4,r4),r0
        cpu.r[1] = cpu.r[5]; // mov r5,r1
        cpu.r[0] = cpu.r[0].wrapping_shl(2); // shll2 r0 (T untouched)
        cpu.r[1] = cpu.r[1].wrapping_add(cpu.r[0]); // add r0,r1
        cpu.set_t(cpu.r[1] > cpu.r[5]); // cmp/hi r5,r1
        cpu.r[3] = cpu.r[5]; // bf.s delay slot
        if !cpu.t() {
            free(cpu, mem);
        } else {
            loop {
                cpu.r[3] = cpu.r[3].wrapping_add(4); // add #4,r3
                cpu.set_t(cpu.r[1] > cpu.r[3]); // cmp/hi r3,r1
                if !cpu.t() {
                    break;
                }
            }
            free(cpu, mem);
        }
    }
    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0); // lds.l @r15+,pr
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}

fn free(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[0] = mem.read_u32(0x8c14b2e8).unwrap_or(0x8c01032c); // mov.l 0x8c14b2e8,r0
    cpu.r[4] = cpu.r[5]; // jsr delay: mov r5,r4
    cpu.pr = 0x8c14b2e2;
    crate::registry::p_8c01032c::run(cpu, mem);
}
