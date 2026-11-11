//! 0x8c18c594 — copy a 28-byte record from r5 to r4.
//!
//! Original: three u32s, one f32, three bytes, then three more u32s (the last
//! store sits in the `rts` delay slot). r0 ends as r4, r1 as the source's
//! +24 word, r3 as the source's +4 word; fr0 holds the copied float; T is
//! untouched (no flag-setting instruction runs).

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let r4 = cpu.r[4];
    let r5 = cpu.r[5];
    let mut r0 = r5;
    let mut r1 = r4;
    let mut r3 = mem.read_u32(r5).unwrap_or(0);
    let _ = mem.write_u32(r4, r3); // mov.l r3,@r4
    r0 = r0.wrapping_add(8); // add #8,r0
    r3 = mem.read_u32(r5.wrapping_add(4)).unwrap_or(0);
    r1 = r1.wrapping_add(8); // add #8,r1
    let _ = mem.write_u32(r4.wrapping_add(4), r3); // mov.l r3,@(4,r4)
    let fr0 = mem.read_f32(r5.wrapping_add(8)).unwrap_or(0.0); // fmov @r0,fr0
    cpu.fr[0] = fr0;
    let _ = mem.write_f32(r1, fr0); // fmov fr0,@r1  (r1 = r4+8)
    r0 = mem.read_u8(r5.wrapping_add(12)).unwrap_or(0) as i8 as i32 as u32; // mov.b @(12,r5),r0
    let _ = mem.write_u8(r4.wrapping_add(12), r0 as u8); // mov.b r0,@(12,r4)
    r0 = mem.read_u8(r5.wrapping_add(13)).unwrap_or(0) as i8 as i32 as u32;
    let _ = mem.write_u8(r4.wrapping_add(13), r0 as u8);
    r0 = mem.read_u8(r5.wrapping_add(14)).unwrap_or(0) as i8 as i32 as u32;
    let _ = mem.write_u8(r4.wrapping_add(14), r0 as u8);
    r1 = mem.read_u32(r5.wrapping_add(16)).unwrap_or(0);
    r0 = r4; // mov r4,r0
    let _ = mem.write_u32(r4.wrapping_add(16), r1);
    r1 = mem.read_u32(r5.wrapping_add(20)).unwrap_or(0);
    let _ = mem.write_u32(r4.wrapping_add(20), r1);
    r1 = mem.read_u32(r5.wrapping_add(24)).unwrap_or(0);
    let _ = mem.write_u32(r4.wrapping_add(24), r1); // rts delay slot: mov.l r1,@(24,r4)
    cpu.r[0] = r0;
    cpu.r[1] = r1;
    cpu.r[3] = r3;
}
