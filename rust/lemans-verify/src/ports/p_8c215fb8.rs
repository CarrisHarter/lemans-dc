//! 0x8c215fb8 - 1 if the next `r6` bytes at r4 and r5 match, else 0.
//!
//! The compare body is at 0x8c215fac, before this entry. The `bt.s` delay
//! decrements `r6` before each compare, so a non-positive count returns 1
//! without reading.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    loop {
        cpu.set_t((cpu.r[6] as i32) > 0); // cmp/pl r6
        cpu.r[6] = cpu.r[6].wrapping_add((-1i32) as u32); // bt.s delay
        if !cpu.t() {
            cpu.r[0] = 1;
            return;
        }
        cpu.r[2] = mem.read_u8(cpu.r[5]).unwrap_or(0) as i8 as i32 as u32;
        cpu.r[5] = cpu.r[5].wrapping_add(1);
        cpu.r[3] = mem.read_u8(cpu.r[4]).unwrap_or(0) as i8 as i32 as u32;
        cpu.r[4] = cpu.r[4].wrapping_add(1);
        cpu.set_t(cpu.r[2] == cpu.r[3]); // cmp/eq r2,r3
        if !cpu.t() {
            cpu.r[0] = 0; // rts delay
            return;
        }
    }
}
