//! 0x8c19a648 — copy a 0xe4-byte entity from r5 to r4.
//!
//! Straight-line block copy: dwords at 0/4, floats at 8..32 (step 4),
//! words at 36..106 (step 2), dwords at 108..128, float at 0x84, dwords at
//! 0x88..0x94 and 0x98..0xa4, float at 0xa8, dwords at 0xac,0xb0,0xb4,0xb8,
//! 0xbc,0xc0,0xc4, floats at 0xc8,0xcc,0xd0 and 0xd4, bytes at 0xd8..0xe3.
//! Prologue pushes r14..r9 and the epilogue pops them, so those registers and
//! r15 come back unchanged (stack traffic is outside the diff window).
//! Final: r0 = r4, r1 = byte[r5+0xe3], r3 = r4+0xc8, r6 = r5+0xd4,
//! r7 = r4+0xd4, fr0 = f32[r5+0xd4]; T untouched.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let r4 = cpu.r[4];
    let r5 = cpu.r[5];

    mem.write_u32(r4, mem.read_u32(r5).unwrap()).unwrap();
    mem.write_u32(r4 + 4, mem.read_u32(r5 + 4).unwrap()).unwrap();

    for off in (8u32..=32).step_by(4) {
        let v = mem.read_f32(r5 + off).unwrap();
        mem.write_f32(r4 + off, v).unwrap();
        cpu.fr[0] = v;
    }
    for off in (36u32..=106).step_by(2) {
        let v = mem.read_u16(r5 + off).unwrap();
        mem.write_u16(r4 + off, v).unwrap();
    }
    for off in (108u32..=128).step_by(4) {
        mem.write_u32(r4 + off, mem.read_u32(r5 + off).unwrap()).unwrap();
    }
    mem.write_f32(r4 + 0x84, mem.read_f32(r5 + 0x84).unwrap()).unwrap();
    for off in (0x88u32..=0x94).step_by(4) {
        mem.write_u32(r4 + off, mem.read_u32(r5 + off).unwrap()).unwrap();
    }
    for off in (0x98u32..=0xa4).step_by(4) {
        mem.write_u32(r4 + off, mem.read_u32(r5 + off).unwrap()).unwrap();
    }
    mem.write_f32(r4 + 0xa8, mem.read_f32(r5 + 0xa8).unwrap()).unwrap();
    for off in [0xacu32, 0xb0, 0xb4, 0xb8, 0xbc, 0xc0, 0xc4] {
        mem.write_u32(r4 + off, mem.read_u32(r5 + off).unwrap()).unwrap();
    }
    for off in [0xc8u32, 0xcc, 0xd0] {
        mem.write_f32(r4 + off, mem.read_f32(r5 + off).unwrap()).unwrap();
    }
    let v = mem.read_f32(r5 + 0xd4).unwrap();
    mem.write_f32(r4 + 0xd4, v).unwrap();
    cpu.fr[0] = v;
    for off in 0xd8u32..=0xe3 {
        let b = mem.read_u8(r5 + off).unwrap();
        mem.write_u8(r4 + off, b).unwrap();
        cpu.r[1] = b as u32; // extu.b r1,r1 each time; last one wins
    }

    cpu.r[0] = r4;
    cpu.r[3] = r4 + 0xc8;
    cpu.r[6] = r5 + 0xd4;
    cpu.r[7] = r4 + 0xd4;
}
