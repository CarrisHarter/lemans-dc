//! 0x8c14aeec - initialise the object: reset two sub-arrays and stamp defaults.
//!
//! 0x8c14b2ec runs on `r4+4`, then `[r14+16] = -1`, `[r14+20] = 0`,
//! `[r14+60] = 0`, `[r14+64] = 0`, 0x8c14d210 on `r4+68`, then 0 at +80/+84/+88
//! and 1 at +92 (word) and +96 (byte). r14 and pr are restored.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[14]);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr);

    cpu.r[14] = cpu.r[4]; // mov r4,r14
    cpu.r[0] = mem.read_u32(0x8c14af2c).unwrap_or(0x8c14b2ec); // mov.l 0x8c14af2c,r0
    cpu.r[4] = cpu.r[4].wrapping_add(4); // jsr delay: add #4,r4
    cpu.pr = 0x8c14aef8;
    crate::registry::p_8c14b2ec::run(cpu, mem);

    cpu.r[0] = 0xffff_ffff; // mov #-1,r0
    let _ = mem.write_u32(cpu.r[14].wrapping_add(16), cpu.r[0]); // mov.l r0,@(16,r14)
    cpu.r[1] = 0; // mov #0,r1
    let _ = mem.write_u32(cpu.r[14].wrapping_add(20), cpu.r[1]); // mov.l r1,@(20,r14)
    cpu.r[4] = cpu.r[14]; // mov r14,r4
    let _ = mem.write_u32(cpu.r[14].wrapping_add(60), cpu.r[1]); // mov.l r1,@(60,r14)
    cpu.r[0] = 64; // mov #64,r0
    let _ = mem.write_u32(cpu.r[14].wrapping_add(64), cpu.r[1]); // mov.l r1,@(r0,r14)

    cpu.r[0] = mem.read_u32(0x8c14af30).unwrap_or(0x8c14d210); // mov.l 0x8c14af30,r0
    cpu.r[4] = cpu.r[4].wrapping_add(68); // jsr delay: add #68,r4
    cpu.pr = 0x8c14af0e;
    crate::registry::p_8c14d210::run(cpu, mem);

    cpu.r[0] = 80; // mov #80,r0
    cpu.r[3] = 0; // mov #0,r3
    let _ = mem.write_u32(cpu.r[14].wrapping_add(80), cpu.r[3]); // mov.l r3,@(r0,r14)
    cpu.r[1] = 1; // mov #1,r1
    cpu.r[0] = 84; // mov #84,r0
    let _ = mem.write_u32(cpu.r[14].wrapping_add(84), cpu.r[3]); // mov.l r3,@(r0,r14)
    cpu.r[0] = 88; // mov #88,r0
    let _ = mem.write_u32(cpu.r[14].wrapping_add(88), cpu.r[3]); // mov.l r3,@(r0,r14)
    cpu.r[0] = 92; // mov #92,r0
    let _ = mem.write_u32(cpu.r[14].wrapping_add(92), cpu.r[1]); // mov.l r1,@(r0,r14)
    cpu.r[0] = 96; // mov #96,r0
    let _ = mem.write_u8(cpu.r[14].wrapping_add(96), cpu.r[1] as u8); // mov.b r1,@(r0,r14)

    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[14] = mem.read_u32(cpu.r[15]).unwrap_or(0); // rts delay
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}
