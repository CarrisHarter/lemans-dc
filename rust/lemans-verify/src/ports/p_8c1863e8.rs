//! 0x8c1863e8 — template record builder.
//!
//! Copies the 16-byte global at 0x8c1d0f18 (pool 0x8c1864a0) into r4+0, +16,
//! +32, +48, +64, +80 (four words each); then the 12-byte global at
//! 0x8c1d3f68 (pool 0x8c1864a4) into r4+96..104; r4+108 = 1.0f (fldi1 fr0,
//! fmov fr0,@r1) and a zero byte at r4+112 (r8 = 0, r0 = 112). Final regs:
//! r0 = r4 (rts delay slot), r1 = r4+108, r3 = *(B+8), r5 = r4+96,
//! r6 = *(B+4), r7 = *B, r8..r14 restored, fr0 = 1.0.

use sh4_oracle::{Cpu, Memory};

fn put(mem: &mut Memory, d: u32, s: u32, n: u32) {
    for i in 0..n {
        let v = mem.read_u32(s + i * 4).unwrap();
        let _ = mem.write_u32(d + i * 4, v);
    }
}

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let r4 = cpu.r[4];
    let a = mem.read_u32(0x8c1864a0).unwrap(); // 0x8c1d0f18
    let b = mem.read_u32(0x8c1864a4).unwrap(); // 0x8c1d3f68

    put(mem, r4, a, 4); // r4+0..12
    put(mem, r4 + 16, a, 4);
    put(mem, r4 + 32, a, 4);
    put(mem, r4 + 48, a, 4);
    put(mem, r4 + 64, a, 4);
    put(mem, r4 + 80, a, 4);
    let b0 = mem.read_u32(b).unwrap();
    let b1 = mem.read_u32(b + 4).unwrap();
    let b2 = mem.read_u32(b + 8).unwrap();
    let _ = mem.write_u32(r4 + 96, b0);
    let _ = mem.write_u32(r4 + 100, b1);
    let _ = mem.write_u32(r4 + 104, b2);
    cpu.fr[0] = 1.0; // fldi1 fr0
    let _ = mem.write_u32(r4 + 108, cpu.fr[0].to_bits()); // fmov fr0,@r1
    let _ = mem.write_u8(r4 + 112, 0); // mov.b r8,@(r0,r4), r8 = 0, r0 = 112

    cpu.r[0] = r4; // rts delay slot: mov r4,r0
    cpu.r[1] = r4.wrapping_add(108);
    cpu.r[3] = b2;
    cpu.r[5] = r4.wrapping_add(96);
    cpu.r[6] = b1;
    cpu.r[7] = b0;
}
