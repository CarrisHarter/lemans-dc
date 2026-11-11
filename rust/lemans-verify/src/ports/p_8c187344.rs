//! 0x8c187344 — record init with two header overwrites and a float block.
//!
//! After 7 pushes and r15 -= 16 (local L = r15-44): *r4 = 0x8c1f0498 (pool
//! 0x8c187444); B = 0x8c1d3f68 (pool 0x8c187448) 3 words to r4+4..12; C =
//! 0x8c1d0f28 (pool 0x8c18744c) 4 words each to r4+16, +32, +64, +48; r4+80 =
//! 0.0f; byte r4+84 = 0; *r4 = 0x8c1f0448 (pool 0x8c187450); r4+80 =
//! *(0x8c187454); C quad again to r4+16; B triple again to r4+4; stack L gets
//! 1.0/0.5/0.4/0.3 (pools 0x8c187458/45c/460) which are copied to r4+32..44;
//! byte r4+84 = 0 again. Final: r0 = r4, r1 = 0, r2 = L+12, r3 = r4+32,
//! r5 = 0.4f bits, r6 = r4+16, r7 = 0x8c1d3f68, r8..r14 restored,
//! fr0 = 0.3f, fr1 = *(0x8c187454).

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let r4 = cpu.r[4];
    let l = cpu.r[15].wrapping_sub(44); // 7 pushes (28) then add #-16,r15
    let b = mem.read_u32(0x8c187448).unwrap(); // 0x8c1d3f68
    let c = mem.read_u32(0x8c18744c).unwrap(); // 0x8c1d0f28

    let put = |mem: &mut Memory, d: u32, s: u32, n: u32| {
        for i in 0..n {
            let v = mem.read_u32(s + i * 4).unwrap();
            let _ = mem.write_u32(d + i * 4, v);
        }
    };

    let _ = mem.write_u32(r4, mem.read_u32(0x8c187444).unwrap()); // 0x8c1f0498
    put(mem, r4 + 4, b, 3); // B triple -> r4+4..12
    put(mem, r4 + 16, c, 4); // C quad -> r4+16..28
    put(mem, r4 + 32, c, 4); // C quad -> r4+32..44
    put(mem, r4 + 64, c, 4); // C quad -> r4+64..76
    put(mem, r4 + 48, c, 4); // C quad -> r4+48..60
    cpu.fr[1] = 0.0; // fldi0 fr1
    let _ = mem.write_u32(r4 + 80, cpu.fr[1].to_bits()); // fmov fr1,@r12
    let _ = mem.write_u8(r4 + 84, 0); // mov.b r1,@(r0,r4), r1 = 0, r0 = 84
    let _ = mem.write_u32(r4, mem.read_u32(0x8c187450).unwrap()); // 0x8c1f0448
    cpu.fr[1] = f32::from_bits(mem.read_u32(0x8c187454).unwrap());
    let _ = mem.write_u32(r4 + 80, cpu.fr[1].to_bits()); // fmov fr1,@r12
    put(mem, r4 + 16, c, 4); // C quad again
    put(mem, r4 + 4, b, 3); // B triple again
    cpu.fr[0] = 1.0; // fldi1 fr0 (was set early)
    // stack floats 1.0, 0.5, 0.4, 0.3 at L+0..12, then copied to r4+32..44
    let f0 = 1.0f32;
    let f1 = f32::from_bits(mem.read_u32(0x8c187458).unwrap()); // 0.5
    let f2 = f32::from_bits(mem.read_u32(0x8c18745c).unwrap()); // 0.4
    let f3 = f32::from_bits(mem.read_u32(0x8c187460).unwrap()); // 0.3
    let _ = mem.write_u32(r4 + 32, f0.to_bits());
    let _ = mem.write_u32(r4 + 36, f1.to_bits());
    let _ = mem.write_u32(r4 + 40, f2.to_bits());
    let _ = mem.write_u32(r4 + 44, f3.to_bits());
    let _ = mem.write_u8(r4 + 84, 0); // second mov.b
    cpu.fr[0] = f3; // last fmov @r0,fr0 with r0 = 0x8c187460

    cpu.r[0] = r4; // mov r4,r0
    cpu.r[1] = 0;
    cpu.r[2] = l.wrapping_add(12);
    cpu.r[3] = r4.wrapping_add(32);
    cpu.r[5] = f2.to_bits(); // r5 = *(L+8) = 0.4f bits
    cpu.r[6] = r4.wrapping_add(16);
    cpu.r[7] = b; // 0x8c1d3f68
}
