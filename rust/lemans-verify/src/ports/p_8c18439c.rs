//! 0x8c18439c — fan-out struct copy.
//!
//! After six pushes, r3 = *r15 (out pointer). The 12-byte global at
//! 0x8c1d3f68 is copied to r3, r7, r3, r6, r3, r7, r5; then the 12-byte
//! blocks at r4+0xec, +0xf8, +0x104, +0x110 are copied to r5, r6, r7, r3.
//! r4 ends as r4+0x110, r1 = *(r4+0x110), r0 = *(r4+0x118), r3 = out pointer.
//! No T-setting instruction runs.

use sh4_oracle::{Cpu, Memory};

fn triple(mem: &Memory, a: u32) -> (u32, u32, u32) {
    (
        mem.read_u32(a).unwrap(),
        mem.read_u32(a + 4).unwrap(),
        mem.read_u32(a + 8).unwrap(),
    )
}

fn put(mem: &mut Memory, d: u32, s: (u32, u32, u32)) {
    let _ = mem.write_u32(d, s.0);
    let _ = mem.write_u32(d + 4, s.1);
    let _ = mem.write_u32(d + 8, s.2);
}

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let r4 = cpu.r[4];
    let r3 = mem.read_u32(cpu.r[15]).unwrap(); // mov.l @(24,r15),r3 after 6 pushes
    cpu.r[3] = r3;
    let g = mem.read_u32(0x8c184460).unwrap(); // pool: 0x8c1d3f68
    cpu.r[1] = g;
    let r0 = r4.wrapping_add(mem.read_u32(0x8c184464).unwrap()); // +0xec
    let r11 = r4.wrapping_add(mem.read_u32(0x8c184468).unwrap()); // +0xf8
    let r13 = r4.wrapping_add(mem.read_u32(0x8c18446c).unwrap()); // +0x104
    let r4n = r4.wrapping_add(mem.read_u32(0x8c184470).unwrap()); // +0x110
    cpu.r[4] = r4n;

    let gt = triple(mem, g);
    let s0 = triple(mem, r0);
    let s1 = triple(mem, r11);
    let s2 = triple(mem, r13);
    let s3 = triple(mem, r4n);

    let r5 = cpu.r[5];
    let r6 = cpu.r[6];
    let r7 = cpu.r[7];
    put(mem, r3, gt);
    put(mem, r7, gt);
    put(mem, r3, gt);
    put(mem, r6, gt);
    put(mem, r3, gt);
    put(mem, r7, gt);
    put(mem, r3, gt);
    put(mem, r5, gt);
    put(mem, r5, s0);
    put(mem, r6, s1);
    put(mem, r7, s2);
    put(mem, r3, s3);

    cpu.r[1] = s3.0; // mov.l @r4,r1 (r4 = r4+0x110)
    cpu.r[0] = s3.2; // mov.l @(8,r4),r0
}
