//! 0x8c0fda80 — same RGBA unpack as 0x8c0e2ed4 but with the packed word in r6
//! and the four floats (fr4 = b3/255, fr2 = b2/255, fr1 = b1/255, fr0 = b0/255,
//! divisor 255.0f from 0x8c0fdae4) stored at r5+48, r5+52, r5+56, r5+60.
//! Single path, no branches.
//!
//! Hidden side effects the C does not show: r5 ends at param+48, r3 = r15-16,
//! r4 = byte b2 of the color, r7 = b3, r0/r1 hold the reloaded fr0/fr1 bits,
//! fpul = b0, fr0..fr4 clobbered, r15 spill/restore pair.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[15] = cpu.r[15].wrapping_sub(16); // add #-16,r15
    let color = cpu.r[6];
    let b3 = (color >> 24) & 0xFF;
    let b2 = (color >> 16) & 0xFF;
    let b1 = (color >> 8) & 0xFF;
    let b0 = color & 0xFF;
    cpu.r[7] = b3; // extu.b r1,r7
    cpu.r[4] = b2; // extu.b r3,r4
    cpu.r[3] = b1; // extu.b r1,r3 (r3 later overwritten by mov r15,r3)
    cpu.r[1] = b0; // extu.b r6,r1 (r1 later reloaded)
    cpu.fr[3] = mem.read_f32(0x8C0FDAE4).unwrap(); // 255.0f
    cpu.fpul = cpu.r[7];
    cpu.fr[4] = cpu.fpul as i32 as f32; // float fpul,fr4
    cpu.fpul = cpu.r[4];
    cpu.fr[4] = cpu.fr[4] / cpu.fr[3]; // fdiv fr3,fr4
    cpu.fr[2] = cpu.fpul as i32 as f32; // float fpul,fr2 (fpul still b2)
    cpu.fpul = cpu.r[3];
    cpu.fr[1] = cpu.fpul as i32 as f32; // float fpul,fr1
    cpu.fpul = cpu.r[1];
    cpu.r[0] = 0;
    let _ = mem.write_f32(cpu.r[15].wrapping_add(cpu.r[0]), cpu.fr[4]);
    cpu.fr[2] = cpu.fr[2] / cpu.fr[3]; // fdiv fr3,fr2
    cpu.r[0] = 4;
    let _ = mem.write_f32(cpu.r[15].wrapping_add(cpu.r[0]), cpu.fr[2]);
    cpu.fr[0] = cpu.fpul as i32 as f32; // float fpul,fr0 (fpul still b0)
    cpu.fr[1] = cpu.fr[1] / cpu.fr[3]; // fdiv fr3,fr1
    cpu.r[0] = 8;
    let _ = mem.write_f32(cpu.r[15].wrapping_add(cpu.r[0]), cpu.fr[1]);
    cpu.fr[0] = cpu.fr[0] / cpu.fr[3]; // fdiv fr3,fr0
    cpu.r[0] = 12;
    let _ = mem.write_f32(cpu.r[15].wrapping_add(cpu.r[0]), cpu.fr[0]);
    cpu.r[3] = cpu.r[15]; // mov r15,r3
    cpu.r[1] = mem.read_u32(cpu.r[3]).unwrap();
    cpu.r[5] = cpu.r[5].wrapping_add(48); // add #48,r5
    cpu.r[0] = mem.read_u32(cpu.r[3] + 4).unwrap();
    let _ = mem.write_u32(cpu.r[5], cpu.r[1]);
    let _ = mem.write_u32(cpu.r[5] + 4, cpu.r[0]);
    cpu.r[1] = mem.read_u32(cpu.r[3] + 8).unwrap();
    cpu.r[0] = mem.read_u32(cpu.r[3] + 12).unwrap();
    let _ = mem.write_u32(cpu.r[5] + 8, cpu.r[1]);
    let _ = mem.write_u32(cpu.r[5] + 12, cpu.r[0]);
    cpu.r[15] = cpu.r[15].wrapping_add(16); // rts delay slot
}
