//! 0x8c0e2c48 — find the 24-byte table entry holding the published global.
//!
//! Loads `r4 = [0x8c240470]` through 0x8c070e74, then scans 16 entries at
//! 0x8c29b7a8 (stride 24) for the first match, keeping the index in r6
//! (initialised -1), and stores it at 0x8c27a938. The loop test is
//! `cmp/gt r5,r1` with r1 = 15, and the `bt.s` delay adds 24 to the offset even
//! on the final not-taken test.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[0] = mem.read_u32(0x8c0e2c78).unwrap_or(0x8c070e74); // mov.l 0x8c0e2c78,r0
    cpu.pr = 0x8c0e2c50;
    crate::registry::p_8c070e74::run(cpu, mem); // jsr @r0 ; nop

    let value = cpu.r[0]; // mov r0,r4
    let mut r5 = 0u32; // mov #0,r5
    let mut r7 = r5; // mov r5,r7
    let mut r6 = u32::MAX; // mov #-1,r6
    cpu.r[0] = mem.read_u32(0x8c0e2c7c).unwrap_or(0x8c29b7a8); // mov.l 0x8c0e2c7c,r0
    let table = cpu.r[0];
    cpu.r[1] = 15; // mov #15,r1
    cpu.r[4] = value;

    loop {
        cpu.r[3] = mem.read_u32(table.wrapping_add(r7)).unwrap_or(0); // mov.l @(r0,r7),r3
        cpu.set_t(cpu.r[3] == value); // cmp/eq r3,r4
        if cpu.r[3] == value {
            r6 = r5; // mov r5,r6
        }
        r5 = r5.wrapping_add(1); // add #1,r5
        cpu.set_t(cpu.r[1] > r5); // cmp/gt r5,r1
        r7 = r7.wrapping_add(24); // bt.s delay slot: add #24,r7
        cpu.r[7] = r7;
        cpu.r[5] = r5;
        cpu.r[6] = r6;
        if cpu.r[1] <= r5 {
            break;
        }
    }
    cpu.r[0] = mem.read_u32(0x8c0e2c80).unwrap_or(0x8c27a938); // mov.l 0x8c0e2c80,r0
    let _ = mem.write_u32(cpu.r[0], cpu.r[6]); // mov.l r6,@r0
}
