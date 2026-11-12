//! 0x8c219ed6 - look up a 0x100-byte record and split its first two bytes out.
//!
//! 0x8c219c9e(r5, 0x100) returns a record pointer. A null result returns -1;
//! otherwise `[r4] = record[0]` and `[r4+4] = record[1]` (both zero-extended) and
//! the function returns 0.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr);
    cpu.r[15] = cpu.r[15].wrapping_sub(8); // add #-8,r15
    let _ = mem.write_u32(cpu.r[15], cpu.r[4]); // mov.l r4,@r15
    let _ = mem.write_u32(cpu.r[15].wrapping_add(4), cpu.r[5]); // mov.l r5,@(4,r15)
    cpu.r[5] = mem.read_u16(0x8c219f0c).unwrap_or(0x100) as u32; // mov.w 0x8c219f0c,r5
    cpu.r[4] = mem.read_u32(cpu.r[15].wrapping_add(4)).unwrap_or(0); // bsr delay: mov.l @(4,r15),r4
    cpu.pr = 0x8c219ee4;
    crate::registry::p_8c219c9e::run(cpu, mem);
    cpu.r[4] = cpu.r[0]; // mov r0,r4
    cpu.set_t(cpu.r[4] == 0); // tst r4,r4
    let mut result;
    if !cpu.t() {
        cpu.r[2] = mem.read_u8(cpu.r[4]).unwrap_or(0) as u32; // mov.b @r4,r2
        cpu.r[3] = mem.read_u32(cpu.r[15]).unwrap_or(0); // mov.l @r15,r3
        cpu.r[2] &= 0xff; // extu.b r2,r2
        let _ = mem.write_u32(cpu.r[3], cpu.r[2]); // mov.l r2,@r3
        cpu.r[0] = mem.read_u8(cpu.r[4].wrapping_add(1)).unwrap_or(0) as u32; // mov.b @(1,r4),r0
        cpu.r[3] = mem.read_u32(cpu.r[15]).unwrap_or(0); // mov.l @r15,r3
        cpu.r[0] &= 0xff; // extu.b r0,r0
        let _ = mem.write_u32(cpu.r[3].wrapping_add(4), cpu.r[0]); // mov.l r0,@(4,r3)
        result = 0; // mov #0,r0
    } else {
        result = 0xffff_ffff; // mov #-1,r0
    }
    cpu.r[0] = result;
    cpu.r[15] = cpu.r[15].wrapping_add(8); // add #8,r15
    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}
