//! 0x8c216dc8 - 1 if the string at r4 equals the string at r5, else 0.
//!
//! A nonzero byte branches back to 0x8c216dc0, before this entry, which
//! compares one pair and advances both pointers.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    loop {
        cpu.r[3] = mem.read_u8(cpu.r[4]).unwrap_or(0) as i8 as i32 as u32;
        cpu.set_t(cpu.r[3] == 0); // tst r3,r3
        if cpu.t() {
            break;
        }
        cpu.r[2] = mem.read_u8(cpu.r[5]).unwrap_or(0) as i8 as i32 as u32; // mov.b @r5+,r2
        cpu.r[5] = cpu.r[5].wrapping_add(1);
        cpu.r[3] = mem.read_u8(cpu.r[4]).unwrap_or(0) as i8 as i32 as u32; // mov.b @r4+,r3
        cpu.r[4] = cpu.r[4].wrapping_add(1);
        cpu.set_t(cpu.r[2] == cpu.r[3]); // cmp/eq r2,r3
        if !cpu.t() {
            cpu.r[0] = 0; // rts delay at 0x8c216dd6
            return;
        }
    }
    cpu.r[1] = mem.read_u8(cpu.r[5]).unwrap_or(0) as i8 as i32 as u32;
    cpu.set_t(cpu.r[1] == 0); // tst r1,r1
    cpu.r[0] = u32::from(cpu.t());
}
