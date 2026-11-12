//! 0x8c070904 - construct the object at r4: sub-init, vtable stamp, tail init.
//!
//! Straight fan-out with the argument kept in an 8-byte frame (pr slot + arg
//! slot) and reloaded for every step: 0x8c152e64(arg), `[arg] = 0x8c1e9538`,
//! 0x8c0b3950(arg+0xd8). Returns arg in r0. The 0xd8 pool word at 0x8c070934
//! is a small addend, not an address.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr); // sts.l pr,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4); // add #-4,r15
    let _ = mem.write_u32(cpu.r[15], cpu.r[4]); // mov.l r4,@r15

    cpu.r[0] = mem.read_u32(0x8c07092c).unwrap_or(0x8c152e64);
    cpu.r[4] = mem.read_u32(cpu.r[15]).unwrap_or(0); // jsr delay: mov.l @r15,r4
    cpu.pr = 0x8c070910;
    crate::registry::p_8c152e64::run(cpu, mem);

    cpu.r[3] = mem.read_u32(cpu.r[15]).unwrap_or(0); // mov.l @r15,r3
    cpu.r[0] = mem.read_u32(0x8c070930).unwrap_or(0x8c1e9538);
    cpu.r[1] = mem.read_u32(0x8c070934).unwrap_or(0xd8);
    let _ = mem.write_u32(cpu.r[3], cpu.r[0]); // mov.l r0,@r3
    cpu.r[4] = mem.read_u32(cpu.r[15]).unwrap_or(0); // mov.l @r15,r4
    cpu.r[0] = mem.read_u32(0x8c070938).unwrap_or(0x8c0b3950);
    cpu.r[4] = cpu.r[4].wrapping_add(cpu.r[1]); // jsr delay: add r1,r4
    cpu.pr = 0x8c070920;
    crate::registry::p_8c0b3950::run(cpu, mem);

    cpu.r[0] = mem.read_u32(cpu.r[15]).unwrap_or(0); // mov.l @r15,r0
    cpu.r[15] = cpu.r[15].wrapping_add(4); // add #4,r15
    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0); // lds.l @r15+,pr
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    // rts; delay nop
}
