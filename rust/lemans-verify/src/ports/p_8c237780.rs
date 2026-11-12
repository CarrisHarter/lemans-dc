//! 0x8c237780 — linear search of a fixed-stride table for a pointer.
//!
//! `base = [0x8c2377c4]`, `count = [base+12]`, `stride = (i16)0x8c2377c0` (0x8c,
//! 140). Entries start at `base+24`; each iteration computes
//! `base+24+idx*stride` with `mul.l` (macl = low 32 bits) and compares it to r4.
//! Returns 1 on a hit (r14), 0 otherwise; an empty table exits before the loop.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[5] = mem.read_u32(0x8c2377c4).unwrap_or(0x8c2c9e88); // mov.l 0x8c2377c4,r5
    let saved13 = cpu.r[13]; // mov.l r14,@-r15 ; mov.l r13,@-r15
    let saved14 = cpu.r[14];
    cpu.r[15] = cpu.r[15].wrapping_sub(8);
    cpu.r[6] = 0; // mov #0,r6
    cpu.r[2] = cpu.r[6]; // mov r6,r2
    cpu.r[14] = cpu.r[6]; // mov r6,r14
    cpu.r[3] = mem.read_u32(cpu.r[5].wrapping_add(12)).unwrap_or(0); // mov.l @(12,r5),r3
    cpu.r[13] = mem.read_u16(0x8c2377c0).unwrap_or(0x008c) as u16 as i16 as i32 as u32; // mov.w
    cpu.set_t(cpu.r[2] >= cpu.r[3]); // cmp/hs r3,r2
    cpu.r[7] = 1; // bt.s delay slot
    if !cpu.t() {
        loop {
            // 0x8c237796
            let product = cpu.r[6].wrapping_mul(cpu.r[13]); // mul.l r13,r6
            cpu.macl = product;
            cpu.r[3] = cpu.r[5]; // mov r5,r3
            cpu.r[3] = cpu.r[3].wrapping_add(24); // add #24,r3
            cpu.r[0] = cpu.macl; // sts macl,r0
            cpu.r[0] = cpu.r[0].wrapping_add(cpu.r[3]); // add r3,r0
            cpu.set_t(cpu.r[0] == cpu.r[4]); // cmp/eq r0,r4
            if cpu.r[0] == cpu.r[4] {
                cpu.r[14] = cpu.r[7]; // bra delay: mov r7,r14
                break;
            }
            cpu.r[2] = mem.read_u32(cpu.r[5].wrapping_add(12)).unwrap_or(0); // mov.l @(12,r5),r2
            cpu.r[6] = cpu.r[6].wrapping_add(1); // add #1,r6
            cpu.set_t(cpu.r[6] >= cpu.r[2]); // cmp/hs r2,r6
            if cpu.t() {
                break; // bf 0x8c237796
            }
        }
    }
    cpu.r[0] = cpu.r[14]; // mov r14,r0
    cpu.r[15] = cpu.r[15].wrapping_add(8); // mov.l @r15+,r13 ; rts delay: @r15+,r14
    cpu.r[13] = saved13;
    cpu.r[14] = saved14;
}
