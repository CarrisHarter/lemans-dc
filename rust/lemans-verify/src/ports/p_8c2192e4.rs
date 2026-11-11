//! 0x8c2192e4 — copy 24 records of 24 bytes from [ [0x8c2193d8] + 40 ] (stride
//! 44) to [0x8c2193d4] (stride 24), permuting the fields:
//!   dst[0..3]   = src[0..3]          (bytes)
//!   dst[4..7]   = src[8..11]         (bytes)
//!   dst[8..11]  = src[16..19]        (dword)
//!   dst[12..15] = src[20..23]        (dword)
//!
//! ```text
//! mov.l 0x8c2193d8,r3 ; mov #0,r6 ; mov.l 0x8c2193d4,r5 ; mov #24,r7
//! mov.l @r3,r4 ; add #40,r4
//! loop (0x8c2192f0):
//!   mov.b @r4,r2 ; add #1,r6 ; cmp/ge r7,r6 ; mov.b r2,@r5
//!   mov.b @(1,r4),r0 ; mov.b r0,@(1,r5) ; ... @(3,r4) -> @(3,r5)
//!   mov.l @(16,r4),r3 ; mov.l r3,@(8,r5)
//!   mov.b @(8,r4),r0 ; mov.b r0,@(4,r5) ; ... @(11,r4) -> @(7,r5)
//!   mov.l @(20,r4),r3 ; mov.l r3,@(12,r5)
//!   add #24,r5 ; bf.s 0x8c2192f0 ; add #44,r4 (delay)
//! rts ; nop
//! ```
//! NOTE: the original uses group-8 R0-displacement moves (0x8441/0x8051/...),
//! which the sh4-oracle decodes with an 8-bit displacement and r0 as base, so
//! the oracle faults on the first such store and this port cannot be
//! oracle-verified; the port follows the disassembly (4-bit displacement).

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let mut r3 = mem.read_u32(0x8C2193D8).unwrap(); // 0x8c2ba634
    cpu.r[6] = 0;
    cpu.r[5] = mem.read_u32(0x8C2193D4).unwrap(); // 0x8c2b88f8
    let mut r5 = cpu.r[5];
    cpu.r[7] = 24;
    let r7 = 24u32;
    let mut r4 = mem.read_u32(r3).unwrap().wrapping_add(40);
    cpu.r[4] = r4;
    loop {
        // 0x8c2192f0
        cpu.r[2] = mem.read_u8(r4).unwrap() as i8 as i32 as u32;
        cpu.r[6] = cpu.r[6].wrapping_add(1);
        cpu.set_t((cpu.r[6] as i32) >= (r7 as i32)); // cmp/ge r7,r6
        let _ = mem.write_u8(r5, cpu.r[2] as u8);
        for k in 1..=3u32 {
            cpu.r[0] = mem.read_u8(r4.wrapping_add(k)).unwrap() as i8 as i32 as u32;
            let _ = mem.write_u8(r5.wrapping_add(k), cpu.r[0] as u8);
        }
        r3 = mem.read_u32(r4.wrapping_add(16)).unwrap();
        cpu.r[3] = r3;
        let _ = mem.write_u32(r5.wrapping_add(8), r3);
        for k in 8..=11u32 {
            cpu.r[0] = mem.read_u8(r4.wrapping_add(k)).unwrap() as i8 as i32 as u32;
            let _ = mem.write_u8(r5.wrapping_add(k - 4), cpu.r[0] as u8);
        }
        r3 = mem.read_u32(r4.wrapping_add(20)).unwrap();
        cpu.r[3] = r3;
        let _ = mem.write_u32(r5.wrapping_add(12), r3);
        r5 = r5.wrapping_add(24);
        cpu.r[5] = r5;
        // bf.s 0x8c2192f0 ; delay add #44,r4 (runs every iteration)
        r4 = r4.wrapping_add(44);
        cpu.r[4] = r4;
        if cpu.t() {
            break; // r6 >= 24
        }
    }
    // rts ; nop
}
