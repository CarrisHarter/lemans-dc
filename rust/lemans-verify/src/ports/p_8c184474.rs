//! 0x8c184474 — four 24-byte block copies.
//!
//! After two pushes: copy r4+24..+47 to r5, r4+48..+71 to r6, r4+72..+95 to
//! r7, then r3 = *r15 (out pointer, @(8,r15) after the pushes) and copy
//! r4+96..+119 to r3. r4 ends as r4+96; r1 = *(r4+112), r0 = *(r4+116).
//! r13/r14 are saved and restored. No T-setting instruction runs.

use sh4_oracle::{Cpu, Memory};

fn put6(mem: &mut Memory, d: u32, s: u32) {
    for i in 0..6u32 {
        let v = mem.read_u32(s + i * 4).unwrap();
        let _ = mem.write_u32(d + i * 4, v);
    }
}

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let r4 = cpu.r[4];
    let r5 = cpu.r[5];
    let r6 = cpu.r[6];
    let r7 = cpu.r[7];
    let r13 = r4.wrapping_add(24);
    let r14 = r4.wrapping_add(48);
    let r3 = r4.wrapping_add(72);
    let r4n = r4.wrapping_add(96);
    cpu.r[4] = r4n;
    put6(mem, r5, r13);
    put6(mem, r6, r14);
    put6(mem, r7, r3);
    let out = mem.read_u32(cpu.r[15]).unwrap(); // mov.l @(8,r15),r3 after 2 pushes
    cpu.r[3] = out;
    put6(mem, out, r4n);
    cpu.r[1] = mem.read_u32(r4n + 16).unwrap(); // mov.l @(16,r4),r1
    cpu.r[0] = mem.read_u32(r4n + 20).unwrap(); // mov.l @(20,r4),r0
}
