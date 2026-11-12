//! 0x8c1aeb9e - byte-wise compare; 0 if both end together, else the difference.
//!
//! The equal-byte path branches back to 0x8c1aeb94, which is before this entry.
//! Both pointers advance in the `bt.s` delay slot before the zero test.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    loop {
        cpu.r[3] = mem.read_u8(cpu.r[4]).unwrap_or(0) as i8 as i32 as u32;
        cpu.r[1] = mem.read_u8(cpu.r[5]).unwrap_or(0) as i8 as i32 as u32;
        cpu.r[3] &= 0xff;
        cpu.r[0] = cpu.r[3] & 0xff;
        cpu.r[1] &= 0xff;
        cpu.r[1] &= 0xff;
        cpu.r[4] = cpu.r[4].wrapping_add(1);
        cpu.set_t(cpu.r[0] == cpu.r[1]); // cmp/eq r1,r0
        cpu.r[5] = cpu.r[5].wrapping_add(1); // bt.s delay
        if !cpu.t() {
            cpu.r[0] = cpu.r[0].wrapping_sub(cpu.r[1]); // rts delay: sub r1,r0
            return;
        }
        cpu.r[0] = cpu.r[3] & 0xff; // extu.b r3,r0
        cpu.set_t(cpu.r[0] == 0); // tst r0,r0
        if cpu.t() {
            cpu.r[0] = 0; // rts delay
            return;
        }
    }
}
