//! 0x8c152e64 - construct the object at r4: sub-init, vtable, five fan-out calls.
//!
//! Straight fan-out with the argument kept in an 8-byte frame (pr slot + arg
//! slot) and reloaded for every call: 0x8c151c1c(arg), `[arg] = 0x8c1ef4e0`,
//! 0x8c0b4ff0(arg+116), 0x8c0b2ee4(arg+0x80), 0x8c0b4ff0(arg+0xb0),
//! 0x8c0b4ff0(arg+0xbc), `[arg+0xc8] = +0.0`, 0x8c0b4ff0(arg+0xcc).
//! Returns arg in r0. Several pool words are small addends
//! (0x80/0xb0/0xbc/0xc8/0xcc), not addresses.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr); // sts.l pr,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4); // add #-4,r15
    let _ = mem.write_u32(cpu.r[15], cpu.r[4]); // mov.l r4,@r15

    cpu.r[0] = mem.read_u32(0x8c152ebc).unwrap_or(0x8c151c1c);
    cpu.r[4] = mem.read_u32(cpu.r[15]).unwrap_or(0); // jsr delay: mov.l @r15,r4
    cpu.pr = 0x8c152e70;
    crate::registry::p_8c151c1c::run(cpu, mem);

    cpu.r[3] = mem.read_u32(cpu.r[15]).unwrap_or(0); // mov.l @r15,r3
    cpu.r[1] = mem.read_u32(0x8c152ec0).unwrap_or(0x8c1ef4e0);
    cpu.r[0] = mem.read_u32(0x8c152ec4).unwrap_or(0x8c0b4ff0);
    let _ = mem.write_u32(cpu.r[3], cpu.r[1]); // mov.l r1,@r3
    cpu.r[4] = mem.read_u32(cpu.r[15]).unwrap_or(0); // mov.l @r15,r4
    cpu.r[4] = cpu.r[4].wrapping_add(116); // jsr delay: add #116,r4
    cpu.pr = 0x8c152e7e;
    crate::registry::p_8c0b4ff0::run(cpu, mem);

    cpu.r[4] = mem.read_u32(cpu.r[15]).unwrap_or(0); // mov.l @r15,r4
    cpu.r[1] = mem.read_u32(0x8c152ec8).unwrap_or(0x80);
    cpu.r[0] = mem.read_u32(0x8c152ecc).unwrap_or(0x8c0b2ee4);
    cpu.r[4] = cpu.r[4].wrapping_add(cpu.r[1]); // jsr delay: add r1,r4
    cpu.pr = 0x8c152e88;
    crate::registry::p_8c0b2ee4::run(cpu, mem);

    cpu.r[4] = mem.read_u32(cpu.r[15]).unwrap_or(0); // mov.l @r15,r4
    cpu.r[1] = mem.read_u32(0x8c152ed0).unwrap_or(0xb0);
    cpu.r[0] = mem.read_u32(0x8c152ec4).unwrap_or(0x8c0b4ff0);
    cpu.r[4] = cpu.r[4].wrapping_add(cpu.r[1]); // jsr delay: add r1,r4
    cpu.pr = 0x8c152e92;
    crate::registry::p_8c0b4ff0::run(cpu, mem);

    cpu.r[4] = mem.read_u32(cpu.r[15]).unwrap_or(0); // mov.l @r15,r4
    cpu.r[1] = mem.read_u32(0x8c152ed4).unwrap_or(0xbc);
    cpu.r[0] = mem.read_u32(0x8c152ec4).unwrap_or(0x8c0b4ff0);
    cpu.r[4] = cpu.r[4].wrapping_add(cpu.r[1]); // jsr delay: add r1,r4
    cpu.pr = 0x8c152e9c;
    crate::registry::p_8c0b4ff0::run(cpu, mem);

    cpu.r[1] = mem.read_u32(cpu.r[15]).unwrap_or(0); // mov.l @r15,r1
    cpu.fr[0] = 0.0; // fldi0 fr0
    cpu.r[0] = mem.read_u32(0x8c152ed8).unwrap_or(0xc8);
    cpu.r[1] = cpu.r[1].wrapping_add(cpu.r[0]); // add r0,r1
    let _ = mem.write_u32(cpu.r[1], cpu.fr[0].to_bits()); // fmov fr0,@r1
    cpu.r[1] = mem.read_u32(0x8c152edc).unwrap_or(0xcc);
    cpu.r[4] = mem.read_u32(cpu.r[15]).unwrap_or(0); // mov.l @r15,r4
    cpu.r[0] = mem.read_u32(0x8c152ec4).unwrap_or(0x8c0b4ff0);
    cpu.r[4] = cpu.r[4].wrapping_add(cpu.r[1]); // jsr delay: add r1,r4
    cpu.pr = 0x8c152eb0;
    crate::registry::p_8c0b4ff0::run(cpu, mem);

    cpu.r[0] = mem.read_u32(cpu.r[15]).unwrap_or(0); // mov.l @r15,r0
    cpu.r[15] = cpu.r[15].wrapping_add(4); // add #4,r15
    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0); // lds.l @r15+,pr
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    // rts; delay nop
}
