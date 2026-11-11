//! 0x8c0e2ed4 — unpack the packed RGBA word in r5 into four floats written at
//! r4+8, r4+12, r4+16, r4+20: fr4 = b3/255, fr2 = b2/255, fr1 = b1/255,
//! fr0 = b0/255, divided by the ROM constant 255.0f at 0x8c0e2f38 (fr3).
//! Single path, no branches.
//!
//! ```text
//! add #-16,r15            ; spill window (excluded from comparison)
//! extract r5 bytes: r7=b3, r6=b2, r3=b1, r1=b0
//! fmov @0x8c0e2f38,fr3    ; fr3 = 255.0f
//! lds r7,fpul; float fpul,fr4; lds r6,fpul; fdiv fr3,fr4
//! float fpul,fr2          ; fpul still b2
//! lds r3,fpul; float fpul,fr1; lds r1,fpul
//! fmov fr4,@(r0,r15); fdiv fr3,fr2; fmov fr2,@(r0,r15)
//! float fpul,fr0; fdiv fr3,fr1; fmov fr1,@(r0,r15); fdiv fr3,fr0; fmov fr0,@(r0,r15)
//! reload spilled words; store at r4+8..r4+20; rts; add #16,r15
//! ```
//! Hidden side effects the C does not show: r4 ends at param+8, r3 = r15-16,
//! r0/r1 hold the reloaded fr0/fr1 bits, r6/r7 hold bytes b2/b3, fpul = b0,
//! fr0..fr4 all clobbered, and the r15 spill/restore pair.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[15] = cpu.r[15].wrapping_sub(16); // add #-16,r15
    let color = cpu.r[5];
    let b3 = (color >> 24) & 0xFF;
    let b2 = (color >> 16) & 0xFF;
    let b1 = (color >> 8) & 0xFF;
    let b0 = color & 0xFF;
    cpu.r[7] = b3; // extu.b r1,r7
    cpu.r[6] = b2; // extu.b r3,r6
    cpu.r[3] = b1; // extu.b r1,r3
    cpu.r[1] = b0; // extu.b r5,r1 (r1 later reloaded)
    cpu.fr[3] = mem.read_f32(0x8C0E2F38).unwrap(); // 255.0f
    cpu.fpul = cpu.r[7];
    cpu.fr[4] = cpu.fpul as i32 as f32; // float fpul,fr4
    cpu.fpul = cpu.r[6];
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
    cpu.r[4] = cpu.r[4].wrapping_add(8);
    cpu.r[0] = mem.read_u32(cpu.r[3] + 4).unwrap();
    let _ = mem.write_u32(cpu.r[4], cpu.r[1]);
    let _ = mem.write_u32(cpu.r[4] + 4, cpu.r[0]);
    cpu.r[1] = mem.read_u32(cpu.r[3] + 8).unwrap();
    cpu.r[0] = mem.read_u32(cpu.r[3] + 12).unwrap();
    let _ = mem.write_u32(cpu.r[4] + 8, cpu.r[1]);
    let _ = mem.write_u32(cpu.r[4] + 12, cpu.r[0]);
    cpu.r[15] = cpu.r[15].wrapping_add(16); // rts delay slot
}
