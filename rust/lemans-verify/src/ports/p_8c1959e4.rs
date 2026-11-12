//! 0x8c1959e4 - zero two fields, clear a 2-word frame into `r4+44`, then call.
//!
//! `[r4+16]` and `[r4+20]` are cleared, a 8-byte frame is zeroed (`[r15]` word and
//! byte `[r15+4]`), the frame's two words are copied to `[r4+44]` / `[r4+48]`, and
//! 0x8c074338 runs with `r4 = r15`, `r5 = -1`.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr);
    cpu.r[15] = cpu.r[15].wrapping_sub(8); // add #-8,r15
    cpu.r[0] = 0; // mov #0,r0
    let _ = mem.write_u32(cpu.r[4].wrapping_add(16), cpu.r[0]); // mov.l r0,@(16,r4)
    cpu.r[6] = cpu.r[15]; // mov r15,r6
    let _ = mem.write_u32(cpu.r[4].wrapping_add(20), cpu.r[0]); // mov.l r0,@(20,r4)
    cpu.r[5] = 0xffff_ffff; // mov #-1,r5
    let _ = mem.write_u32(cpu.r[15], cpu.r[0]); // mov.l r0,@r15
    cpu.r[4] = cpu.r[4].wrapping_add(44); // add #44,r4
    let _ = mem.write_u8(cpu.r[15].wrapping_add(4), cpu.r[0] as u8); // mov.b r0,@(4,r15)
    cpu.r[0] = mem.read_u32(0x8c195a10).unwrap_or(0x8c074338); // mov.l 0x8c195a10,r0
    cpu.r[3] = mem.read_u32(cpu.r[6]).unwrap_or(0); // mov.l @r6,r3
    cpu.r[1] = mem.read_u32(cpu.r[6].wrapping_add(4)).unwrap_or(0); // mov.l @(4,r6),r1
    let _ = mem.write_u32(cpu.r[4], cpu.r[3]); // mov.l r3,@r4
    let _ = mem.write_u32(cpu.r[4].wrapping_add(4), cpu.r[1]); // mov.l r1,@(4,r4)
    cpu.r[4] = cpu.r[6]; // jsr delay: mov r6,r4
    cpu.pr = 0x8c195a06;
    crate::registry::p_8c074338::run(cpu, mem);
    cpu.r[15] = cpu.r[15].wrapping_add(8); // add #8,r15
    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}
