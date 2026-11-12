//! 0x8c215fea - copy `r6` bytes from r4 to r5.
//!
//! The copy body is at 0x8c215fe4, before this entry. The `bt.s` delay slot
//! decrements `r6` before each copy, so a non-positive count copies nothing
//! and still decrements `r6`.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    loop {
        cpu.set_t((cpu.r[6] as i32) > 0); // cmp/pl r6
        cpu.r[6] = cpu.r[6].wrapping_add((-1i32) as u32); // bt.s delay
        if !cpu.t() {
            return;
        }
        cpu.r[3] = mem.read_u8(cpu.r[4]).unwrap_or(0) as i8 as i32 as u32; // mov.b @r4+,r3
        cpu.r[4] = cpu.r[4].wrapping_add(1);
        let _ = mem.write_u8(cpu.r[5], cpu.r[3] as u8); // mov.b r3,@r5
        cpu.r[5] = cpu.r[5].wrapping_add(1);
    }
}
