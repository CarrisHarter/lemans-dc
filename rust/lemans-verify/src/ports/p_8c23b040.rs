//! 0x8c23b040 — release a slot: clear its bit, drop the count, free the payload.
//!
//! First 0x8c23ae00(r5) gates the call (a zero result returns 0x0000eeee); then
//! 0x8c237780(r5) must hit a table entry. `bit = 1 << ([r5] & 63)`; when it is
//! set in `[0x8c2c9e88+4]` the bit is cleared there and `[+8]` is decremented,
//! after which 0x8c23991c runs on `[r5+0x88]`. Finally `[r5+28]` and `[r5+32]`
//! are zeroed and r0 = 0. r13/r14/pr/r15 are restored on every exit.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let saved13 = cpu.r[13]; // mov.l r14,@-r15 ; mov.l r13,@-r15 ; sts.l pr,@-r15 ; add #-8,r15
    let saved14 = cpu.r[14];
    cpu.r[15] = cpu.r[15].wrapping_sub(8);
    cpu.r[13] = mem.read_u32(0x8c23b0a4).unwrap_or(0x8c2c9e88); // mov.l 0x8c23b0a4,r13
    let arg4 = cpu.r[4];
    let arg5 = cpu.r[5];
    cpu.r[14] = cpu.r[5]; // mov r5,r14
    let _ = mem.write_u32(cpu.r[15].wrapping_add(4), arg4); // mov.l r4,@(4,r15)
    let _ = mem.write_u32(cpu.r[15], arg5); // mov.l r5,@r15
    cpu.r[4] = mem.read_u32(cpu.r[15].wrapping_add(4)).unwrap_or(0); // bsr delay: mov.l @(4,r15),r4
    cpu.pr = 0x8c23b054;
    crate::registry::p_8c23ae00::run(cpu, mem); // bsr 0x8c23ae00
    cpu.set_t(cpu.r[0] == 0); // tst r0,r0
    if cpu.r[0] == 0 {
        return sentinel(cpu, saved13, saved14); // bt 0x8c23b062
    }
    cpu.r[3] = mem.read_u32(0x8c23b0a8).unwrap_or(0x8c237780); // mov.l 0x8c23b0a8,r3
    cpu.r[4] = mem.read_u32(cpu.r[15]).unwrap_or(0); // jsr delay: mov.l @r15,r4
    cpu.pr = 0x8c23b05e;
    crate::registry::p_8c237780::run(cpu, mem); // jsr @r3
    cpu.set_t(cpu.r[0] == 0); // tst r0,r0
    if cpu.r[0] == 0 {
        return sentinel(cpu, saved13, saved14); // bf 0x8c23b068 (inverted: zero -> sentinel)
    }
    // 0x8c23b068
    cpu.r[5] = mem.read_u32(cpu.r[14]).unwrap_or(0); // mov.l @r14,r5
    cpu.r[4] = 1; // mov #1,r4
    cpu.r[3] = mem.read_u32(cpu.r[13].wrapping_add(4)).unwrap_or(0); // mov.l @(4,r13),r3
    let sh = (cpu.r[5] & 63) as u32; // shad r5,r4
    cpu.r[4] = if cpu.r[5] & 0x8000_0000 != 0 {
        ((cpu.r[4] as i32) >> sh) as u32
    } else {
        cpu.r[4] << sh
    };
    cpu.set_t((cpu.r[4] & cpu.r[3]) == 0); // tst r4,r3
    if (cpu.r[4] & cpu.r[3]) != 0 {
        cpu.r[3] = mem.read_u32(cpu.r[13].wrapping_add(4)).unwrap_or(0); // mov.l @(4,r13),r3
        cpu.r[4] = !cpu.r[4]; // not r4,r4
        cpu.r[3] &= cpu.r[4]; // and r4,r3
        let _ = mem.write_u32(cpu.r[13].wrapping_add(4), cpu.r[3]); // mov.l r3,@(4,r13)
        cpu.r[2] = mem.read_u32(cpu.r[13].wrapping_add(8)).unwrap_or(0); // mov.l @(8,r13),r2
        cpu.r[2] = cpu.r[2].wrapping_sub(1); // add #-1,r2
        let _ = mem.write_u32(cpu.r[13].wrapping_add(8), cpu.r[2]); // mov.l r2,@(8,r13)
    }
    // 0x8c23b082
    cpu.r[3] = mem.read_u32(0x8c23b0b0).unwrap_or(0x8c23991c); // mov.l 0x8c23b0b0,r3
    cpu.r[0] = mem.read_u16(0x8c23b0a0).unwrap_or(0x0088) as u16 as i16 as i32 as u32; // mov.w
    cpu.r[4] = mem.read_u32(cpu.r[14].wrapping_add(cpu.r[0])).unwrap_or(0); // jsr delay: mov.l @(r0,r14),r4
    cpu.pr = 0x8c23b08a;
    crate::registry::p_8c23991c::run(cpu, mem); // jsr @r3
    cpu.r[4] = 0; // mov #0,r4
    cpu.r[0] = cpu.r[4]; // mov r4,r0
    let _ = mem.write_u32(cpu.r[14].wrapping_add(28), cpu.r[4]); // mov.l r4,@(28,r14)
    let _ = mem.write_u32(cpu.r[14].wrapping_add(32), cpu.r[4]); // mov.l r4,@(32,r14)
    cpu.r[15] = cpu.r[15].wrapping_add(8); // add #8,r15 ; lds.l @r15+,pr ; pops
    cpu.r[13] = saved13;
    cpu.r[14] = saved14;
}

fn sentinel(cpu: &mut Cpu, saved13: u32, saved14: u32) {
    cpu.r[0] = 0x0000eeee; // mov.l 0x8c23b0ac,r0 ; bra 0x8c23b092
    cpu.r[15] = cpu.r[15].wrapping_add(8); // add #8,r15 ; lds.l @r15+,pr ; pops
    cpu.r[13] = saved13;
    cpu.r[14] = saved14;
}
