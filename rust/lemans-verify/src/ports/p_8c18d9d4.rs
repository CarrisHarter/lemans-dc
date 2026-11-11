//! 0x8c18d9d4 — copy a 36-byte record from r5 to r4.
//!
//! Layout: u32s at +0/+4/+8/+12, an f32 at +16, bytes at +20/+21/+22, u32s at
//! +24/+28/+32. The float goes through fr0 (`fmov @r7,fr0` / `fmov fr0,@r14`
//! with r7 = r5+16, r14 = r4+16); each byte is `mov.b` sign-extended then
//! `extu.b` zero-extended, so the stored value is the raw byte. r13/r14 are
//! saved and restored (r14 in the `rts` delay slot). Exit: r0 = r4,
//! r1 = src+32 word, r3 = zero-extended src byte +21, r6 = zero-extended src
//! byte +20, r7 = r5+16; fr0 = the copied float; T untouched.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let r4 = cpu.r[4];
    let r5 = cpu.r[5];
    let r15_entry = cpu.r[15];
    let r15 = r15_entry.wrapping_sub(8); // push r14, r13
    let _ = mem.write_u32(r15.wrapping_add(4), cpu.r[14]);
    let _ = mem.write_u32(r15, cpu.r[13]);

    let mut r0 = mem.read_u32(r5).unwrap_or(0);
    let mut r7 = r5;
    let mut r14 = r4;
    let _ = mem.write_u32(r4, r0);
    let mut r6 = r5;
    r0 = mem.read_u32(r5.wrapping_add(4)).unwrap_or(0);
    let mut r3 = r5;
    let mut r1 = r5;
    let _ = mem.write_u32(r4.wrapping_add(4), r0);
    r7 = r7.wrapping_add(16);
    r0 = mem.read_u32(r5.wrapping_add(8)).unwrap_or(0);
    r14 = r14.wrapping_add(16);
    r6 = r6.wrapping_add(20);
    let _ = mem.write_u32(r4.wrapping_add(8), r0);
    r3 = r3.wrapping_add(21);
    let r13 = mem.read_u32(r5.wrapping_add(12)).unwrap_or(0);
    r0 = 20;
    r1 = r1.wrapping_add(22);
    let _ = mem.write_u32(r4.wrapping_add(12), r13);
    let fr0 = mem.read_f32(r7).unwrap_or(0.0); // fmov @r7,fr0 (r7 = r5+16)
    cpu.fr[0] = fr0;
    let _ = mem.write_f32(r14, fr0); // fmov fr0,@r14 (r14 = r4+16)
    r6 = mem.read_u8(r6).unwrap_or(0) as u32; // mov.b @r6,r6 + extu.b r6,r6
    let _ = mem.write_u8(r0.wrapping_add(r4), r6 as u8); // mov.b r6,@(r0,r4), r0=20
    r3 = mem.read_u8(r3).unwrap_or(0) as u32; // r3 = r5+21
    r0 = 21;
    let _ = mem.write_u8(r0.wrapping_add(r4), r3 as u8);
    r1 = mem.read_u8(r1).unwrap_or(0) as u32; // r1 = r5+22
    r0 = 22;
    let _ = mem.write_u8(r0.wrapping_add(r4), r1 as u8);
    r1 = mem.read_u32(r5.wrapping_add(24)).unwrap_or(0);
    r0 = r4;
    let _ = mem.write_u32(r4.wrapping_add(24), r1);
    r1 = mem.read_u32(r5.wrapping_add(28)).unwrap_or(0);
    let _ = mem.write_u32(r4.wrapping_add(28), r1);
    r1 = mem.read_u32(r5.wrapping_add(32)).unwrap_or(0);
    let _ = mem.write_u32(r4.wrapping_add(32), r1);

    // pops: r13 now, r14 in the rts delay slot
    cpu.r[13] = mem.read_u32(r15).unwrap_or(cpu.r[13]);
    cpu.r[14] = mem.read_u32(r15.wrapping_add(4)).unwrap_or(cpu.r[14]);
    cpu.r[0] = r0;
    cpu.r[1] = r1;
    cpu.r[3] = r3;
    cpu.r[6] = r6;
    cpu.r[7] = r7;
    cpu.r[15] = r15_entry;
}
