//! 0x8c23991c — clear the busy bit and drop the pending count for a slot.
//!
//! Calls 0x8c239860 with the argument saved on the frame; a zero result returns
//! 0x0000eeee. Otherwise `[r4]` (the saved pointer) supplies a bit index:
//! `mask = 1 << (index & 63)`; `[0x8c2c9e88+0x248] &= ~mask`;
//! `[0x8c2c9e88+0x24c] -= 1`; r0 = 0. r4 ends holding the table base.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[15] = cpu.r[15].wrapping_sub(4); // sts.l pr,@-r15 ; add #-4,r15
    let arg = cpu.r[4];
    let _ = mem.write_u32(cpu.r[15], arg); // bsr delay: mov.l r4,@r15
    cpu.pr = 0x8c239924;
    crate::registry::p_8c239860::run(cpu, mem); // bsr 0x8c239860
    cpu.set_t(cpu.r[0] == 0); // tst r0,r0
    if cpu.r[0] == 0 {
        cpu.r[0] = mem.read_u32(0x8c239968).unwrap_or(0x0000eeee); // mov.l 0x8c239968,r0
        cpu.r[15] = cpu.r[15].wrapping_add(4); // add #4,r15 ; lds.l @r15+,pr
        return;
    }
    // 0x8c239932
    cpu.r[5] = mem.read_u32(cpu.r[15]).unwrap_or(0); // mov.l @r15,r5
    cpu.r[2] = 1; // mov #1,r2
    cpu.r[4] = mem.read_u32(0x8c23996c).unwrap_or(0x8c2c9e88); // mov.l 0x8c23996c,r4
    cpu.r[3] = mem.read_u32(cpu.r[5]).unwrap_or(0); // mov.l @r5,r3
    cpu.r[0] = mem.read_u16(0x8c239960).unwrap_or(0x0248) as u16 as i16 as i32 as u32; // mov.w
    let shift = (cpu.r[3] & 63) as u32; // shad r3,r2
    cpu.r[2] = if cpu.r[3] & 0x8000_0000 != 0 {
        ((cpu.r[2] as i32) >> shift) as u32
    } else {
        cpu.r[2] << shift
    };
    cpu.r[3] = mem.read_u32(cpu.r[4].wrapping_add(cpu.r[0])).unwrap_or(0); // mov.l @(r0,r4),r3
    cpu.r[2] = !cpu.r[2]; // not r2,r2
    cpu.r[3] &= cpu.r[2]; // and r2,r3
    let _ = mem.write_u32(cpu.r[4].wrapping_add(cpu.r[0]), cpu.r[3]); // mov.l r3,@(r0,r4)
    cpu.r[0] = cpu.r[0].wrapping_add(4); // add #4,r0
    cpu.r[2] = mem.read_u32(cpu.r[4].wrapping_add(cpu.r[0])).unwrap_or(0); // mov.l @(r0,r4),r2
    cpu.r[2] = cpu.r[2].wrapping_sub(1); // add #-1,r2
    let _ = mem.write_u32(cpu.r[4].wrapping_add(cpu.r[0]), cpu.r[2]); // mov.l r2,@(r0,r4)
    cpu.r[0] = 0; // mov #0,r0
    cpu.r[15] = cpu.r[15].wrapping_add(4); // add #4,r15 ; lds.l @r15+,pr
}
