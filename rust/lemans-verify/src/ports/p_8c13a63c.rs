//! 0x8c13a63c - call 0x8c0cc51c for each 20-byte record whose byte at +12 is 0.
//!
//! `count = [r12+60]`, `base = [r12+56]`; the loop walks `base + i*20` while
//! `count > i` (signed `cmp/gt`). The `bra` delay slot seeds `r13 = r14 = 0`.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let mut saved = [0u32; 4];
    for (i, reg) in [14usize, 13, 12].iter().enumerate() {
        saved[i] = cpu.r[*reg];
        cpu.r[15] = cpu.r[15].wrapping_sub(4);
        let _ = mem.write_u32(cpu.r[15], cpu.r[*reg]);
    }
    saved[3] = cpu.pr;
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr);

    cpu.r[12] = cpu.r[4]; // mov r4,r12
    cpu.r[14] = 0; // mov #0,r14
    cpu.r[13] = cpu.r[14]; // bra delay: mov r14,r13
    loop {
        cpu.r[0] = mem.read_u32(cpu.r[12].wrapping_add(60)).unwrap_or(0); // mov.l @(60,r12),r0
        cpu.set_t((cpu.r[0] as i32) > (cpu.r[14] as i32)); // cmp/gt r14,r0
        if !cpu.t() {
            break;
        }
        cpu.r[4] = mem.read_u32(cpu.r[12].wrapping_add(56)).unwrap_or(0); // mov.l @(56,r12),r4
        cpu.r[4] = cpu.r[4].wrapping_add(cpu.r[13]); // add r13,r4
        cpu.r[0] = mem.read_u8(cpu.r[4].wrapping_add(12)).unwrap_or(0) as u32; // mov.b @(12,r4),r0
        cpu.r[0] &= 0xff; // extu.b r0,r0
        cpu.set_t(cpu.r[0] == 0); // tst r0,r0
        if cpu.r[0] == 0 {
            cpu.r[0] = mem.read_u32(0x8c13a674).unwrap_or(0x8c0cc51c); // mov.l 0x8c13a674,r0
            cpu.pr = 0x8c13a65e;
            crate::registry::p_8c0cc51c::run(cpu, mem);
        }
        cpu.r[13] = cpu.r[13].wrapping_add(20); // add #20,r13
        cpu.r[14] = cpu.r[14].wrapping_add(1); // add #1,r14
    }
    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    for (i, reg) in [12usize, 13].iter().enumerate() {
        cpu.r[*reg] = mem.read_u32(cpu.r[15]).unwrap_or(saved[i + 1]);
        cpu.r[15] = cpu.r[15].wrapping_add(4);
    }
    cpu.r[14] = mem.read_u32(cpu.r[15]).unwrap_or(0); // rts delay
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}
