//! 0x8c0273b0 — copy two 48-byte blocks into obj+0xe4 and obj+0xb4.
//!
//! Six unrolled 8-byte iterations copying words from r5→obj+0xe4 and
//! r6→obj+0xb4. Register side effects the C does not show: r4 ends as
//! `param1+0xb4` (not restored), r1=6, r3=48, r7=6, r0 keeps the last
//! sign-extended word load, and T ends clear because the final
//! `cmp/hi r1,r7` compares 6 > 6. r10–r14 are pushed/popped, so they
//! keep their entry values.

use sh4_oracle::{Cpu, Memory};

fn w(mem: &Memory, addr: u32) -> u32 {
    // mov.w loads sign-extend in the SH-4 (and in the oracle)
    mem.read_u16(addr).unwrap() as i16 as i32 as u32
}

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let p1 = cpu.r[4];
    let d1 = p1.wrapping_add(mem.read_u32(0x8c027410).unwrap()); // literal 0xe4
    let d2 = p1.wrapping_add(mem.read_u32(0x8c027414).unwrap()); // literal 0xb4
    cpu.r[4] = d2; // add r0,r4 — r4 is NOT saved/restored
    cpu.r[7] = 6; // mov #6,r7
    let mut i = 0u32; // r3
    for n in 1..=6u32 {
        let s1 = cpu.r[5].wrapping_add(i);
        let s2 = cpu.r[6].wrapping_add(i);
        let o1 = d1.wrapping_add(i);
        let o2 = d2.wrapping_add(i);
        cpu.r[0] = w(mem, s1);
        let _ = mem.write_u16(o1, cpu.r[0] as u16);
        cpu.r[0] = w(mem, s1 + 2);
        let _ = mem.write_u16(o1 + 2, cpu.r[0] as u16);
        cpu.r[1] = n; // add #1,r1
        cpu.r[0] = w(mem, s1 + 4);
        let _ = mem.write_u16(o1 + 4, cpu.r[0] as u16);
        i += 8; // add #8,r3
        cpu.r[0] = w(mem, s1 + 6);
        let _ = mem.write_u16(o1 + 6, cpu.r[0] as u16);
        cpu.r[0] = w(mem, s2);
        let _ = mem.write_u16(o2, cpu.r[0] as u16);
        cpu.r[0] = w(mem, s2 + 2);
        let _ = mem.write_u16(o2 + 2, cpu.r[0] as u16);
        cpu.r[0] = w(mem, s2 + 4);
        let _ = mem.write_u16(o2 + 4, cpu.r[0] as u16);
        cpu.r[0] = w(mem, s2 + 6);
        let _ = mem.write_u16(o2 + 6, cpu.r[0] as u16); // bt.s delay slot
        cpu.r[3] = i;
    }
    cpu.set_t(false); // final cmp/hi r1,r7: 6 > 6 is false
}
