//! 0x8c16d690 - initialise a 20-entry table with 0x200-stride records.
//!
//! Zeroes a byte at `+5+i`, two word arrays at `+28+4i` and `+108+4i`, and a
//! global at `0x8c20bad0+4i`, then strcpys the string at `0x8c20cf87` into
//! `0x8c2b2a28 + i*0x200` for i = 0..19. Afterwards it clears
//! `[0x8c2b5228]`, sets byte `[0x8c20bb20] = 1`, stores the 400.0 float from
//! `0x8c16d764` at `0x8c20bb28`, and memsets 24 bytes at `base+0xdc`.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[14]);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[13]);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[12]);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[11]);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[10]);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr);

    cpu.r[10] = cpu.r[4]; // mov r4,r10
    cpu.r[14] = 0; // mov #0,r14
    cpu.r[0] = cpu.r[14]; // mov r14,r0
    let _ = mem.write_u8(cpu.r[10].wrapping_add(4), cpu.r[0] as u8); // mov.b r0,@(4,r10)
    cpu.r[12] = cpu.r[14]; // mov r14,r12
    cpu.r[0] = mem.read_u32(0x8c16d72c).unwrap_or(0xc4); // mov.l 0x8c16d72c,r0
    cpu.r[1] = 127; // mov #127,r1
    let _ = mem.write_u32(cpu.r[10].wrapping_add(cpu.r[0]), cpu.r[14]); // mov.l r14,@(r0,r10)
    cpu.r[0] = mem.read_u32(0x8c16d730).unwrap_or(0xcc); // mov.l 0x8c16d730,r0
    let _ = mem.write_u32(cpu.r[10].wrapping_add(cpu.r[0]), cpu.r[14]); // mov.l r14,@(r0,r10)
    cpu.r[11] = mem.read_u32(0x8c16d734).unwrap_or(0x200); // mov.l 0x8c16d734,r11
    cpu.r[0] = mem.read_u32(0x8c16d738).unwrap_or(0xd0); // mov.l 0x8c16d738,r0
    let product = (cpu.r[11] as i32 as i64) * (cpu.r[12] as i32 as i64); // mul.l r11,r12
    cpu.mach = (product >> 32) as u32;
    cpu.macl = product as u32;
    let _ = mem.write_u32(cpu.r[10].wrapping_add(cpu.r[0]), cpu.r[1]); // mov.l r1,@(r0,r10)
    cpu.r[12] = cpu.macl; // sts macl,r12
    cpu.r[0] = mem.read_u32(0x8c16d73c).unwrap_or(0xd8); // mov.l 0x8c16d73c,r0
    cpu.r[13] = cpu.r[14]; // mov r14,r13
    let _ = mem.write_u32(cpu.r[10].wrapping_add(cpu.r[0]), cpu.r[1]); // mov.l r1,@(r0,r10)
    cpu.r[3] = mem.read_u32(cpu.r[10].wrapping_add(cpu.r[0])).unwrap_or(0); // mov.l @(r0,r10),r3
    cpu.r[1] = mem.read_u32(0x8c16d740).unwrap_or(0x8c2b2a28); // mov.l 0x8c16d740,r1
    cpu.r[0] = mem.read_u32(0x8c16d744).unwrap_or(0xd4); // mov.l 0x8c16d744,r0
    cpu.r[12] = cpu.r[12].wrapping_add(cpu.r[1]); // add r1,r12
    let _ = mem.write_u32(cpu.r[10].wrapping_add(cpu.r[0]), cpu.r[3]); // mov.l r3,@(r0,r10)

    loop {
        cpu.r[1] = cpu.r[10]; // mov r10,r1
        cpu.r[0] = cpu.r[14]; // mov r14,r0
        cpu.r[1] = cpu.r[1].wrapping_add(5); // add #5,r1
        cpu.r[4] = 0; // mov #0,r4
        let _ = mem.write_u8(cpu.r[1].wrapping_add(cpu.r[0]), cpu.r[4] as u8); // mov.b r4,@(r0,r1)
        cpu.r[3] = cpu.r[10]; // mov r10,r3
        cpu.r[1] = cpu.r[10]; // mov r10,r1
        cpu.r[0] = cpu.r[13]; // mov r13,r0
        cpu.r[3] = cpu.r[3].wrapping_add(28); // add #28,r3
        let _ = mem.write_u32(cpu.r[3].wrapping_add(cpu.r[0]), cpu.r[4]); // mov.l r4,@(r0,r3)
        cpu.r[1] = cpu.r[1].wrapping_add(108); // add #108,r1
        let _ = mem.write_u32(cpu.r[1].wrapping_add(cpu.r[0]), cpu.r[4]); // mov.l r4,@(r0,r1)
        cpu.r[5] = mem.read_u32(0x8c16d748).unwrap_or(0x8c20ce68); // mov.l 0x8c16d748,r5
        cpu.r[1] = mem.read_u32(0x8c16d74c).unwrap_or(0x11f); // mov.l 0x8c16d74c,r1
        cpu.r[0] = mem.read_u32(0x8c16d750).unwrap_or(0x8c20bad0); // mov.l 0x8c16d750,r0
        cpu.r[5] = cpu.r[5].wrapping_add(cpu.r[1]); // add r1,r5
        let _ = mem.write_u32(cpu.r[0].wrapping_add(cpu.r[13]), cpu.r[4]); // mov.l r4,@(r0,r13)
        cpu.r[0] = mem.read_u32(0x8c16d754).unwrap_or(0x8c017f48); // mov.l 0x8c16d754,r0
        cpu.r[4] = cpu.r[12]; // jsr delay: mov r12,r4
        cpu.pr = 0x8c16d6f2;
        crate::registry::p_8c017f48::run(cpu, mem);
        cpu.r[14] = cpu.r[14].wrapping_add(1); // add #1,r14
        cpu.r[0] = 20; // mov #20,r0
        cpu.set_t((cpu.r[0] as i32) > (cpu.r[14] as i32)); // cmp/gt r14,r0
        cpu.r[13] = cpu.r[13].wrapping_add(4); // add #4,r13
        let more = cpu.t();
        cpu.r[12] = cpu.r[12].wrapping_add(cpu.r[11]); // bt.s delay: add r11,r12
        if !more {
            break;
        }
    }

    cpu.r[0] = mem.read_u32(0x8c16d758).unwrap_or(0xdc); // mov.l 0x8c16d758,r0
    cpu.r[5] = 0; // mov #0,r5
    cpu.r[4] = mem.read_u32(0x8c16d75c).unwrap_or(0x8c2b5228); // mov.l 0x8c16d75c,r4
    cpu.r[10] = cpu.r[10].wrapping_add(cpu.r[0]); // add r0,r10
    cpu.r[3] = mem.read_u32(0x8c16d760).unwrap_or(0x8c20bb20); // mov.l 0x8c16d760,r3
    cpu.r[1] = 1; // mov #1,r1
    cpu.r[0] = 0x8c16d764; // mova 0x8c16d764,r0
    let _ = mem.write_u32(cpu.r[4], cpu.r[5]); // mov.l r5,@r4
    let _ = mem.write_u8(cpu.r[3], cpu.r[1] as u8); // mov.b r1,@r3
    cpu.r[4] = cpu.r[10]; // mov r10,r4
    cpu.fr[0] = f32::from_bits(mem.read_u32(cpu.r[0]).unwrap_or(0)); // fmov @r0,fr0
    cpu.r[0] = mem.read_u32(0x8c16d768).unwrap_or(0x8c20bb28); // mov.l 0x8c16d768,r0
    let _ = mem.write_u32(cpu.r[0], cpu.fr[0].to_bits()); // fmov fr0,@r0
    cpu.r[0] = mem.read_u32(0x8c16d76c).unwrap_or(0x8c010f88); // mov.l 0x8c16d76c,r0
    cpu.r[6] = 24; // jsr delay: mov #24,r6
    cpu.pr = 0x8c16d71e;
    crate::registry::p_8c010f88::run(cpu, mem);

    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[10] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[11] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[12] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[13] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[14] = mem.read_u32(cpu.r[15]).unwrap_or(0); // rts delay
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}
