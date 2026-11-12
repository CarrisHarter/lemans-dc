//! 0x8c138494 — release every handle in `[r4+8 .. + ([r4+4]<<2)]`, then zero the count.
//!
//! `end = base + count*4`. When `end > base` each pointer is passed to
//! 0x8c018bd8 with `r5 = -1`, stepping 4 bytes. `[r4+4] = 0` on every path.
//! `bf` has no delay slot, so `r11 = -1` runs only when the loop is entered.
//! The exit T is the last `cmp/hi`, which is 0 (the cursor is not below `end`).

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let saved = [cpu.r[14], cpu.r[13], cpu.r[12], cpu.r[11], cpu.pr];
    cpu.r[15] = cpu.r[15].wrapping_sub(20);
    cpu.r[14] = cpu.r[4];
    cpu.r[13] = mem.read_u32(cpu.r[14].wrapping_add(8)).unwrap_or(0);
    let count = mem.read_u32(cpu.r[14].wrapping_add(4)).unwrap_or(0);
    cpu.r[0] = count.wrapping_shl(2); // shll2
    cpu.r[12] = cpu.r[13].wrapping_add(cpu.r[0]);
    cpu.set_t(cpu.r[12] > cpu.r[13]); // cmp/hi r13,r12
    if cpu.t() {
        cpu.r[11] = 0xffff_ffff; // mov #-1,r11
        loop {
            cpu.r[0] = mem.read_u32(0x8c1384d0).unwrap_or(0x8c018bd8);
            cpu.r[5] = cpu.r[11];
            cpu.r[4] = cpu.r[13];
            cpu.pr = 0x8c1384b8;
            crate::registry::p_8c018bd8::run(cpu, mem);
            cpu.r[13] = cpu.r[13].wrapping_add(4);
            cpu.set_t(cpu.r[12] > cpu.r[13]);
            if !cpu.t() {
                break;
            }
        }
    }
    cpu.r[0] = 0;
    let _ = mem.write_u32(cpu.r[14].wrapping_add(4), cpu.r[0]);
    cpu.r[15] = cpu.r[15].wrapping_add(20);
    cpu.pr = saved[4];
    cpu.r[11] = saved[3];
    cpu.r[12] = saved[2];
    cpu.r[13] = saved[1];
    cpu.r[14] = saved[0];
}
