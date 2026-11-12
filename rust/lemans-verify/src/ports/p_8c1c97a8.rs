//! 0x8c1c97a8 - stamp two vtable words, init the sub-object at `+28`, copy a
//! three-word table into `+4..+12`, then set the trailing fields.
//!
//! `[r4]` is written twice (0x8c1eb6a0 then 0x8c1f2b40, so the first store is
//! dead), 0x8c1261d4 runs on `r4+28`, and the three words at `[0x8c2156b8]` are
//! snapshotted through a frame into `+4`/`+8`/`+12`. Then `+16 = 0`, byte `+20 = 0`,
//! `+24 = r4` (self-link) and `+36 = 0x1f4`.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr);
    cpu.r[15] = cpu.r[15].wrapping_sub(16); // add #-16,r15
    let _ = mem.write_u32(cpu.r[15], cpu.r[4]); // mov.l r4,@r15

    cpu.r[3] = mem.read_u32(cpu.r[15]).unwrap_or(0); // mov.l @r15,r3
    cpu.r[0] = mem.read_u32(0x8c1c9808).unwrap_or(0x8c1eb6a0); // mov.l 0x8c1c9808,r0
    cpu.r[1] = mem.read_u32(0x8c1c980c).unwrap_or(0x8c1f2b40); // mov.l 0x8c1c980c,r1
    let _ = mem.write_u32(cpu.r[3], cpu.r[0]); // mov.l r0,@r3 (dead)
    cpu.r[0] = mem.read_u32(0x8c1c9810).unwrap_or(0x8c1261d4); // mov.l 0x8c1c9810,r0
    let _ = mem.write_u32(cpu.r[3], cpu.r[1]); // mov.l r1,@r3
    cpu.r[4] = mem.read_u32(cpu.r[15]).unwrap_or(0); // mov.l @r15,r4
    cpu.r[4] = cpu.r[4].wrapping_add(28); // jsr delay: add #28,r4
    cpu.pr = 0x8c1c97c0;
    crate::registry::p_8c1261d4::run(cpu, mem);

    cpu.r[1] = mem.read_u32(cpu.r[15]).unwrap_or(0); // mov.l @r15,r1
    cpu.r[7] = cpu.r[15]; // mov r15,r7
    cpu.r[0] = mem.read_u32(0x8c1c9814).unwrap_or(0x8c1f2b28); // mov.l 0x8c1c9814,r0
    cpu.r[7] = cpu.r[7].wrapping_add(4); // add #4,r7
    cpu.r[6] = mem.read_u32(0x8c1c9818).unwrap_or(0x8c2156b8); // mov.l 0x8c1c9818,r6
    cpu.r[4] = 0; // mov #0,r4
    let _ = mem.write_u32(cpu.r[1], cpu.r[0]); // mov.l r0,@r1
    cpu.r[5] = mem.read_u32(cpu.r[6]).unwrap_or(0); // mov.l @r6,r5
    cpu.r[0] = 20; // mov #20,r0
    cpu.r[3] = mem.read_u32(cpu.r[6].wrapping_add(4)).unwrap_or(0); // mov.l @(4,r6),r3
    cpu.r[1] = mem.read_u32(0x8c1c981c).unwrap_or(0x1f4); // mov.l 0x8c1c981c,r1
    let _ = mem.write_u32(cpu.r[7], cpu.r[5]); // mov.l r5,@r7
    let _ = mem.write_u32(cpu.r[7].wrapping_add(4), cpu.r[3]); // mov.l r3,@(4,r7)
    cpu.r[3] = mem.read_u32(cpu.r[6].wrapping_add(8)).unwrap_or(0); // mov.l @(8,r6),r3
    let _ = mem.write_u32(cpu.r[7].wrapping_add(8), cpu.r[3]); // mov.l r3,@(8,r7)
    cpu.r[3] = mem.read_u32(cpu.r[15]).unwrap_or(0); // mov.l @r15,r3
    let _ = mem.write_u32(cpu.r[3].wrapping_add(16), cpu.r[4]); // mov.l r4,@(16,r3)
    cpu.r[3] = mem.read_u32(cpu.r[15]).unwrap_or(0); // mov.l @r15,r3
    let _ = mem.write_u8(cpu.r[3].wrapping_add(cpu.r[0]), cpu.r[4] as u8); // mov.b r4,@(r0,r3)
    cpu.r[4] = mem.read_u32(cpu.r[15]).unwrap_or(0); // mov.l @r15,r4
    cpu.r[3] = mem.read_u32(cpu.r[7]).unwrap_or(0); // mov.l @r7,r3
    cpu.r[4] = cpu.r[4].wrapping_add(4); // add #4,r4
    cpu.r[0] = mem.read_u32(cpu.r[7].wrapping_add(4)).unwrap_or(0); // mov.l @(4,r7),r0
    let _ = mem.write_u32(cpu.r[4], cpu.r[3]); // mov.l r3,@r4
    let _ = mem.write_u32(cpu.r[4].wrapping_add(4), cpu.r[0]); // mov.l r0,@(4,r4)
    cpu.r[0] = mem.read_u32(cpu.r[7].wrapping_add(8)).unwrap_or(0); // mov.l @(8,r7),r0
    let _ = mem.write_u32(cpu.r[4].wrapping_add(8), cpu.r[0]); // mov.l r0,@(8,r4)
    cpu.r[0] = mem.read_u32(cpu.r[15]).unwrap_or(0); // mov.l @r15,r0
    let _ = mem.write_u32(cpu.r[0].wrapping_add(24), cpu.r[0]); // mov.l r0,@(24,r0)
    cpu.r[0] = mem.read_u32(cpu.r[15]).unwrap_or(0); // mov.l @r15,r0
    let _ = mem.write_u32(cpu.r[0].wrapping_add(36), cpu.r[1]); // mov.l r1,@(36,r0)
    cpu.r[0] = mem.read_u32(cpu.r[15]).unwrap_or(0); // mov.l @r15,r0
    cpu.r[15] = cpu.r[15].wrapping_add(16); // add #16,r15
    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}
