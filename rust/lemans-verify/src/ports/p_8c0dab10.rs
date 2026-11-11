//! 0x8c0dab10 — pick a slot from the table at 0x5e0+r4 and optionally copy
//! 64 bytes of it into the global buffer at 0x8c27a8c8; returns slot->+12.
//!
//! CFG:
//!   b = *0x8c27a8a8; if (b != 0) return 0x8c27a8c8;            // path A, T=0
//!   b = *0x8c27a8b0; if (b != 0) { 16-word copy }               // path C, T=0
//!   return [[0x5e0+r4]]+12;                                      // path B, T=1
//!
//! Copy store order (dest 0x8c27a8c8, src = entry+12): dest+4..=16 <-
//! src+4..=16; dest+20..=32 <- src+20..=32; dest+36..=48 <- src+36..=48;
//! dest+52..=64 <- src+52..=64.
//!
//! Hidden side effects on the copy path: r1 := src+64 value, r3 := src+60
//! value, r5 := src+52 value, r6 := inner+36, r7 := dest+36; T := 0.
//! On the skip path only r0 changes and T := 1 (last tst). r11..r14 are
//! pushed/popped; r15 net unchanged.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let flag_a = mem.read_u32(0x8C0DABAC).unwrap(); // 0x8c27a8a8
    let b = mem.read_u8(flag_a).unwrap();
    cpu.set_t(b == 0); // tst r0,r0 after extu.b
    if b != 0 {
        // path A: bra to the epilogue with r0 = the global buffer pointer
        cpu.r[0] = mem.read_u32(0x8C0DABB0).unwrap(); // 0x8c27a8c8
        return;
    }
    let flag_b = mem.read_u32(0x8C0DABB4).unwrap(); // 0x8c27a8b0
    let b2 = mem.read_u8(flag_b).unwrap();
    cpu.set_t(b2 == 0); // second tst
    let table = mem.read_u32(0x8C0DABB8).unwrap(); // 0x5e0
    let entry = mem.read_u32(table + cpu.r[4]).unwrap();
    if b2 != 0 {
        // copy path
        let inner = mem.read_u32(entry + 12).unwrap();
        let dest = mem.read_u32(0x8C0DABB0).unwrap(); // 0x8c27a8c8
        let copy = |mem: &mut Memory, d: u32, s: u32| {
            let v = mem.read_u32(s).unwrap();
            let _ = mem.write_u32(d, v);
        };
        copy(mem, dest + 4, inner + 4);
        copy(mem, dest + 8, inner + 8);
        copy(mem, dest + 12, inner + 12);
        copy(mem, dest + 16, inner + 16);
        copy(mem, dest + 20, inner + 20);
        copy(mem, dest + 24, inner + 24);
        copy(mem, dest + 28, inner + 28);
        copy(mem, dest + 32, inner + 32);
        copy(mem, dest + 36, inner + 36);
        copy(mem, dest + 40, inner + 40);
        copy(mem, dest + 44, inner + 44);
        copy(mem, dest + 48, inner + 48);
        copy(mem, dest + 52, inner + 52);
        copy(mem, dest + 56, inner + 56);
        copy(mem, dest + 60, inner + 60);
        copy(mem, dest + 64, inner + 64);
        cpu.r[5] = mem.read_u32(inner + 52).unwrap();
        cpu.r[3] = mem.read_u32(inner + 60).unwrap();
        cpu.r[1] = mem.read_u32(inner + 64).unwrap();
        cpu.r[6] = inner + 36;
        cpu.r[7] = dest + 36;
    }
    // shared tail at 0x8c0dab9a: r0 = [[0x5e0+r4]]+12
    cpu.r[0] = mem.read_u32(entry + 12).unwrap();
}
