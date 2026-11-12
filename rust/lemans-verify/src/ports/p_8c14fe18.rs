//! 0x8c14fe18 — byte-identical body to 0x8c14fdf4 (offset-24 variant; the
//! return value is loaded through `@(12,r0)` in the rts delay slot).
//!
//! Original:
//! ```text
//! mov.l @(4,r4),r1 ; add #24,r1 ; mov.l @(4,r1),r0
//! cmp/hi r5,r0 ; bf 0x8c14fe2e    <- plain bf: NO delay slot
//! mov.l @(8,r1),r1 ; shll2 r5 ; mov r5,r0 ; mov.l @(r0,r1),r0
//! rts ; mov.l @(12,r0),r0
//! 0x8c14fe2e: mov #0,r0 ; rts ; nop
//! ```

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let r4 = cpu.r[4];
    let x = mem.read_u32(r4 + 4).unwrap_or(0);
    let mut r1 = x.wrapping_add(24); // add #24,r1
    let r0 = mem.read_u32(r1 + 4).unwrap_or(0); // r0 = mem[X+28]
    let t = r0 > cpu.r[5]; // cmp/hi r5,r0
    if t {
        r1 = mem.read_u32(r1 + 8).unwrap_or(0); // r1 = mem[X+32]
        cpu.set_t(cpu.r[5] >> 30 & 1 != 0); // shll2 r5 sets T from bit 30
        let r5 = cpu.r[5] << 2; // shll2 r5
        cpu.r[5] = r5;
        let v = mem.read_u32(r5.wrapping_add(r1)).unwrap_or(0); // mov.l @(r0,r1),r0
        cpu.r[0] = mem.read_u32(v.wrapping_add(12)).unwrap_or(0); // rts delay slot
    } else {
        cpu.r[0] = 0;
    }
    cpu.r[1] = r1;
    if !t {
        cpu.set_t(t); // miss path: cmp/hi T survives (mov #0,r0 does not touch it)
    }
}
