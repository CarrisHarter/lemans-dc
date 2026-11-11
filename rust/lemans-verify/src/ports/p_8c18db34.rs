//! 0x8c18db34 — copy a 32-byte record from r5 to r4 (compact variant of
//! 0x8c18d9d4).
//!
//! Layout: u32s at +0/+4/+8, an f32 at +12, bytes at +16/+17/+18, u32s at
//! +20/+24/+28 (the last store sits in the `rts` delay slot). Only r14 is
//! saved (popped in the delay slot). Exit: r0 = r4, r1 = src+28 word,
//! r3 = zero-extended src byte +17, r6 = zero-extended src byte +16,
//! r7 = r5+12; fr0 = the copied float; T untouched.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let r4 = cpu.r[4];
    let r5 = cpu.r[5];
    let r15_entry = cpu.r[15];
    let r15 = r15_entry.wrapping_sub(4); // mov.l r14,@-r15
    let _ = mem.write_u32(r15, cpu.r[14]);

    let mut r0 = mem.read_u32(r5).unwrap_or(0);
    let mut r7 = r5;
    let mut r14 = r4;
    let _ = mem.write_u32(r4, r0);
    let mut r6 = r5;
    r0 = mem.read_u32(r5.wrapping_add(4)).unwrap_or(0);
    let mut r3 = r5;
    let mut r1 = r5;
    let _ = mem.write_u32(r4.wrapping_add(4), r0);
    r7 = r7.wrapping_add(12);
    r0 = mem.read_u32(r5.wrapping_add(8)).unwrap_or(0);
    r14 = r14.wrapping_add(12);
    r6 = r6.wrapping_add(16);
    let _ = mem.write_u32(r4.wrapping_add(8), r0);
    r3 = r3.wrapping_add(17);
    let fr0 = mem.read_f32(r7).unwrap_or(0.0); // fmov @r7,fr0 (r7 = r5+12)
    cpu.fr[0] = fr0;
    r0 = 16;
    r1 = r1.wrapping_add(18);
    let _ = mem.write_f32(r14, fr0); // fmov fr0,@r14 (r14 = r4+12)
    r6 = mem.read_u8(r6).unwrap_or(0) as u32; // mov.b + extu.b (r6 = r5+16)
    let _ = mem.write_u8(r0.wrapping_add(r4), r6 as u8); // [r4+16]
    r3 = mem.read_u8(r3).unwrap_or(0) as u32; // r3 = r5+17
    r0 = 17;
    let _ = mem.write_u8(r0.wrapping_add(r4), r3 as u8); // [r4+17]
    r1 = mem.read_u8(r1).unwrap_or(0) as u32; // r1 = r5+18
    r0 = 18;
    let _ = mem.write_u8(r0.wrapping_add(r4), r1 as u8); // [r4+18]
    r1 = mem.read_u32(r5.wrapping_add(20)).unwrap_or(0);
    r0 = r4;
    let _ = mem.write_u32(r4.wrapping_add(20), r1);
    r1 = mem.read_u32(r5.wrapping_add(24)).unwrap_or(0);
    let _ = mem.write_u32(r4.wrapping_add(24), r1);
    r1 = mem.read_u32(r5.wrapping_add(28)).unwrap_or(0);
    let _ = mem.write_u32(r4.wrapping_add(28), r1); // rts delay slot

    cpu.r[14] = mem.read_u32(r15).unwrap_or(cpu.r[14]); // rts delay pop
    cpu.r[0] = r0;
    cpu.r[1] = r1;
    cpu.r[3] = r3;
    cpu.r[6] = r6;
    cpu.r[7] = r7;
    cpu.r[15] = r15_entry;
}
