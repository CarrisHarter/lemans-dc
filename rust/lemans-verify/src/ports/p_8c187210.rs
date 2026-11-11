//! 0x8c187210 — mixed record copy.
//!
//! After 5 pushes: 16 bytes from r5 to r4+0..12; 16 bytes from r6 to r4+16..28
//! (r4 := *(r6+12)); 16 bytes from r7 to r4+32..44 (r4 := *(r7+12)); arg1 =
//! *r15, 16 bytes to r4+48..60 (r4 := *(arg1+12)); fr4 to r4+64; arg2 =
//! *(r15+4), 12 bytes to r4+68..76; fr5 to r4+80. Final regs: r0 = r4_orig,
//! r1 = r4_orig+68, r3 = *(arg2+8), r4 = *(arg1+12), r5 = arg2, r6 = arg1,
//! r10..r14 restored.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let r4 = cpu.r[4];
    let r5 = cpu.r[5];
    let r6 = cpu.r[6];
    let r7 = cpu.r[7];
    let r13 = r4.wrapping_add(16);
    let r14 = r4.wrapping_add(32);
    let r3p = r4.wrapping_add(48);
    let r11 = r4.wrapping_add(64);
    let r1p = r4.wrapping_add(68);
    let r12 = r4.wrapping_add(80);

    let mut put4 = |mem: &mut Memory, d: u32, s: u32| {
        for i in 0..4u32 {
            let v = mem.read_u32(s + i * 4).unwrap();
            let _ = mem.write_u32(d + i * 4, v);
        }
    };
    put4(mem, r4, r5); // r4+0..12 from r5
    put4(mem, r13, r6); // r4+16..28 from r6
    put4(mem, r14, r7); // r4+32..44 from r7
    let arg1 = mem.read_u32(cpu.r[15]).unwrap(); // @(20,r15) after 5 pushes
    put4(mem, r3p, arg1); // r4+48..60 from arg1
    let _ = mem.write_u32(r11, cpu.fr[4].to_bits()); // fmov fr4,@r11 -> r4+64
    let arg2 = mem.read_u32(cpu.r[15] + 4).unwrap(); // @(24,r15)
    let a = mem.read_u32(arg2).unwrap();
    let b = mem.read_u32(arg2 + 4).unwrap();
    let c = mem.read_u32(arg2 + 8).unwrap();
    let _ = mem.write_u32(r1p, a); // r4+68
    let _ = mem.write_u32(r1p + 4, b); // r4+72
    let _ = mem.write_u32(r1p + 8, c); // r4+76
    let _ = mem.write_u32(r12, cpu.fr[5].to_bits()); // fmov fr5,@r12 -> r4+80

    cpu.r[0] = r4; // mov r4,r0
    cpu.r[1] = r1p;
    cpu.r[3] = c; // mov.l @(8,r5),r3 with r5 = arg2
    cpu.r[4] = a; // mov.l @r5,r4 with r5 = arg2 (last r4 write)
    cpu.r[5] = arg2;
    cpu.r[6] = arg1;
}
