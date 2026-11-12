//! 0x8c2188c0 - lazily initialise a descriptor, keyed on the word at `[r4+76]`.
//!
//! When `[r4+76]` is already non-zero the function returns 0 without touching
//! anything. Otherwise it stores `r5`/`r6`/`r7` at `+4`/`+8`/`+12`, derives
//! `+16` as `(r7 + 0x7ff) >> 11` (arithmetic `shad` by -11), zeroes the rest of
//! the block, sets `+76` to 1, copies the word at `[0x8c23c2ec]` to `+78`, and
//! returns r4.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[0] = 76; // mov #76,r0
    cpu.r[3] = mem.read_u16(cpu.r[4].wrapping_add(cpu.r[0])).unwrap_or(0) as i16 as i32 as u32; // mov.w (sign-extends)
    cpu.set_t(cpu.r[3] == 0); // tst r3,r3
    if !cpu.t() {
        cpu.r[0] = 0; // rts delay: mov #0,r0
        return;
    }
    cpu.r[3] = mem.read_u16(0x8c2189f0).unwrap_or(0x7ff) as i16 as i32 as u32; // mov.w (sign-extends)
    cpu.r[2] = cpu.r[7]; // mov r7,r2
    cpu.r[1] = 0xffff_fff5; // mov #-11,r1
    let _ = mem.write_u32(cpu.r[4].wrapping_add(4), cpu.r[5]); // mov.l r5,@(4,r4)
    cpu.r[2] = cpu.r[2].wrapping_add(cpu.r[3]); // add r3,r2
    let _ = mem.write_u32(cpu.r[4].wrapping_add(8), cpu.r[6]); // mov.l r6,@(8,r4)
    cpu.r[5] = 0; // mov #0,r5
    let _ = mem.write_u32(cpu.r[4].wrapping_add(12), cpu.r[7]); // mov.l r7,@(12,r4)
    cpu.r[2] = (cpu.r[2] as i32 >> 11) as u32; // shad r1,r2
    cpu.r[0] = 64; // mov #64,r0
    let _ = mem.write_u32(cpu.r[4].wrapping_add(16), cpu.r[2]); // mov.l r2,@(16,r4)
    for off in [20u32, 24, 28, 32, 36, 44, 52, 60] {
        let _ = mem.write_u32(cpu.r[4].wrapping_add(off), cpu.r[5]); // mov.l r5,@(off,r4)
    }
    let _ = mem.write_u32(cpu.r[4].wrapping_add(64), cpu.r[5]); // mov.l r5,@(r0,r4)
    cpu.r[0] = 68; // mov #68,r0
    let _ = mem.write_u32(cpu.r[4].wrapping_add(68), cpu.r[5]); // mov.l r5,@(r0,r4)
    cpu.r[0] = 72; // mov #72,r0
    let _ = mem.write_u16(cpu.r[4].wrapping_add(72), cpu.r[5] as u16); // mov.w r5,@(r0,r4)
    cpu.r[0] = 74; // mov #74,r0
    let _ = mem.write_u16(cpu.r[4].wrapping_add(74), cpu.r[5] as u16); // mov.w r5,@(r0,r4)
    cpu.r[0] = 76; // mov #76,r0
    cpu.r[2] = 1; // mov #1,r2
    let _ = mem.write_u16(cpu.r[4].wrapping_add(76), cpu.r[2] as u16); // mov.w r2,@(r0,r4)
    cpu.r[0] = 78; // mov #78,r0
    cpu.r[2] = mem.read_u32(0x8c2189f4).unwrap_or(0x8c23c2ec); // mov.l 0x8c2189f4,r2
    cpu.r[1] = mem.read_u16(cpu.r[2]).unwrap_or(0) as u32; // mov.w @r2,r1
    let _ = mem.write_u16(cpu.r[4].wrapping_add(78), cpu.r[1] as u16); // mov.w r1,@(r0,r4)
    cpu.r[0] = 80; // mov #80,r0
    let _ = mem.write_u16(cpu.r[4].wrapping_add(80), cpu.r[5] as u16); // mov.w r5,@(r0,r4)
    cpu.r[0] = 82; // mov #82,r0
    let _ = mem.write_u16(cpu.r[4].wrapping_add(82), cpu.r[5] as u16); // mov.w r5,@(r0,r4)
    cpu.r[0] = cpu.r[4]; // mov r4,r0
}
