//! 0x8c027418 — copy a 48-byte block to obj+0x114 plus six dwords
//! from r6's pointer table to obj+0x144.
//!
//! Six iterations: 8 bytes of words from r5+i → obj+0x114+i, then
//! `[obj+0x144+off] = [r6+off]` for off = 0,4,…,20 (the u32 store is the
//! `bt.s` delay slot). Side effects the C hides: r4 ends as `param1+0x144`,
//! r0=20, r1=6, r3=48, r7=24, T ends clear (final `cmp/hi r1,r14` is 6>6).
//! r11–r14 are pushed/popped and keep their entry values.

use sh4_oracle::{Cpu, Memory};

fn w(mem: &Memory, addr: u32) -> u32 {
    mem.read_u16(addr).unwrap() as i16 as i32 as u32
}

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let p1 = cpu.r[4];
    let d1 = p1.wrapping_add(mem.read_u32(0x8c027464).unwrap()); // literal 0x114
    let d2 = p1.wrapping_add(mem.read_u32(0x8c027468).unwrap()); // literal 0x144
    cpu.r[4] = d2; // add r0,r4 — r4 is NOT saved/restored
    let mut i = 0u32; // r3
    let mut off = 0u32; // r7
    for n in 1..=6u32 {
        let s = cpu.r[5].wrapping_add(i);
        let o = d1.wrapping_add(i);
        cpu.r[0] = w(mem, s);
        let _ = mem.write_u16(o, cpu.r[0] as u16);
        cpu.r[1] = n; // add #1,r1
        cpu.r[0] = w(mem, s + 2);
        i += 8; // add #8,r3
        cpu.r[3] = i;
        let _ = mem.write_u16(o + 2, cpu.r[0] as u16);
        cpu.r[0] = w(mem, s + 4);
        let _ = mem.write_u16(o + 4, cpu.r[0] as u16);
        cpu.r[0] = w(mem, s + 6);
        let _ = mem.write_u16(o + 6, cpu.r[0] as u16);
        cpu.r[0] = off; // mov r7,r0
        let v = mem.read_u32(cpu.r[6].wrapping_add(off)).unwrap(); // mov.l @(r0,r6),r13
        off += 4; // add #4,r7
        cpu.r[7] = off;
        let _ = mem.write_u32(d2.wrapping_add(cpu.r[0]), v); // bt.s delay slot
    }
    cpu.set_t(false); // final cmp/hi r1,r14: 6 > 6 is false
}
