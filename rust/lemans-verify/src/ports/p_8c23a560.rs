//! 0x8c23a560 — mark a found object: clear bits then OR in 0x00020000 (and 0x00040000).
//!
//! `0x8c237780(r4)` searches a table; a miss returns 0x0000eeee. On a hit the
//! object is `[r4+0x88]`, whose flags word at `+36` is `&= 0xfff9ffff`, then
//! `|= 0x00020000`, and `|= 0x00040000` when the second argument is non-zero.
//! Both arguments are read back from the frame, so r4 ends as the object.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[15] = cpu.r[15].wrapping_sub(8); // sts.l pr,@-r15 ; add #-8,r15
    let arg0 = cpu.r[4];
    let arg1 = cpu.r[5];
    let _ = mem.write_u32(cpu.r[15], arg0); // mov.l r4,@r15
    let _ = mem.write_u32(cpu.r[15].wrapping_add(4), arg1); // mov.l r5,@(4,r15)
    cpu.r[3] = mem.read_u32(0x8c23a5c4).unwrap_or(0x8c237780); // mov.l 0x8c23a5c4,r3
    cpu.r[4] = mem.read_u32(cpu.r[15]).unwrap_or(0); // jsr delay: mov.l @r15,r4
    cpu.pr = 0x8c23a56e;
    crate::registry::p_8c237780::run(cpu, mem); // jsr @r3
    cpu.set_t(cpu.r[0] == 0); // tst r0,r0
    if cpu.r[0] == 0 {
        cpu.r[0] = mem.read_u32(0x8c23a5c8).unwrap_or(0x0000eeee); // mov.l 0x8c23a5c8,r0
        cpu.r[15] = cpu.r[15].wrapping_add(8); // add #8,r15 ; lds.l @r15+,pr
        return;
    }
    // 0x8c23a57c
    cpu.r[0] = mem.read_u16(0x8c23a5c0).unwrap_or(0x0088) as u16 as i16 as i32 as u32; // mov.w
    cpu.r[4] = mem.read_u32(cpu.r[15]).unwrap_or(0); // mov.l @r15,r4
    cpu.r[3] = mem.read_u32(0x8c23a5cc).unwrap_or(0xfff9ffff); // mov.l 0x8c23a5cc,r3
    cpu.r[4] = mem.read_u32(cpu.r[4].wrapping_add(cpu.r[0])).unwrap_or(0); // mov.l @(r0,r4),r4
    cpu.r[2] = mem.read_u32(cpu.r[4].wrapping_add(36)).unwrap_or(0); // mov.l @(36,r4),r2
    cpu.r[2] &= cpu.r[3]; // and r3,r2
    let _ = mem.write_u32(cpu.r[4].wrapping_add(36), cpu.r[2]); // mov.l r2,@(36,r4)
    cpu.r[1] = mem.read_u32(cpu.r[4].wrapping_add(36)).unwrap_or(0); // mov.l @(36,r4),r1
    cpu.r[2] = mem.read_u32(0x8c23a5d0).unwrap_or(0x00020000); // mov.l 0x8c23a5d0,r2
    cpu.r[1] |= cpu.r[2]; // or r2,r1
    let _ = mem.write_u32(cpu.r[4].wrapping_add(36), cpu.r[1]); // mov.l r1,@(36,r4)
    // 0x8c23a592
    cpu.r[0] = mem.read_u32(cpu.r[15].wrapping_add(4)).unwrap_or(0); // mov.l @(4,r15),r0
    cpu.set_t(cpu.r[0] == 0); // tst r0,r0
    if cpu.r[0] != 0 {
        cpu.r[0] = mem.read_u32(cpu.r[4].wrapping_add(36)).unwrap_or(0); // mov.l @(36,r4),r0
        cpu.r[1] = mem.read_u32(0x8c23a5d4).unwrap_or(0x00040000); // mov.l 0x8c23a5d4,r1
        cpu.r[0] |= cpu.r[1]; // or r1,r0
        let _ = mem.write_u32(cpu.r[4].wrapping_add(36), cpu.r[0]); // mov.l r0,@(36,r4)
    }
    cpu.r[0] = 0; // mov #0,r0
    cpu.r[15] = cpu.r[15].wrapping_add(8); // add #8,r15 ; lds.l @r15+,pr
}
